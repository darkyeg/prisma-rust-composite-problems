### Problem Explanation: Unique Constraints with Composite Type Fields

In this problem, we're dealing with unique constraints on composite type fields, specifically in the context of a user and their phone model. We have a `Product` model with fields `id` and `name`, where `name` is of type `Language`, which itself consists of two fields, `ar` (Arabic) and `en` (English). We want to enforce uniqueness on combinations of `ar` and `en` values.

```graphql
model Product {
  id   String   @id @default(cuid()) @map("_id")
  name Language

  @@unique([name.ar, name.en])
}

type Language {
  ar String
  en String
}
```

The generated code contains a function `name_name` which takes two parameters of type `crate::prisma::language::Data`, representing `name.ar` and `name.en` respectively. However, we need to modify this to generate a function `name_ar_name_en` that takes separate parameters for `ar` and `en`.

### Desired Solution

We need to generate a function `name_ar_name_en` that accepts `ar` and `en` as separate parameters instead of passing a single `Language` object.

```rust
pub fn name_ar_name_en<T: From<UniqueWhereParam>>(
    ar: String,
    en: String,
) -> T {
    UniqueWhereParam::NameArNameEn(ar, en).into()
}
```
