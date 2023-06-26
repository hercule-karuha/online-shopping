-- This file should undo anything in `up.sql`
alter table orders
    drop column if exists product_name;