<template>
    <main>
        <header>
            <input type="text" v-model="keyword" placeholder="请输入关键字">
            <button @click="search">搜索</button>
        </header>
        <div class="content">
            <DataList :dataSource="dataSource" @changePage="changePage">
                <template #default="{ data }">
                    <ProductItem :data="data" />
                </template>
            </DataList>
        </div>
    </main>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import DataList from '@/components/DataList.vue'
import ProductItem from './ProductItem.vue'
import { searchProduct } from '../../api/search'
import message from '@/utils/message'
const route = useRoute()
const router = useRouter()
const keyword = ref('')
const dataSource = ref({
    total: 0,
    pageCount: 0,
    list: []
})
onMounted(async() => {
    if (!route.query.keyword) return
    keyword.value = route.query.keyword
    const res = await  searchProduct({
        keyword: keyword.value,
        pageNo: '1',
        pageSize: '15',
        storeId: '0'
    })
    if (!res) return
    console.log(res.data)
    dataSource.value = res.data
})

watch(() => route.query.keyword, async (val) => {
    keyword.value = val
    if (!val) return
    const res = await  searchProduct({
        keyword: keyword.value,
        pageNo: '1',
        pageSize: '15',
        storeId: '0'
    })
    if (!res) return
    dataSource.value = res.data
})
const search = async () => {
    if (!keyword.value || keyword.value.trim() == '') {
        message.warning('请输入关键字')
        return
    }
    router.replace({
        path: '/product/list',
        query: {
            keyword: keyword.value
        }
    })
}
const changePage = async (pageNo) => {
    const res = await searchProduct({
        keyword: keyword.value,
        pageNo: pageNo.toStirng(),
        pageSize: '15',
        storeId: '0'
    })
    if (!res) return
    dataSource.value = res.data
}
</script>

<style lang="scss" scoped>
main {
    width: var(--global-width);
    margin: 0 auto;

    header {
        margin-top: 40px;
        padding: 20px;
        border-radius: 20px;
        background-color: white;
        display: flex;
        justify-content: center;
        align-items: center;
        input{
            height: 38px;
            outline: none;
            border-radius: 10px;
            width: 400px;
            margin-right: 30px;
            border: 1px solid #8f8f8f;
            padding-left: 10px;
        }
        button{
            height: 38px;
            width: 100px;
            outline: none;
            border-radius: 10px;
            background-color: var(--main-color-purple);
            border: none;
            color: white;
            cursor: pointer;
            &:hover{
                background-color: var(--main-color-purple-hover);
            }
        }
    }
    .content{
        margin-top: 30px;
        background-color: white;
        padding: 20px;
        border-radius: 20px;

    }
}
</style>
