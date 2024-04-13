// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Nullable<Integer>,
        body -> Text,
    }
}
