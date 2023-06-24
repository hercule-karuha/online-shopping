alter table products
    add cover_id varchar(200);
alter table products
    drop column if exists store_address;
alter table orders
    drop column if exists user_phone;
alter table orders
    drop column if exists user_address;