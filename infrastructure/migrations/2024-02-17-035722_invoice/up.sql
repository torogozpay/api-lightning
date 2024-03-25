-- Your SQL goes here
CREATE TABLE invoices (
 id serial primary key,
 business_id integer not null,
 presell_id integer not null,
 split_id integer not null,
 order_id integer not null,
 bolt11 varchar null,
 payment_hash varchar null,
 payment_secret varchar null,
 description varchar(250) not null,
 currency varchar not null,
 total_amount numeric(18,2) not null default 0,
 amount_msat integer not null default 0,
 status varchar not null,
 invoice_date timestamp with time zone not null,
 created_at timestamp with time zone not null default now(),
 updated_at timestamp with time zone null,
 distributed boolean not null default false
);

create unique index idx_cn_invoices on invoices (
 business_id,
 presell_id,
 order_id
);


/**/

CREATE TABLE payment_split
(
    id serial primary key,
    presell_id integer NOT NULL,
    lnaddress character varying NOT NULL,
    amount numeric(18,2) NOT NULL DEFAULT 0,
    amount_msat integer NOT NULL DEFAULT 0,
    status character varying NOT NULL,
    bolt11 character varying NULL,
    attempts integer NOT NULL DEFAULT 0,
    CONSTRAINT payment_split_pkey PRIMARY KEY (id)
);