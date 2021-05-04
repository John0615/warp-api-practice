use serde::{Serialize, Deserialize};

#[rbatis::crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub email1: String,
    pub phone_number: String,
    pub nick_name: String,
    pub pwd: String,
    pub department: String,
    pub position: String,
    pub create_datetime: String,
    pub status: i32,
    pub head_img_letter: String,
    pub head_img_name: String,
    pub head_img_status: i32,
    pub biography: String,
    pub register_from: String,
    pub signature: String,
    pub ip: String,
    pub contry_code: String,
}

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn email1(&self) -> &str {
        self.email1.as_str()
    }

    pub async fn phone_number(&self) -> &str {
        self.phone_number.as_str()
    }

    pub async fn nick_name(&self) -> &str {
        self.nick_name.as_str()
    }

    pub async fn pwd(&self) -> &str {
        self.pwd.as_str()
    }

    pub async fn department(&self) -> &str {
        self.department.as_str()
    }

    pub async fn position(&self) -> &str {
        self.position.as_str()
    }

    pub async fn create_datetime(&self) -> &str {
        self.create_datetime.as_str()
    }

    pub async fn status(&self) -> i32 {
        self.status
    }

    pub async fn head_img_letter(&self) -> &str {
        self.head_img_letter.as_str()
    }

    pub async fn head_img_name(&self) -> &str {
        self.head_img_name.as_str()
    }

    pub async fn head_img_status(&self) -> i32 {
        self.head_img_status
    }

    pub async fn biography(&self) -> &str {
        self.biography.as_str()
    }

    pub async fn register_from(&self) -> &str {
        self.register_from.as_str()
    }

    pub async fn signature(&self) -> &str {
        self.signature.as_str()
    }

    pub async fn ip(&self) -> &str {
        self.ip.as_str()
    }

    pub async fn contry_code(&self) -> &str {
        self.contry_code.as_str()
    }
}