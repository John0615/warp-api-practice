use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::models::user::{NewLgUser, LgUser};

use crate::util::constant::GqlResult;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // 插入新用户
    async fn new_user(
        &self,
        ctx: &Context<'_>,
        new_lg_user: NewLgUser,
    ) -> GqlResult<LgUser> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        crate::service::user_service::insert_user(my_pool, new_lg_user).await
    }
}