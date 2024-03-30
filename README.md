### Problem Explanation: Missing `name` Field in Product Model

In the provided Prisma schema, we have defined two models: `Product` and `FakeModelToTest`. The `Product` model has fields `id`, `fake_model_id`, and `fake_model`, along with a nested `name` field of type `Language`. However, there seems to be an issue as the `name` field is missing from the generated code.

```prisma
model Product {
  id            String @id @default(cuid()) @map("_id")
  fake_model_id String

  fake_model FakeModelToTest @relation(fields: [fake_model_id], references: [id])

  name Language
}

type Language {
  ar String
  en String
}
```

### Error:

The generated code is missing the `name` field, which is essential as per the schema definition.

```rust
// Error
// error: name
// --> packages/db/src/prisma.rs:332:7461
product::select!(db_select_product { name });
```
