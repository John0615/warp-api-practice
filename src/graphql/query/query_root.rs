use async_graphql::*;

struct Book {
  id: i32,
  title: String,
  author: String,
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
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn books(&self) -> Vec<Book> {
      vec![
          Book {
              id: 1,
              title: "踢啊四四四口大口大口大口".to_string(),
              author: "John".to_string(),
          },
          Book {
              id: 2,
              title: "上课送我我哦饿哦饿".to_string(),
              author: "Jobin".to_string(),
          },
          Book {
              id: 3,
              title: "i为i为i开".to_string(),
              author: "Edward".to_string(),
          },
      ]
  }

}