use async_graphql::{Error, ErrorExtensions};
use rbatis::core::db::db_adapter::DBExecResult;
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

pub async fn insert_user(my_pool: &Rbatis) -> GqlResult<DBExecResult> {
  let user_data = LgUser {
    id: Some(1),
    email1: Some("1.qq.com".to_string()),
    phone_number: Some("17722443746".to_string()),
    nick_name: Some("John_1".to_string()),
    pwd: Some("123456".to_string()),
    department: Some("8989".to_string()),
    position: Some("ioio".to_string()),
    create_datetime: Some("2021-05-10 12:00:00".to_string()),
    status: Some(1),
    head_img_letter: Some("ieowwoei".to_string()),
    head_img_name: Some("oepepe".to_string()),
    head_img_status: Some(1),
    biography: Some("com".to_string()),
    register_from: Some(1),
    signature: Some("com".to_string()),
    ip: Some("1.qq".to_string()),
    country_code: Some("9093839".to_string()),
  };
  
  let user = my_pool.save("", &user_data).await;
  match user {
    Ok(value) => {
      println!("has value {:?}", value);
      Ok(value)
    },
    Err(error) => Err(Error::new("find-user-error").extend_with(|_, e| e.set("details", "No records"))),
  }
}
