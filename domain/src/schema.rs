// @generated automatically by Diesel CLI.

diesel::table! {
    invoices (id) {
        id -> Int4,
        business_id -> Int4,
        order_id -> Int4,
        presell_id -> Uuid,
        bolt11 -> Varchar,
        payment_hash -> Nullable<Varchar>,
        payment_secret -> Nullable<Varchar>,
        #[max_length = 250]
        description -> Varchar,
        customer_name -> Varchar,
        customer_email -> Varchar,
        currency -> Varchar,
        sub_total -> Numeric,
        taxes -> Numeric,
        shipping -> Numeric,
        total_amount -> Numeric,
        amount_sat -> Int4,
        status -> Int4,
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
        tipo_asociado -> Varchar,
        lnaddress -> Varchar,
        amount_sat -> Int4,
        fee_sat -> Int4,
        status -> Int4,
        bolt11 -> Nullable<Varchar>,
        payment_hash -> Nullable<Varchar>,
        payment_secret -> Nullable<Varchar>,
        attempts -> Int4,
        reported -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invoices,
    payment_split,
);
