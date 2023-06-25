alter table orders
    drop column if exists store_id;
alter table orders
    drop column if exists store_address;