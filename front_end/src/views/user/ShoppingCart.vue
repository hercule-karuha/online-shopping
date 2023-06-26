<template>
    <main>
        <header>
            <div class="title">
                <span>购物车( 全部{{ 10 }} )</span>
            </div>
            <div class="info">
                <span>已选商品({{ 0 }})</span>
                <span class="price">{{ "20.00" }}</span>
                <button>结 算</button>
            </div>
        </header>
        <div class="content">
            <DataList :data-source="dataSource" :flex="false">
                <template #default="{ data }">
                    <ShoppingCartItem :data="data" />
                </template>
            </DataList>
        </div>
    </main>
</template>

<script setup>
import DataList from '@/components/DataList.vue'
import ShoppingCartItem from '@/components/ShoppingCartItem.vue'
const dataSource = {
    pageSize: 10,
    pageNo: 1,
    pageCount: 2,
    total: 20,
    list: [
        {
            storeId: 1,
            name: '小米旗舰店',
            products: [
                {
                    productId: 1,
                    cover: 'https://pic.imgdb.cn/item/64475b180d2dde5777857756.webp',
                    name: 'XXxxxxxxxxxxxxXXXXXXXXXXX',
                    num: 2,
                    price: 100,
                },
                {
                    productId: 2,
                    cover: 'https://pic.imgdb.cn/item/64475aff0d2dde5777855968.jpg',
                    name: 'XXxxxxxxxxxxxxX手机---sXXXXXXXXXXX',
                    num: 8,
                    price: 100,
                }
            ]
        },
        {
            storeId: 2,
            name: '小华旗舰店',
            products: [
                {
                    productId: 2,
                    cover: 'https://pic.imgdb.cn/item/64475aff0d2dde5777855968.jpg',
                    name: 'XXxxxxxxxxxxxxX手机---sXXXXXXXXXXX',
                    num: 8,
                    price: 100,
                }
            ]
        }
    ]
}
</script>

<style scoped lang="scss">
main{
    border-radius: 20px;
    background-color: white;
    header{
        display: flex;
        align-items: center;
        font-size: 22px;
        padding: 20px 20px;
        border-bottom: 1px solid #eee;
        justify-content: space-between;
        .info{
            display: flex;
            align-items: center;
            span{
                margin-right: 20px;
                font-size: 20px;
            }
            .price{
                color: pink;
                font-size: 28px;
                font-weight: 500;
            }
            button{
                outline: none;
                border: none;
                border-radius: 20px;
                padding: 0 14px;
                height: 40px;
                background-color: pink;
                color: white;
                font-size: 20px;
                cursor: pointer;
            }
        }
    }
}
</style>