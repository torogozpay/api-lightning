-- Your SQL goes here
CREATE TABLE invoices (
 id serial primary key,
 business_id integer not null,
 order_id integer not null,
 presell_id uuid not null,
 bolt11 varchar not null,
 payment_hash varchar null,
 payment_secret varchar null,
 description varchar(250) not null,
 customer_name varchar not null,
 customer_email varchar not null,
 currency varchar not null,
 sub_total numeric(18,2) not null default 0,
 taxes numeric(18,2) not null default 0,
 shipping numeric(18,2) not null default 0,
 total_amount numeric(18,2) not null default 0,
 amount_sat integer not null default 0,
 status integer not null,
 invoice_date timestamp with time zone not null,
 created_at timestamp with time zone not null default now(),
 updated_at timestamp with time zone null,
 distributed boolean not null default false,
 apply_split boolean not null default false
);

create unique index idx_cn_invoices on invoices (
 business_id,
 order_id,
 presell_id
);


/**/

CREATE TABLE payment_split
(
    id serial primary key,
    invoice_id integer NOT NULL,
    tipo_asociado character varying NOT NULL,
    lnaddress character varying NOT NULL,
    amount_sat integer NOT NULL DEFAULT 0,
    fee_sat integer NOT NULL DEFAULT 0,
    status integer NOT NULL DEFAULT 0,
    bolt11 character varying NULL,
    payment_hash varchar null,
    payment_secret varchar null,
    attempts integer NOT NULL DEFAULT 0,
    reported boolean not null default false
);