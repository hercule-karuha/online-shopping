<template>
    <main>
        <header>
            <div class="title">
                <span>购物车( 全部{{ dataSource.total?dataSource.total:'0' }} )</span>
            </div>
            <div class="info">
                <span>已选商品({{ 0 }})</span>
                <span class="price">{{ totalPrice.toFixed(2) }}</span>
                <button @click="purchase">结 算</button>
            </div>
        </header>
        <div class="content">
            <DataList :data-source="dataSource" :flex="false" @changePage="changePage" :loading="loading">
                <template #default="{ data }">
                    <ShoppingCartItem :data="data" @delete="delItem" />
                </template>
            </DataList>
        </div>
    </main>
</template>

<script setup>
import DataList from '@/components/DataList.vue'
import ShoppingCartItem from './ShoppingCartItem.vue'
import { usePurchaseListStore } from '@/stores/purchaseList'
import { configProviderProps } from 'element-plus'
import { useRouter } from 'vue-router'
import message from '@/utils/message'
import { ref, onMounted, watch } from 'vue'
import { getShoppingCart } from '@/api/user'
const router = useRouter()
const purchaseListStore = usePurchaseListStore()
const dataSource = ref({ list: [] })
const totalPrice = ref(0)
const loading = ref(false)
onMounted(async () => {
    const res = await getShoppingCart({
        pageSize: '8',
        pageNo: '1',
    })
    if (!res) return
    dataSource.value = res.data
})
watch( purchaseListStore.list, (val) => {
    totalPrice.value = val.reduce((total, item) => {
        return total + item.price * item.num
    }, 0)
}, {immediate: true})
const purchase = () => {
    if (purchaseListStore.list.length === 0) {
        message.warning('请至少选择一件商品')
        return
    }
    router.push('/purchase')
}


const delItem = (id) => {
    dataSource.value.list = dataSource.value.list.filter(item => item.productId !== id)
}

const changePage = async (pageNo) => {
    loading.value = true
    const res = await getShoppingCart({
        pageSize: '8',
        pageNo:  (Number.parseInt(pageNo)).toString(),
    })
    loading.value = false
    if (!res) return
    dataSource.value = res.data
}
</script>

<style scoped lang="scss">
main {
    border-radius: 20px;
    background-color: white;

    header {
        display: flex;
        align-items: center;
        font-size: 22px;
        padding: 20px 20px;
        border-bottom: 1px solid #eee;
        justify-content: space-between;

        .info {
            display: flex;
            align-items: center;

            span {
                margin-right: 20px;
                font-size: 20px;
            }

            .price {
                color: pink;
                font-size: 28px;
                font-weight: 500;
            }

            button {
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