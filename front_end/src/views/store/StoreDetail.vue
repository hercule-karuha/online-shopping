<template>
    <main>
        <header>
            <div class="info">
                <div class="cover">
                    <img :src="storeInfo.storeId?proxy.globalInfo.storeCoverUrl+storeInfo.storeId:''" alt="">
                </div>
                <div class="name">{{ storeInfo.name }}</div>
 
                <div class="edit" v-if="editable">
                    <div @click="router.push('/store/edit')">
                        <el-icon><EditPen /></el-icon>
                        <span >编辑店铺</span>
                    </div>
                    <div @click="router.push('/store/newProduct')">
                        <el-icon><CirclePlus /></el-icon>
                        <span >商品上新</span>
                    </div>
                </div>
            </div>
            <div class="search">
                    <input type="text" v-model="keyword" placeholder="搜索 商品">
                    <div>
                        <button @click="search('all')">搜全部</button>
                        <button @click="search('inner')">搜本店</button>
                    </div>
            </div>
        </header>
        <div class="content">
            <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
            <el-tab-pane label="全部" name="all">
                <DataList :dataSource="dataSource" :loading="loading" v-if="activeName == 'all'" @changePage="changePage">
                    <template #default="{data}">
                        <ProductItem :data=data :show-edit="editable" @delProduct="delProduct"></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="销量⬆️" name="saleup">
                <DataList :dataSource="dataSource" :loading="loading" v-if="activeName == 'saleup'" @changePage="changePage">
                    <template #default="{data}">
                        <ProductItem :data=data :show-edit="editable" @delProduct="delProduct"></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="销量⬇️" name="saledown">
                <DataList :dataSource="dataSource" :loading="loading" v-if="activeName == 'saledown'" @changePage="changePage" >
                    <template #default="{data}">
                        <ProductItem :data=data :show-edit="editable" @delProduct="delProduct"></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="价格⬆️" name="priceup">
                <DataList :dataSource="dataSource" :loading="loading" v-if="activeName == 'priceup'" @changePage="changePage">
                    <template #default="{data}">
                        <ProductItem :data=data :show-edit="editable" @delProduct="delProduct"></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="价格⬇️" name="pricedown">
                <DataList :dataSource="dataSource" :loading="loading" v-if="activeName == 'pricedown'" @changePage="changePage">
                    <template #default="{data}">
                        <ProductItem :data=data :show-edit="editable" @delProduct="delProduct"></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            
            <el-tab-pane v-if="userInfoStore.userInfo && userInfoStore.userInfo.userId == storeInfo.userId" label="订单"  name="orders">
                <div class="search-order">
                    <input type="text" v-model="orderSearchKeyword" placeholder="请输入商品名或订单号">
                    <button @click="searchOrders">搜索销售记录</button>
                </div>
                <DataList :dataSource="dataSource" v-if="activeName == 'orders'" :flex="false" @changePage="changePage" :loading="loading">
                    <template #default="{ data }">
                    <div class="order-item">
                        <div class="top">
                            <div class="time">
                                {{ data.time }}
                            </div>
                            <div class="order-id">
                                订单号：{{ data.orderId }}
                            </div>
                        </div>
                        <div class="center">
                            <PurchaseListItem :data="data" :tips="true" :bgc="false"></PurchaseListItem>
                        </div>
                        <div class="buttom">
                            <div class="send">发货地址:{{ data.sendAddress }}</div>
                            <div class="receive">收货地址{{ data.receiveAdress }}</div>
                        </div>
                    </div>
                </template>
                </DataList>
            </el-tab-pane>
        </el-tabs>
        </div>
    </main>
</template>

<script setup>
import { ref, getCurrentInstance, onMounted } from 'vue'
import DataList from '@/components/DataList.vue'
import { getStoreInfo, getStoreProductList, getOrders } from '@/api/store'
import { useRoute, useRouter } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo.js'
import ProductItem from '../product/ProductItem.vue'
import PurchaseListItem from '@/views/purchase/PurchaseListItem.vue'
import { deleteProduct } from '@/api/product'
import message from '@/utils/message'
import { searchProduct , searchSales} from '@/api/search'
const userInfoStore = useUserInfoStore()
const { proxy } = getCurrentInstance()
const route = useRoute()
const router = useRouter()
const keyword = ref('')
const storeInfo = ref({
})
const insearch = ref(false)
const editable = ref(false)
const loading = ref(false)
const orderSearchKeyword = ref('')
const activeName = ref('all')
let query = {}
const dataSource = ref({ list:[]})

const parseOrderFormat = () => {
    dataSource.value.list.forEach(item => {
        item.sendAddress = JSON.parse(item.sendAddress)
        item.sendAddress = item.sendAddress.areaArr.join('  ') + '  ' + item.sendAddress.detailAddress
        item.receiveAdress = item.receiveAddress + '  ' + item.phone
    })
}
// orderType 0 时间顺序 1 时间倒序  2 销量顺序 3 销量降序 4 价格升序 5价格降序
onMounted(async () => {
    const result = await getStoreInfo(route.params.id)
    if (!result) return
    storeInfo.value = result.data
    if (userInfoStore.userInfo && userInfoStore.userInfo.userId == storeInfo.value.userId) {
        editable.value = true
    }
    const requst = {
        storeId: route.params.id,
        pageSize: '15',
        pageNo: '1',
        orderType: '0'
    }
    const listResult = await getStoreProductList(requst)
    if (!listResult) return
    dataSource.value = listResult.data

})

