use crate::product;

product::include!(db_product { fake_model });

/// Where is `name` field?
/// # Fields
///  * `id`
///  * `fake_model_id`
///  * `fake_model`
///
/// # Expected
/// * `id`
/// * `fake_model_id`
/// * `fake_model`
/// * `name`
type Product = db_product::Data;

// Error
// error: name
// --> packages/db/src/prisma.rs:332:7461
product::select!(db_select_product { name });
