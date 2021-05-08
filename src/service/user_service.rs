use async_graphql::{Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use crate::models::user::LgUser;
use crate::util::constant::GqlResult;

pub async fn all_users(my_pool: &Rbatis) -> GqlResult<Vec<LgUser>> {
 
  let users: Vec<LgUser> = my_pool.fetch_list("").await.unwrap();
  println!("{:?}", users);

  if users.len() > 0 {
      Ok(users)
  } else {
      Err(Error::new("1-all-users")
          .extend_with(|_, e| e.set("details", "No records")))
  }
}

pub async fn find_user(my_pool: &Rbatis, id: i32) -> GqlResult<LgUser> {
  let user: Option<LgUser> = my_pool.fetch_by_id("", &id).await.unwrap();
  match user {
    Some(value) => {
      println!("has value {:?}", value);
      Ok(value)
    },
    None => Err(Error::new("find-user-error").extend_with(|_, e| e.set("details", "No records"))),
  }
} 
