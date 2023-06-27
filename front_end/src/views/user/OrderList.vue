<template>
    <main>
        <header>
            <div class="title">购买记录</div>
            <div class="search">
                <input type="text" v-model="keyword" placeholder="请输入商品名称或订单号">
                <button @click="search">搜索</button>
            </div>
        </header>
        <div class="content">
            <DataList :data-source="dataSource" :flex="false">
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
        </div>
    </main>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import DataList from '@/components/DataList.vue'
import PurchaseListItem from '@/views/purchase/PurchaseListItem.vue'
import message from '@/utils/message.js'
import { getOrderList } from '@/api/user.js'
import { parseAddressCodeArr } from '@/utils/tools.js'
const keyword = ref('')
const dataSource = ref({ list: [] })
onMounted(async () => {
    const res = await getOrderList({
        pageSize: '10',
        pageNo: '1',
    })
    if (!res) return
    dataSource.value = res.data
    dataSource.value.list.forEach((item) => {
        item.sendAddress = JSON.parse(item.sendAddress)
        item.sendAddress = item.sendAddress.areaArr.join('  ') 
        item.receiveAdress = item.receiveAddress + '  ' + item.phone
    })
})

const search = async () => {
    if (keyword.value.trim() === '') {
        message.warning('关键词不能为空')
    }
}
</script>

<style lang="scss" scoped>
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

        .search {
            display: flex;
            align-items: center;

            input {
                outline: none;
                border-radius: 8px;
                border: 1px solid #826999;
                padding-left: 5px;
                height: 34px;
                width: 280px;
            }

            button {
                background-color: var(--main-color-purple);
                border-radius: 8px;
                margin-left: 10px;
                height: 34px;
                outline: none;
                border: none;
                cursor: pointer;
                padding: 0 20px;
                color: white;
                font-size: 18px;

                &:hover {
                    background-color: var(--main-color-purple-hover);
                }
            }
        }
    }

    .content {
        margin-top: 20px;

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
            .center{
                // background-color: var(--main-color-purple-hover);
            }
            .buttom{
                padding-left: 30px;
                padding-bottom: 10px;
            }
        }
    }
}</style>