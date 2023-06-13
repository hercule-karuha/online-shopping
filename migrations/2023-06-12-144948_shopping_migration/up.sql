--
-- PostgreSQL database dump
--

-- Dumped from database version 15.3
-- Dumped by pg_dump version 15.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: pg_database_owner
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO pg_database_owner;

--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: pg_database_owner
--

COMMENT ON SCHEMA public IS 'standard public schema';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: orders; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.orders (
    order_id character varying(20) NOT NULL,
    user_id character varying(20) NOT NULL,
    product_id character varying(20) NOT NULL,
    purchase_time timestamp without time zone DEFAULT now() NOT NULL,
    total_price numeric(10,2) NOT NULL,
    quantity integer NOT NULL,
    CONSTRAINT orders_quantity_check CHECK ((quantity > 0)),
    CONSTRAINT orders_total_price_check CHECK ((total_price >= (0)::numeric))
);


ALTER TABLE public.orders OWNER TO postgres;

--
-- Name: TABLE orders; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.orders IS '订单表';


--
-- Name: COLUMN orders.order_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.order_id IS '订单id';


--
-- Name: COLUMN orders.user_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.user_id IS '买家即用户id';


--
-- Name: COLUMN orders.product_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.product_id IS '商品';


--
-- Name: COLUMN orders.purchase_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.purchase_time IS '购买时间，默认当前时间';


--
-- Name: COLUMN orders.total_price; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.total_price IS '应付金额';


--
-- Name: COLUMN orders.quantity; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.orders.quantity IS '购买数量';


--
-- Name: product_images; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.product_images (
    image_id character varying(20) NOT NULL,
    path character varying(200) NOT NULL
);


ALTER TABLE public.product_images OWNER TO postgres;

--
-- Name: TABLE product_images; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.product_images IS '商品图片表，包含各个商品的图片信息';


--
-- Name: COLUMN product_images.image_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.product_images.image_id IS '商品图片ID';


--
-- Name: COLUMN product_images.path; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.product_images.path IS '商品图片相对路径';


--
-- Name: products; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.products (
    product_id character varying(20) NOT NULL,
    name character varying(100) NOT NULL,
    description text,
    image_id character varying(20),
    price numeric(10,2) NOT NULL,
    sales integer DEFAULT 0,
    stock integer DEFAULT 0,
    CONSTRAINT products_price_check CHECK ((price >= (0)::numeric)),
    CONSTRAINT products_sales_check CHECK ((sales >= 0)),
    CONSTRAINT products_stock_check CHECK ((stock >= 0))
);


ALTER TABLE public.products OWNER TO postgres;

--
-- Name: TABLE products; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.products IS '商品表，包含各个商品的详细信息';


--
-- Name: COLUMN products.product_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.product_id IS '商品 ID，外键关联stores_products_unit表';


--
-- Name: COLUMN products.name; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.name IS '商品名称';


--
-- Name: COLUMN products.description; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.description IS '商品详细描述';


--
-- Name: COLUMN products.image_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.image_id IS '商品图片 ID, 具体图片在 product_images 表中';


--
-- Name: COLUMN products.price; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.price IS '单价， DECIMAL(10,2)，表示为数字类型的价格';


--
-- Name: COLUMN products.sales; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.sales IS '销量';


--
-- Name: COLUMN products.stock; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.products.stock IS '库存量';


--
-- Name: shopping_cart; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.shopping_cart (
    user_id character varying(20) NOT NULL,
    order_id character varying(20) NOT NULL
);


ALTER TABLE public.shopping_cart OWNER TO postgres;

--
-- Name: stores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.stores (
    store_id character varying(20) NOT NULL,
    name character varying(100) NOT NULL
);


ALTER TABLE public.stores OWNER TO postgres;

--
-- Name: TABLE stores; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.stores IS '商铺表，用于存储商店信息';


--
-- Name: COLUMN stores.store_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.stores.store_id IS '商铺id';


--
-- Name: COLUMN stores.name; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.stores.name IS '商铺名称';


--
-- Name: stores_products_unit; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.stores_products_unit (
    store_id character varying(20) NOT NULL,
    product_id character varying(20) NOT NULL
);


ALTER TABLE public.stores_products_unit OWNER TO postgres;

--
-- Name: TABLE stores_products_unit; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.stores_products_unit IS '商铺_商品联合表，用于表示商店中有哪些商品';


--
-- Name: COLUMN stores_products_unit.store_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.stores_products_unit.store_id IS '商铺id';


