create table users
(
    user_id   serial
        primary key,
    account   char(11)    not null
        unique
        constraint users_account_check
            check (account ~ '^[1-9]\d{10}$'::text),
    password  varchar(20) not null
        constraint users_password_check
            check ((password)::text ~ '^(?=.*\d)(?=.*[A-z])[\da-zA-Z]{8,20}$'::text),
    gender    char(2)
        constraint users_gender_check
            check ((gender = '男'::bpchar) OR (gender = '女'::bpchar)),
    user_type char(4)
        constraint users_user_type_check
            check ((user_type = '买家'::bpchar) OR (user_type = '卖家'::bpchar)),
    phone     char(11)
        constraint users_phone_check
            check (phone ~ '^[1-9]\d{10}$'::text),
    address   varchar(100)
);

comment on table users is '用户表,用于存储用户信息';

comment on column users.user_id is '用户ID';

comment on column users.account is '用户账号,用于登录 11位纯数字,首位不为0';

comment on column users.password is '用户密码:8-20位字母和数字';

comment on column users.gender is '用户性别: 男/女';

comment on column users.user_type is '用户类型 买家/卖家';

comment on column users.phone is '用户电话:11位纯数字';

comment on column users.address is '用户/商家地址';


create index idx_users_account_password
    on users (account, password);

create table stores
(
    store_id serial
        primary key,
    name     varchar(100) not null
);

comment on table stores is '商铺表，用于存储商店信息';

comment on column stores.store_id is '商铺id';

comment on column stores.name is '商铺名称';


create table product_images
(
    image_id integer      not null,
    path     varchar(200) not null,
    constraint product_images_pk
        primary key (image_id, path)
);

comment on table product_images is '商品图片表，包含各个商品的图片信息';

comment on column product_images.image_id is '商品图片ID';

comment on column product_images.path is '商品图片相对路径';

create index idx_product_images_image_id
    on product_images (image_id);

create table products
(
    product_id  serial
        primary key,
    name        varchar(100)   not null,
    description text,
    image_id    integer,
    price       numeric(10, 2) not null
        constraint products_price_check
            check (price >= (0)::numeric),
    sales       integer default 0
        constraint products_sales_check
            check (sales >= 0),
    stock       integer default 0
        constraint products_stock_check
            check (stock >= 0)
);

comment on table products is '商品表，包含各个商品的详细信息';

comment on column products.product_id is '商品 ID，外键关联stores_products_unit表';

comment on column products.name is '商品名称';

comment on column products.description is '商品详细描述';

comment on column products.image_id is '商品图片 ID, 具体图片在 product_images 表中';

comment on column products.price is '单价， DECIMAL(10,2)，表示为数字类型的价格';

comment on column products.sales is '销量';

comment on column products.stock is '库存量';

create index idx_products_price
    on products (price);

create index idx_products_sales
    on products (sales);

create index idx_products_stock
    on products (stock);

create table orders
(
    order_id      serial
        primary key,
    store_product_id INTEGER not null,
    purchase_time timestamp default now() not null,
    total_price   numeric(10, 2)          not null
        constraint orders_total_price_check
            check (total_price >= (0)::numeric),
    quantity      integer                 not null
        constraint orders_quantity_check
            check (quantity > 0)
);

comment on table orders is '订单表';

comment on column orders.order_id is '订单id';

comment on column orders.store_product_id is '商店中商品id';

comment on column orders.purchase_time is '购买时间，默认当前时间';

comment on column orders.total_price is '应付金额';

comment on column orders.quantity is '购买数量';

create index idx_orders_store_product_id
    on orders (store_product_id);

create table shopping_cart
(
    user_id  integer not null
        references users,
    order_id integer not null
        references orders,
    primary key (user_id, order_id)
);

create table stores_products_unit
(
    store_product_id serial primary key,
    store_id   integer not null
        constraint fk_stores_products_unit_store_id
            references stores,
    product_id integer not null
        unique
        constraint fk_stores_products_unit_product_id
            references products
);

comment on table stores_products_unit is '商铺_商品联合表，用于表示商店中有哪些商品';

comment on column stores_products_unit.store_id is '商铺id';

comment on column stores_products_unit.product_id is '商铺里面有的商品id';

create table user_stores_unit (
    user_stores_id serial not null
        primary key,
    user_id INTEGER NOT NULL
        REFERENCES users,
    store_id INTEGER NOT NULL
        REFERENCES stores
);

CREATE  INDEX idx_user_stores_unit ON user_stores_unit(user_id,store_id);

comment on table user_stores_unit is '用户商铺联合表';

comment on  column user_stores_unit.user_stores_id is '用户商铺ID';

comment on column user_stores_unit.user_id is '用户id';

comment on column user_stores_unit.store_id is '商铺ID';




