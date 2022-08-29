-- Your SQL goes here
create table users (
    id serial primary key,
    uuid text not null,
    name text not null,
    email text not null,
    enc_key text not null,
    customer_id text not null
)
