use async_graphql::{Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use crate::models::user::{LgUser, NewLgUser};
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


// 通过 email 获取用户
pub async fn get_user_phone_number(
  my_pool: &Rbatis,
  phone_number: &str,
) -> GqlResult<LgUser> {
  let phone_number_wrapper = my_pool.new_wrapper().eq("phone_number", phone_number);
  let user = my_pool.fetch_by_wrapper::<LgUser>("", &phone_number_wrapper).await;

  if user.is_ok() {
      Ok(user.unwrap())
  } else {
      Err(Error::new("email 不存在")
          .extend_with(|_, e| e.set("details", "1_EMAIL_NOT_EXIStS")))
  }
}

pub async fn insert_user(my_pool: &Rbatis, new_lg_user: NewLgUser) -> GqlResult<LgUser> {
  let save_result = my_pool.save("", &new_lg_user).await;
  match save_result {
    Ok(value) => {
      println!("has value {:?}", value);
      self::get_user_phone_number(my_pool, &"17722443746".to_string()).await
    },
    Err(_error) => Err(Error::new("find-user-error").extend_with(|_, e| e.set("details", "No records"))),
  }
}
