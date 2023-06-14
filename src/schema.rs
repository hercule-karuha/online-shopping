// @generated automatically by Diesel CLI.

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        store_product_id -> Int4,
        purchase_time -> Timestamp,
        total_price -> Numeric,
        quantity -> Int4,
    }
}

diesel::table! {
    product_images (image_id, path) {
        image_id -> Int4,
        #[max_length = 200]
        path -> Varchar,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        image_id -> Nullable<Int4>,
        price -> Numeric,
        sales -> Nullable<Int4>,
        stock -> Nullable<Int4>,
    }
}

diesel::table! {
    shopping_cart (user_id, order_id) {
        user_id -> Int4,
        order_id -> Int4,
    }
}

diesel::table! {
    stores (store_id) {
        store_id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    stores_products_unit (store_product_id) {
        store_product_id -> Int4,
        store_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    user_stores_unit (user_stores_id) {
        user_stores_id -> Int4,
        user_id -> Int4,
        store_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 11]
        account -> Bpchar,
        #[max_length = 20]
        password -> Varchar,
        #[max_length = 2]
        gender -> Nullable<Bpchar>,
        #[max_length = 4]
        user_type -> Nullable<Bpchar>,
        #[max_length = 11]
        phone -> Nullable<Bpchar>,
        #[max_length = 100]
        address -> Nullable<Varchar>,
    }
}

diesel::joinable!(shopping_cart -> orders (order_id));
diesel::joinable!(shopping_cart -> users (user_id));
diesel::joinable!(stores_products_unit -> products (product_id));
diesel::joinable!(stores_products_unit -> stores (store_id));
diesel::joinable!(user_stores_unit -> stores (store_id));
diesel::joinable!(user_stores_unit -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    product_images,
    products,
    shopping_cart,
    stores,
    stores_products_unit,
    user_stores_unit,
    users,
);
