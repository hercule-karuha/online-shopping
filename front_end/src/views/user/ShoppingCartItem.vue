<template>
    <div class="content">
        <!-- <div class="store">
            <el-checkbox :label="'店铺: '+data.name" size="large" 
            @change="changeSelectAll"
            v-model="selected.store"></el-checkbox>
        </div> -->
        <!-- <div class="products">
            <div
            class="product-item" 
            v-for="(item, index) in data.products" :key="item.productId">
                <div class="left">
                    <el-checkbox size="large"
                    v-model="selected.products[index]"></el-checkbox>
                    <div class="cover">
                        <img :src="item.cover" alt="">
                    </div>
                    <div class="name">{{ item.name }}</div>
                </div>
                <div class="price">¥{{ item.price }}</div>
                <el-input-number v-model="item.num"></el-input-number>
                <div class="sum">¥{{ item.price*item.num }}</div>
                <div class="operator">
                    <span @click="delCart(item.id)">删除</span>
                </div>
            </div>
        </div> -->
        <div class="content">
            <div class="product-item">
                <div class="left">
                    <el-checkbox size="large" v-model="selected"></el-checkbox>
                    <div class="cover">
                        <img @click="goProduct(data.productId)" :src="proxy.globalInfo.productCoverUrl+data.productId" alt="">
                    </div>
                    <div @click="goProduct(data.productId)" class="name">{{ data.productName }}</div>
                </div>
                <div class="price">¥{{ data.price }}</div>
                <el-input-number @change="changeNum" :min="1" v-model="data.num"></el-input-number>
                <div class="sum">¥{{ (data.price * data.num).toFixed(2) }}</div>
                <div class="operator">
                    <span @click="delCart(data.productId)">删除</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, watch, getCurrentInstance } from 'vue'
import message from '@/utils/message'
import { useRouter } from 'vue-router'
import { usePurchaseListStore } from '@/stores/purchaseList'
import { editShoppingCart } from '@/api/purchase'
const router = useRouter()
const {proxy } = getCurrentInstance()
const purchaseListStore = usePurchaseListStore()
const props = defineProps({
    data: {
        type: Object,
        default: () => { }
    }
})
const emit = defineEmits(['delete', 'select'])
const selected = ref(false)
watch(selected, (val) => {
    if (val) {
        purchaseListStore.add(props.data)
    } else {
        purchaseListStore.remove(props.data)
    }
})
const delCart = async (id) => {
    message.confirm("确定要删除吗?", async () => {
        // 发送请求
        const res = await editShoppingCart(
            {list:[
                {
                productId: id,
                num: '0'
            }
            ]}
        )
        if (! res) return
        message.success("删除成功")
        emit('delete', id)
    })
}

const goProduct = (id) => {
    router.push({
        path: '/product/detail/' + id 
    })
}

const changeNum = async (num) => {
    console.log(num)
    // 发送请求
    const res = await editShoppingCart(
        {list:[
            {
            productId: props.data.productId,
            num: num.toString()
        }
        ]}
    )

} 
</script>

<style scoped lang="scss">
.content {
    margin: 20px;
    flex-grow: 1;

    .product-item {
        display: flex;
        width: 100%;
        padding: 24px 20px;
        background-color: #f3f3f3;
        border-radius: 20px;
        transition: all 0.3s;
        &:hover {
            background-color: #f8f5f8;
        }
        width: 100%;
        align-items: start;
        justify-content: space-between;

        .left {
            display: flex;
            align-items: start;

            .cover {
                margin-left: 10px;
                width: 90px;
                height: 90px;
                border-radius: 10px;
                overflow: hidden;
                cursor: pointer;

                img {
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
            }

            .name {
                margin-left: 20px;
                width: 180px;
                word-wrap: break-word;
                cursor: pointer;
                transition: all 0.1s;

                &:hover {
                    text-decoration: underline;
                    color: pink;
                }
            }

        }

        .price {
            font-weight: 600;
        }

        .sum {
            font-weight: 600;
            color: rgb(255, 144, 162);
        }

        .operator {
            span {
                cursor: pointer;

                &:hover {
                    text-decoration: underline;
                    color: pink;
                }
            }
        }
    }

}</style>