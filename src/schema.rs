// @generated automatically by Diesel CLI.

diesel::table! {
    exchanges_rates (id) {
        id -> Nullable<Integer>,
        base_currency -> Text,
        target_currency -> Text,
        rate -> Float,
        last_update -> Text,
    }
}
