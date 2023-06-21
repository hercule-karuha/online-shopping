<template>
    <main>
        <header>
            <div class="info">
                <div class="cover">
                    <img :src="storeInfo.cover" alt="">
                </div>
                <div class="name">{{ storeInfo.name }}</div>
                <div class="sales">
                    <span>销量</span>
                    <span>{{ storeInfo.sales }}</span>
                </div>
                <div class="edit">
                    <div>
                        <el-icon><EditPen /></el-icon>
                        <span>编辑店铺</span>
                    </div>
                    <div>
                        <el-icon><CirclePlus /></el-icon>
                        <span>商品上新</span>
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
            <el-tab-pane :label="tabNav.all" name="all">
                <DataList></DataList>
            </el-tab-pane>
            <el-tab-pane :label="tabNav.sales" name="sales">
                <DataList></DataList>
            </el-tab-pane>
            <el-tab-pane :label="tabNav.price" name="price">
                <DataList></DataList>
            </el-tab-pane>
            <el-tab-pane :label="tabNav.new" name="new">
                <DataList></DataList>
            </el-tab-pane>
            <el-tab-pane :label="tabNav.orders" name="orders">
                <DataList></DataList>
            </el-tab-pane>
        </el-tabs>
        </div>
    </main>
</template>

<script setup>
import { ref } from 'vue'
import DataList from '@/components/DataList.vue'
const storeInfo = {
    name: '小米旗舰店',
    sales: 1000,
    address: '福建省厦门市湖里区',
    cover: 'https://pic.imgdb.cn/item/64475aff0d2dde5777855968.jpg'
}
const activeName = ref('all')
const tabNav = ref({
    all: '全部',
    sales: '销量⬇️',
    price: '价格⬆️',
    new: '新品',
    orders: '订单'
})
const handleClick = (tab) => {
    console.log(tab.props.name)
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
            .sales{
                display: flex;
                flex-direction: column;
                margin-left: 40px;
                font-size: 18px;
                align-items: center;
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