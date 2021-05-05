use warp::{http,Filter, http::Response as HttpResponse, Rejection};

use graphql::query::query_root::QueryRoot;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{BadRequest, Response};
use dbs::mysql::my_pool;

use http::StatusCode;
use std::convert::Infallible;

use api::index::{Store, post_json, delete_json, update_grocery_list, delete_grocery_list_item, get_grocery_list};

#[tokio::main]
async fn main() {

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);
    
    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    // 获取 MySql 数据池后，可以将其增加到：
    // 1. 作为 async-graphql 的全局数据；
    // 2. 作为 warp 的应用程序数据，优势是可以进行原子操作；
    // 3. 使用 lazy-static.rs
    let my_pool = my_pool().await;

    // graphql
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(my_pool).finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    

    let routes = add_items
                .or(get_items)
                .or(delete_item)
                .or(update_item)
                .or(graphql_playground)
                .or(graphql_post)
                .recover(|err: Rejection| async move {
                    if let Some(BadRequest(err)) = err.find() {
                        return Ok::<_, Infallible>(warp::reply::with_status(
                            err.to_string(),
                            StatusCode::BAD_REQUEST,
                        ));
                    }
        
                    Ok(warp::reply::with_status(
                        "INTERNAL_SERVER_ERROR".to_string(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


mod graphql;
mod api;
mod dbs;
mod models;
mod service;
mod util;