// @generated automatically by Diesel CLI.

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        user_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        purchase_time -> Nullable<Timestamp>,
        total_price -> Nullable<Float8>,
        quantity -> Nullable<Int4>,
        #[max_length = 20]
        user_phone -> Nullable<Varchar>,
        #[max_length = 100]
        user_address -> Nullable<Varchar>,
        #[max_length = 100]
        store_address -> Nullable<Varchar>,
        store_id -> Nullable<Int4>,
        #[max_length = 100]
        product_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        store_id -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        price -> Nullable<Float8>,
        sales -> Nullable<Int4>,
        stock -> Nullable<Int4>,
        #[max_length = 200]
        detail_images -> Nullable<Varchar>,
        #[max_length = 100]
        store_address -> Nullable<Varchar>,
    }
}

diesel::table! {
    shopping_carts (user_id, product_id) {
        user_id -> Int4,
        product_id -> Int4,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    stores (store_id) {
        store_id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 20]
        user_name -> Nullable<Varchar>,
        #[max_length = 20]
        password -> Nullable<Varchar>,
        gender -> Nullable<Int4>,
        user_type -> Nullable<Int4>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 100]
        address -> Nullable<Varchar>,
    }
}

diesel::joinable!(orders -> products (product_id));
diesel::joinable!(orders -> stores (store_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(products -> stores (store_id));
diesel::joinable!(shopping_carts -> products (product_id));
diesel::joinable!(shopping_carts -> users (user_id));
diesel::joinable!(stores -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    products,
    shopping_carts,
    stores,
    users,
);