--
-- Name: COLUMN stores_products_unit.product_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.stores_products_unit.product_id IS '商铺里面有的商品id';


--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    user_id character varying(20) NOT NULL,
    account character(11) NOT NULL,
    password character varying(20) NOT NULL,
    gender character(2),
    user_type character(4),
    phone character(11),
    address character varying(100),
    CONSTRAINT users_account_check CHECK ((account ~ '^[1-9]\d{10}$'::text)),
    CONSTRAINT users_gender_check CHECK (((gender = '男'::bpchar) OR (gender = '女'::bpchar))),
    CONSTRAINT users_password_check CHECK (((password)::text ~ '^(?=.*\d)(?=.*[A-z])[\da-zA-Z]{8,20}$'::text)),
    CONSTRAINT users_phone_check CHECK ((phone ~ '^[1-9]\d{10}$'::text)),
    CONSTRAINT users_user_type_check CHECK (((user_type = '买家'::bpchar) OR (user_type = '卖家'::bpchar)))
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: TABLE users; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.users IS '用户表,用于存储用户信息';


--
-- Name: COLUMN users.user_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.user_id IS '用户ID';


--
-- Name: COLUMN users.account; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.account IS '用户账号,用于登录 11位纯数字,首位不为0';


--
-- Name: COLUMN users.password; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.password IS '用户密码:8-20位字母和数字';


--
-- Name: COLUMN users.gender; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.gender IS '用户性别: 男/女';


--
-- Name: COLUMN users.user_type; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.user_type IS '用户类型 买家/卖家';


--
-- Name: COLUMN users.phone; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.phone IS '用户电话:11位纯数字';


--
-- Name: COLUMN users.address; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.address IS '用户/商家地址';


--
-- Data for Name: orders; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.orders (order_id, user_id, product_id, purchase_time, total_price, quantity) FROM stdin;
\.


--
-- Data for Name: product_images; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.product_images (image_id, path) FROM stdin;
\.


--
-- Data for Name: products; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.products (product_id, name, description, image_id, price, sales, stock) FROM stdin;
\.


--
-- Data for Name: shopping_cart; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.shopping_cart (user_id, order_id) FROM stdin;
\.


--
-- Data for Name: stores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.stores (store_id, name) FROM stdin;
\.


--
-- Data for Name: stores_products_unit; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.stores_products_unit (store_id, product_id) FROM stdin;
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (user_id, account, password, gender, user_type, phone, address) FROM stdin;
\.


--
-- Name: orders orders_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_pkey PRIMARY KEY (order_id);


--
-- Name: product_images product_images_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_images
    ADD CONSTRAINT product_images_pk PRIMARY KEY (image_id, path);


--
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (product_id);


--
-- Name: shopping_cart shopping_cart_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shopping_cart
    ADD CONSTRAINT shopping_cart_pkey PRIMARY KEY (user_id, order_id);


--
-- Name: stores stores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.stores
    ADD CONSTRAINT stores_pkey PRIMARY KEY (store_id);


--
-- Name: stores_products_unit stores_products_unit_product_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.stores_products_unit
    ADD CONSTRAINT stores_products_unit_product_id_key UNIQUE (product_id);


--
-- Name: users users_account_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_account_key UNIQUE (account);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (user_id);


--
-- Name: idx_orders_user_id_product_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_orders_user_id_product_id ON public.orders USING btree (user_id, product_id);


--
-- Name: idx_product_images_image_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_product_images_image_id ON public.product_images USING btree (image_id);


--
-- Name: idx_products_price; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_products_price ON public.products USING btree (price);


--
-- Name: idx_products_sales; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_products_sales ON public.products USING btree (sales);


--
-- Name: idx_products_stock; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_products_stock ON public.products USING btree (stock);


--
-- Name: idx_users_account_password; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_users_account_password ON public.users USING btree (account, password);


--
-- Name: products fk_products_product_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT fk_products_product_id FOREIGN KEY (product_id) REFERENCES public.stores_products_unit(product_id);


--
-- Name: stores_products_unit fk_stores_products_unit_store_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.stores_products_unit
    ADD CONSTRAINT fk_stores_products_unit_store_id FOREIGN KEY (store_id) REFERENCES public.stores(store_id);


--
-- Name: orders orders_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(product_id);


--
-- Name: orders orders_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- Name: shopping_cart shopping_cart_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shopping_cart
    ADD CONSTRAINT shopping_cart_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(order_id);


--
-- Name: shopping_cart shopping_cart_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shopping_cart
    ADD CONSTRAINT shopping_cart_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- PostgreSQL database dump complete
--

