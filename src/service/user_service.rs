use async_graphql::{Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use crate::models::user::{LgUser, NewLgUser};
use crate::util::constant::GqlResult;

pub async fn all_users(my_pool: &Rbatis) -> GqlResult<Vec<LgUser>> {
 
  let users: Vec<LgUser> = my_pool.fetch_list().await.unwrap();
  println!("{:?}", users);

  if users.len() > 0 {
      Ok(users)
  } else {
      Err(Error::new("1-all-users")
          .extend_with(|_, e| e.set("details", "No records")))
  }
}

pub async fn find_user(my_pool: &Rbatis, id: i32) -> GqlResult<LgUser> {
  let user: Option<LgUser> = my_pool.fetch_by_column("id", &id.to_string()).await.unwrap();
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
  let user = my_pool.fetch_by_wrapper::<LgUser>(&phone_number_wrapper).await;

  println!("user>>>>>> {:?}", user);

  if user.is_ok() {
      Ok(user.unwrap())
  } else {
      Err(Error::new("phone_number 不存在")
          .extend_with(|_, e| e.set("details", "1_EMAIL_NOT_EXIStS")))
  }
}

pub async fn insert_user(my_pool: &Rbatis, mut new_lg_user: NewLgUser) -> GqlResult<LgUser> {
  // email 转小写
  new_lg_user.email1 = new_lg_user.email1.to_lowercase();
  if self::get_user_phone_number(my_pool, &new_lg_user.phone_number).await.is_ok() {
    Err(Error::new("phone_number 已存在")
        .extend_with(|_, e| e.set("details", "1_EMAIL_EXIStS")))
  } else {
    let save_result = my_pool.save(&new_lg_user).await;
    match save_result {
      Ok(_value) => {
        println!("phone_number {:?}", &new_lg_user.phone_number);
        self::get_user_phone_number(my_pool, &new_lg_user.phone_number).await
      },
      Err(_error) => Err(Error::new("save-user-error").extend_with(|_, e| e.set("save", "error"))),
    }
  }
  
}
