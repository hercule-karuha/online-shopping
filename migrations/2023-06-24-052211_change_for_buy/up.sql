alter table products
    drop column if exists cover_id;
alter table products
    add store_address varchar(1000);
alter table orders
    add user_phone varchar(20);
alter table orders
    add user_address varchar(1000);