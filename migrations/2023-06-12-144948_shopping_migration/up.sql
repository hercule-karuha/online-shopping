create table users
(
    user_id   serial
        primary key,
    user_name   varchar(20) unique,
    password  varchar(20),
    gender    INTEGER,
    user_type INTEGER,
    phone     varchar(20),
    address   varchar(100)
);

comment on table users is '用户表,用于存储用户信息';

comment on column users.user_id is '用户ID';

comment on column users.user_name is '用户账号,用于登录';

comment on column users.password is '用户密码';

comment on column users.gender is '用户性别: ';

comment on column users.user_type is '用户类型 ';

comment on column users.phone is '用户电话';

comment on column users.address is '用户/商家地址';

create table stores
(
    store_id serial
        primary key,
    user_id INTEGER
        references users,
    name     varchar(100),
    address varchar(100)
);

comment on table stores is '商铺表，用于存储商店信息';

comment on column stores.store_id is '商铺id';

comment on column stores.name is '商铺名称';

create index idx_stores_user_id
    on stores(user_id);


create table products
(
    product_id  serial
        primary key,
    store_id INTEGER
        references stores,
    name        varchar(100),
    description text,
    cover_id varchar(200),
    price       float,
    sales       integer,
    stock       integer,
    detail_images varchar(200)
);

comment on table products is '商品表，包含各个商品的详细信息';
comment on column products.product_id is '商品 ID，外键关联stores_products_unit表';
comment on column products.store_id is '所属商店id';
comment on column products.name is '商品名称';
comment on column products.description is '商品详细描述';
comment on column products.price is '单价， DECIMAL(10,2)，表示为数字类型的价格';
comment on column products.sales is '销量';
comment on column products.stock is '库存量';

create table orders
(
    order_id      serial
        primary key,
    user_id INTEGER
        references users,
    product_id INTEGER
        references products,
    purchase_time timestamp ,
    total_price   float,
    quantity      integer
);

comment on table orders is '订单表';

comment on column orders.order_id is '订单id';

comment on column orders.product_id is '商店中商品id';

comment on column orders.purchase_time is '购买时间，默认当前时间';

comment on column orders.total_price is '应付金额';

comment on column orders.quantity is '购买数量';

create EXTENSION if not exists btree_gist ;

create index id_orders_store_product_id
    on orders using gist(user_id,product_id);

create table shopping_carts(
    user_id INTEGER
        references users,
    product_id INTEGER
        references products,
    quantity INTEGER,
    primary key (user_id,product_id)
);




