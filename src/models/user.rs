use serde::{Serialize, Deserialize};
// use chrono::NaiveDateTime;

#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
pub struct LgUser {
    pub id: Option<i32>,
    pub email1: Option<String>,
    pub phone_number: Option<String>,
    pub nick_name: Option<String>,
    pub pwd: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    pub create_datetime: Option<String>,
    pub status: Option<i32>,
    pub head_img_letter: Option<String>,
    pub head_img_name: Option<String>,
    pub head_img_status: Option<i32>,
    pub biography: Option<String>,
    pub register_from: Option<i32>,
    pub signature: Option<String>,
    pub ip: Option<String>,
    pub country_code: Option<String>,
}

#[async_graphql::Object]
impl LgUser {
    pub async fn id(&self) -> Option<i32> {
        self.id
    }

    pub async fn email1(&self) -> Option<String> {
        self.email1.clone()
    }

    pub async fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub async fn nick_name(&self) -> Option<String> {
        self.nick_name.clone()
    }

    pub async fn pwd(&self) -> Option<String> {
        self.pwd.clone()
    }

    pub async fn department(&self) -> Option<String> {
        self.department.clone()
    }

    pub async fn position(&self) -> Option<String> {
        self.position.clone()
    }

    pub async fn create_datetime(&self) -> Option<String> {
        self.create_datetime.clone()
    }

    pub async fn status(&self) -> Option<i32> {
        self.status
    }

    pub async fn head_img_letter(&self) -> Option<String> {
        self.head_img_letter.clone()
    }

    pub async fn head_img_name(&self) -> Option<String> {
        self.head_img_name.clone()
    }

    pub async fn head_img_status(&self) -> Option<i32> {
        self.head_img_status
    }

    pub async fn biography(&self) -> Option<String> {
        self.biography.clone()
    }

    pub async fn register_from(&self) -> Option<i32> {
        self.register_from
    }

    pub async fn signature(&self) -> Option<String> {
        self.signature.clone()
    }

    pub async fn ip(&self) -> Option<String> {
        self.ip.clone()
    }

    pub async fn country_code(&self) -> Option<String> {
        self.country_code.clone()
    }
}