const handleClick = async (tab) => {
    console.log(tab.props)
    query = {
        pageSize: '15',
        pageNo: '1',
        storeId: route.params.id
    }
    // 点击相同的tab
    if (activeName.value == tab.props.name && !insearch.value) {
        retuen
    } else {
        activeName.value = tab.props.name
        if (activeName.value == 'all') {
            query.orderType = '0'
        } else if (activeName.value == 'saleup') {
            query.orderType = '2'
        } else if (activeName.value == 'saledown') {
            query.orderType = '3'
        } else if (activeName.value == 'priceup') {
            query.orderType = '4'
        } else if (activeName.value == 'pricedown') {
            query.orderType = '5'
        } else if (activeName.value == 'orders') {
            query = {
                pageSize: '15',
                pageNo: '1',
                storeId: route.params.id
            }
            loading.value = true
            const res = await getOrders(query)
            loading.value = false
            if (!res) return
            dataSource.value = res.data
            parseOrderFormat()
            return
        }
    }
    loading.value = true
    getStoreProductList(query).then(res => {
        if (!res) return
        dataSource.value = res.data
        loading.value = false
    })
}

const changePage = async (pageNo) => {
    query.pageNo = pageNo.toString()
    if (activeName.value == 'orders') {
        const res = await getOrders(query)
        if (!res) return
        dataSource.value = res.data
        parseOrderFormat()
        return
    } else {
        const res = await getStoreProductList(query)
        if (!res) return
        dataSource.value = res.data
    }
}
const delProduct = async (id) => {
    const res = await deleteProduct({
        productId: id
    })
    if (!res) return
    dataSource.value.list = dataSource.value.list.filter(item => item.productId != id)
    message.success('删除成功')
}

const search = (param) => {
    if (!keyword.value || keyword.value.trim() =='') {
        message.warning('请输入搜索关键字')
        return
    }
    if (param == 'all') {
        router.push({
            path: '/product/list',
            query: {
                keyword: keyword.value
            }
        })
    } else {
        query = {
            pageSize: '15',
            pageNo: '1',
            storeId: route.params.id,
            keyword: keyword.value
        }
        searchProduct(query).then(res => {
            if (!res) return
            dataSource.value = res.data
            insearch.value = true
        })
    }
}

const searchOrders = async() => {
    if (!orderSearchKeyword.value || orderSearchKeyword.value.trim() =='') {
        message.warning('请输入搜索关键字')
        return
    }
    query = {
        pageSize: '15',
        pageNo: '1',
        storeId: route.params.id,
        keyword: orderSearchKeyword.value
    }
    const res = await searchSales(query)
    if (!res) return
    dataSource.value = res.data
    parseOrderFormat()
    insearch.value = true
}
</script>

<style scoped lang="scss">
main{
    
    width: var(--global-width);
    
    margin: 0 auto;
    header{
        margin-top: 40px;
        display: flex;
        justify-content: space-between;
        padding-top: 14px;
        padding-bottom: 14px;
        align-items: center;
        background-color: white;
        border-radius: 20px;
        .info{
            display: flex;
            align-items: center;
            .cover{
                margin-left: 20px;
                height: 70px;
                width: 70px;
                border-radius: 10px;
                overflow: hidden;
                img{
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
            }
            .name{
                font-size: 30px;
                margin-left: 40px;
            }
            .edit{
                display: flex;
                div {
                    margin-left: 30px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    cursor: pointer;
                    font-size: 18px;
                    &:hover{
                        color: var(--main-color-purple);
                    }
                }
            }
        }
        .search{
            margin-right: 20px;
            display: flex;
            width: 460px;
            height: 38px;
            input{
                flex: 1;
                outline: none;
                border-radius: 10px;
                border: 1px solid #bebebe;
                padding-left: 10px;
            }
            button{
                outline: none;
                border: none;
                border-radius: 10px;
                height: 100%;
                margin-left: 10px;
                background-color: var(--main-color-purple);
                color: white;
                padding: 0 10px;
                cursor: pointer;
                &:hover{
                    background-color: var(--main-color-purple-hover);
                }
            }
        }
    }
    .content{
        margin-top: 40px;
        border-radius: 20px;
        background-color: white;
        padding: 10px 20px;
        .order-item {
            border-radius: 20px;
            overflow: hidden;
            margin: 20px;
            background-color: rgb(252, 251, 255);
            &:hover {
                background-color: rgb(250, 245, 255);
            }
            .top {
                display: flex;
                padding-left: 30px;
                height: 40px;
                display: flex;
                align-items: center;
            
                .order-id {
                    margin-left: 20px;
                }
            }
            transition: all 0.3s;
    
            .buttom{
                padding-left: 30px;
                padding-bottom: 10px;
            }
        }
    }
}
.search-order{
    margin-left: 20px;
    input{
        height: 30px;
        outline: none;
        border: 1px solid #bebebe;
        border-radius: 10px;
        width: 280px;
        padding-left: 10px;
    }
    button {
        height: 30px;
        outline: none;
        padding: auto 10px;
        border: none;
        border-radius: 10px;
        margin-left: 10px;
        color: white;
        background-color: var(--main-color-purple);
        cursor: pointer;
        &:hover{
            background-color: var(--main-color-purple-hover);
        }
    }
}
</style>