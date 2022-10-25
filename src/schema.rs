// @generated automatically by Diesel CLI.

diesel::table! {
    scans (id) {
        id -> Int4,
        ip -> Text,
        region -> Text,
    }
}
