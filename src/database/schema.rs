table! {
    imports (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        raw_image_id -> Nullable<Bpchar>,
        sidecar_id -> Nullable<Bpchar>,
        image_id -> Nullable<Bpchar>,
        user_id -> Int4,
    }
}
