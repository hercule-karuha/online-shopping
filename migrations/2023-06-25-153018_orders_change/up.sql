-- Your SQL goes here
alter table orders
    add store_address varchar(100);
alter table orders
    add store_id integer;
alter table orders
    add constraint orders_store_id__fk
        foreign key (store_id) references stores(store_id);
