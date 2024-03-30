use db::LazyClient;

#[allow(non_upper_case_globals)]
static client: LazyClient = LazyClient::new();

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  client
    .init(&std::env::var("DATABASE_URL").expect("DATABASE_URL not found"))
    .await;

  // The problem
  // .................................

  println!("Hello, world!");
}
