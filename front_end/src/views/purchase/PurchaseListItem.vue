<template>
    <div :class="['list-item', bgc?'bgc':'']">
        <div class="left">
            <div class="cover it">
                <img :src="proxy.globalInfo.productCoverUrl+data.productId" alt="" @click="goProduct">
            </div>
            <div class="name" @click="goProduct">
                {{ data.productName }}
            </div>
        </div>
        <div class="price it"><span v-if="tips">单价： </span>{{ Number.parseFloat(data.price).toFixed(2) }}</div>
        <div class="num it"><span v-if="tips">数量： </span>{{ data.num }}</div>
        <div class="sum it"><span v-if="tips">总价： </span> {{ (data.num * data.price).toFixed(2) }}</div>
    </div>
</template>

<script setup>
import { getCurrentInstance } from 'vue'
import { useRouter } from 'vue-router'
const { proxy } = getCurrentInstance()
const router = useRouter()
const props = defineProps({
    data: {
        type: Object,
        default: () => { }
    },
    tips: {
        type: Boolean,
        default: false
    },
    bgc: {
        type: Boolean,
        default: true
    }
})
const goProduct = () => {
    router.push('/product/detail/' + props.data.productId)
}
</script>

<style lang="scss" scoped>
.left {
    display: flex;
    align-items: start;
}

.label {
    font-size: 18px;
    margin-bottom: 20px;

    span {
        padding-bottom: 10px;
        cursor: pointer;
        position: relative;

        &::after {
            position: absolute;
            content: '';
            width: 0;
            height: 2px;
            background-color: var(--main-color-purple);
            bottom: 0px;
            left: 0;
        }

        &:hover {
            &::after {
                width: 100%;
                transition: width 0.3s;
            }
        }
    }
}

.bgc{
    background-color: #f7f8ff;
    &:hover {
        background-color: #f8f5f8;
    }
}
.list-item {
    display: flex;
    width: 100%;
    padding: 24px 20px;
    
    border-radius: 20px;
    transition: all 0.3s;

    

    width: 100%;
    align-items: start;
    justify-content: space-between;

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

    .sum {
        font-weight: 600;
        font-size: 20px;
        color: rgb(255, 116, 116);
    }
}
</style>