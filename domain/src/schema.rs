// @generated automatically by Diesel CLI.

diesel::table! {
    invoices (id) {
        id -> Int4,
        business_id -> Int4,
        presell_id -> Int4,
        split_id -> Int4,
        order_id -> Int4,
        bolt11 -> Nullable<Varchar>,
        payment_hash -> Nullable<Varchar>,
        payment_secret -> Nullable<Varchar>,
        #[max_length = 250]
        description -> Varchar,
        currency -> Varchar,
        total_amount -> Numeric,
        amount_msat -> Int4,
        status -> Varchar,
        invoice_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        distributed -> Bool,
        apply_split -> Bool,
    }
}

diesel::table! {
    payment_split (id) {
        id -> Int4,
        invoice_id -> Int4,
        lnaddress -> Varchar,
        amount -> Numeric,
        amount_msat -> Int4,
        status -> Varchar,
        bolt11 -> Nullable<Varchar>,
        attempts -> Int4,
    }
}

diesel::joinable!(payment_split -> invoices (invoice_id));

diesel::allow_tables_to_appear_in_same_query!(
    invoices,
    payment_split,
);
