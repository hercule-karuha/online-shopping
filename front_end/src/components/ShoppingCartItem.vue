<template>
    <div class="content">
        <div class="store">
            <el-checkbox :label="'店铺: '+data.name" size="large" 
            @change="changeSelectAll"
            v-model="selected.store"></el-checkbox>
        </div>
        <div class="products">
            <div
            class="product-item" 
            v-for="(item, index) in data.products" :key="item.productId">
                <div class="left">
                    <el-checkbox text-color="#CDC6EB" size="large"
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
        </div>
    </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import message from '@/utils/message'
import { useRouter } from 'vue-router'
const router = useRouter()
const props = defineProps({
    data: {
        type: Object,
        default: () => { }
    }
})
const selected = ref({
    store: false,
    products: new Array(props.data.products.length).fill(false)
})

const changeSelectAll = (state) => {
    if (state) {
        selected.value.products = selected.value.products.map(() => true)
    } else {
        selected.value.products = selected.value.products.map(() => false)
    }
}
watch(() => selected.value.products, (val) => {
    if (val.every(item => item)) {
        selected.value.store = true
    } else {
        selected.value.store = false
    }
}, { deep: true })
const delCart = (id) => {
    message.confirm("确定要删除吗?")
    console.log(id)
}

const goProdut = (id) => {

}
</script>

<style scoped lang="scss">
.content{
    margin: 20px;
    flex-grow: 1;
    .store{
        margin-bottom: 6px;
    }
    .products{
        width: 100%;
        background-color: #f3f3f3;
        border-radius: 20px;
        transition: all 0.3s;
        &:hover{
            background-color: #f8f5f8;
        }
        .product-item{
            display: flex;
            padding: 24px 20px;
            
            
            width: 100%;
            align-items: start;
            justify-content: space-between;
            .left{
                display: flex;
                align-items: start;
                .cover{
                    margin-left: 10px;
                    width: 90px;
                    height: 90px;
                    border-radius: 10px;
                    overflow: hidden;
                    cursor: pointer;
                    img{
                        width: 100%;
                        height: 100%;
                        object-fit: cover;
                    }
                }
                .name{
                    margin-left: 20px;
                    width: 180px;
                    word-wrap: break-word;
                    cursor: pointer;
                    transition: all 0.1s;
                    &:hover{
                        text-decoration: underline;
                        color: pink;
                    }
                }
                
            }
            .price{
                font-weight: 600;
            }
            .sum{
                font-weight: 600;
                color: rgb(255, 144, 162);
            }
            .operator{
                span{
                    cursor: pointer;
                    &:hover{
                        text-decoration: underline;
                        color: pink;
                    }
                }
            }
        }
    }
}
</style>