use async_graphql::*;
use rbatis::rbatis::Rbatis;
use crate::models::user::LgUser;
// use rbatis::core::db::db_adapter::DBExecResult;

struct Book {
  id: i32,
  title: String,
  author: String,
  describe: String,
}

#[Object]
impl Book {
  async fn id(&self) -> i32 {
      self.id
  }
  async fn title(&self) -> &str {
      &self.title
  }

  async fn author(&self) -> &str {
      &self.author
  }

  async fn describe(&self) -> &str {
      &self.describe
  }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn books(&self) -> Vec<Book> {
    // println!("just logging: {}", &self.id);
      vec![
          Book {
              id: 1,
              title: "踢啊四四四口大口大口大口".to_string(),
              author: "John".to_string(),
              describe: "这是一本好书".to_string()
          },
          Book {
              id: 2,
              title: "上课送我我哦饿哦饿".to_string(),
              author: "Jobin".to_string(),
              describe: "这是一本好书".to_string()
          },
          Book {
              id: 3,
              title: "i为i为i开".to_string(),
              author: "Edward".to_string(),
              describe: "这是一本好书".to_string()
          },
      ]
  }

  async fn all_users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<LgUser>, async_graphql::Error> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        crate::service::user_service::all_users(my_pool).await
    }

    async fn find_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> std::result::Result<LgUser, async_graphql::Error> {
        println!("id>>>>>>>>>>: {}", id);
        let my_pool = ctx.data_unchecked::<Rbatis>();
        crate::service::user_service::find_user(my_pool, id).await
    }


}