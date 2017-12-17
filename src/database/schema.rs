table! {
    imports (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
        camera -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        raw_image_id -> Nullable<Int4>,
    }
}
