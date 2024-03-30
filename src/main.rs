use db::{product, LazyClient};

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

  // 1 - Error
  // Unable to match input value to any allowed input type for the field. Parse errors:
  // [Query parsing/validation error at `Query.findFirstProduct.where.ProductWhereInput.name`: Value types mismatch. Have: Boolean(true), want: Object(LanguageNullableCompositeFilter), Query parsing/validation error at `Query.findFirstProduct.where.ProductWhereInput.name`: Value types mismatch. Have: Boolean(true), want: Object(LanguageObjectEqualityInput), Query parsing/validation error at `Query.findFirstProduct.where.ProductWhereInput.name`: Value types mismatch. Have: Boolean(true), want: Null]"),
  // "query_position": String("Query.findFirstProduct.where.ProductWhereInput.name
  let result = client
    .product()
    .find_first(vec![product::name::is_set()])
    .exec()
    .await;

  println!("{:#?}", result);

  // 2 - Missing (equal/is/is_not)(None) in generated code
  // it is so important when use nullable fields
}
