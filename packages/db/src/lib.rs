#[allow(warnings, unused)]
mod prisma;

use std::ops::Deref;

use once_cell::sync::OnceCell;
pub use prisma::*;

pub struct LazyClient {
  client: OnceCell<PrismaClient>,
}

impl LazyClient {
  #[inline]
  pub const fn new() -> Self {
    Self {
      client: OnceCell::new(),
    }
  }

  #[inline]
  pub async fn init(&self, url: &str) {
    let client = new_client_with_url(url)
      .await
      .expect("Failed to connect to database");

    self
      .client
      .set(client)
      .expect("Database connection already initialized");
  }
}

impl Deref for LazyClient {
  type Target = PrismaClient;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self
      .client
      .get()
      .expect("Database connection not initialized")
  }
}
