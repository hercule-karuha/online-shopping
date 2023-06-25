alter table orders
    drop column store_address;
alter table orders
    drop constraint orders_store_id__fk;
alter table orders
    drop column store_id;