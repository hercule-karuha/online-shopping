<template>
    <main>
        <header>
            <div class="info">
                <div class="cover">
                    <img :src="proxy.globalInfo.storeCoverUrl+storeInfo.storeId" alt="">
                </div>
                <div class="name">{{ storeInfo.name }}</div>
 
                <div class="edit" v-if="userInfoStore.userInfo&&userInfoStore.userInfo.userId == storeInfo.userId">
                    <div>
                        <el-icon><EditPen /></el-icon>
                        <span @click="router.push('/store/edit')">编辑店铺</span>
                    </div>
                    <div>
                        <el-icon><CirclePlus /></el-icon>
                        <span @click="router.push('/store/newProduct')">商品上新</span>
                    </div>
                </div>
            </div>
            <div class="search">
                <!-- <div class="search-box"> -->
                    <input type="text" placeholder="搜索 商品/店铺">
                    <div>
                        <button>搜全部</button>
                        <button>搜本店</button>
                    </div>
                <!-- </div> -->
            </div>
        </header>
        <div class="content">
            <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
            <el-tab-pane label="全部" name="all">
                <DataList :dataSource="dataSource" v-if="activeName == 'all'">
                    <template #default="{data}">
                        <ProductItem :data=data></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="销量⬆️" name="saleup">
                <DataList :dataSource="dataSource" v-if="activeName == 'saleup'">
                    <template #default="{data}">
                        <ProductItem :data=data></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="销量⬇️" name="saledown">
                <DataList :dataSource="dataSource" v-if="activeName == 'saledown'" >
                    <template #default="{data}">
                        <ProductItem :data=data></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="价格⬆️" name="priceup">
                <DataList :dataSource="dataSource" v-if="activeName == 'priceup'">
                    <template #default="{data}">
                        <ProductItem :data=data></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            <el-tab-pane label="价格⬇️" name="pricedown">
                <DataList :dataSource="dataSource" v-if="activeName == 'pricedown'" >
                    <template #default="{data}">
                        <ProductItem :data=data></ProductItem>
                    </template>
                </DataList>
            </el-tab-pane>
            
            <el-tab-pane v-if="userInfoStore.userInfo && userInfoStore.userInfo.userId == storeInfo.userId" label="订单" name="orders">
                <DataList></DataList>
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
const userInfoStore = useUserInfoStore()
const { proxy } = getCurrentInstance()
const route = useRoute()
const router = useRouter()
const storeInfo = ref({
})

const activeName = ref('all')
let query = {}
const dataSource = ref({ list:[]})

// orderType 0 时间顺序 1 时间倒序  2 销量顺序 3 销量降序 4 价格升序 5价格降序
onMounted(async () => {
    const result = await getStoreInfo(route.params.id)
    if (!result) return
    storeInfo.value = result.data
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
    if (activeName.value == tab.props.name) {
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
            const res = await getOrders(query)
            if (!res) return
            dataSource.value = res.data
            return
        }
    }
    getStoreProductList(query).then(res => {
        if (!res) return
        dataSource.value = res.data
    })
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
    }
}
</style>