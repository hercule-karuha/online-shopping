<template>
  <main>
    <header>我的店铺</header>
    <div class="content">
        <DataList :data-source="dataSource" :flex="false">
            <template #default="{data}">
                <div class="store-item">
                    <StoreItem  :data="data"></StoreItem>
                </div>
            </template>
        </DataList>
    </div>
  </main>
</template>

<script setup>
import { ref } from 'vue'
import DataList from '@/components/DataList.vue'
import { onBeforeMount, onMounted } from 'vue'
import StoreItem from './StoreItem.vue'
import { useUserInfoStore } from '@/stores/userInfo'
import { getStoreInfo } from '@/api/store'
import { useRouter } from 'vue-router'
const router = useRouter()
const userInfoStore = useUserInfoStore()
const dataSource = ref({
    pageNo: 1,
    pageSize: 1,
    pageCount: 1,
    total: 1,
    list: []
})
onMounted(async () => {
    if (!userInfoStore.userInfo) {
        router.push('/account/login')
    }
    if (!userInfoStore.userInfo.storeId) {
        router.push('/')
    }
    const result = await getStoreInfo(userInfoStore.userInfo.storeId)
    if (!result) {
        return
    }
    dataSource.value.list = [result.data]
})
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
    }
    .content{
        .store-item{
            margin: 20px;
        }
    }
}
</style>