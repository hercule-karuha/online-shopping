<template>
    <main>
        <div class="content">
            <div class="header">{{ newStoreFlag?'开店':'修改店铺信息' }}</div>
            <el-form
            :model="formData"
            ref="formDataRef"
            :rules="rules"
            class="form"
            label-position="top"
            status-icon>
                <el-form-item
                label="请上传你的商店封面" prop="cover">
                    <CoverUpload @upload-image="uploadCover"
                    :imageUrl="newStoreFlag?'':coverUrl" :update="!newStoreFlag"></CoverUpload>
                </el-form-item>
                <el-form-item
                label="请输入你的商店名称" prop="name">
                    <el-input  v-model="formData.name"
                    type="text" clearable></el-input>
                </el-form-item>
                <el-form-item 
                label="请选择你的发货区县"
                prop="selectedRegion">
                    <el-cascader :options="options"
                   v-model="formData.selectedRegion"></el-cascader>
                </el-form-item>
                <el-form-item
                prop="detailAddress"
                label="请输入详细地址">
                    <el-input v-model="formData.detailAddress"
                    type="text"
                    clearable></el-input>
                </el-form-item>
            </el-form>
            
            <button @click="submit">确认</button>
        </div>
    </main>
</template>

<script setup>
import { ref, onMounted, getCurrentInstance } from 'vue'
import CoverUpload from '@/components/CoverUpload.vue'
import { regionData } from 'element-china-area-data'
import { getStoreInfo,newStore, editStore } from '@/api/store'
import message from '@/utils/message'
import { parseAddressCodeArr } from '@/utils/tools'
import { getUserInfo } from '@/api/user'
import { useRoute, useRouter } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo'

const coverUrl = ref('')


const newStoreFlag = ref(true)
const { proxy } = getCurrentInstance()
const options = regionData
const router = useRouter()
const route = useRoute()
const userInfoStore = useUserInfoStore()
const formData = ref({
    selectedRegion: ['35', '3501', '350121']
})
const formDataRef = ref(null)
const uploadCover = (file) => {
    formData.value.cover = file
}
onMounted(async () => {
    if (route.path === '/store/edit') {
        newStoreFlag.value = false
    }
    if  (userInfoStore.userInfo === {} || !userInfoStore.userInfo) {
        const updateUserInfo = await getUserInfo()
        userInfoStore.userInfo = updateUserInfo.data
        
    }
    if (route.path === '/store/edit') {
        if (userInfoStore.userInfo.storeId == 0) {
            router.push('/')
        }
        newStoreFlag.value = false
        const result = await getStoreInfo(userInfoStore.userInfo.storeId)
        if (!result) {
            // message.error('获取商店信息失败')
            return
        } else {
            const storeInfo = result.data
            coverUrl.value = proxy.globalInfo.storeCoverUrl+userInfoStore.userInfo.storeId
            storeInfo.address = JSON.parse(storeInfo.address)
            formData.value.name = storeInfo.name
            formData.value.storeId = storeInfo.storeId
            formData.value.detailAddress = storeInfo.address.detailAddress
            formData.value.selectedRegion = storeInfo.address.codeArr
        }
    }
})

const submit = () => {
    formDataRef.value.validate(async (valid) => {
        if (valid) {
            // 获取区县码对应的区县名
            const addressArea = parseAddressCodeArr(formData.value.selectedRegion)
            let address = {
                codeArr : formData.value.selectedRegion,
                areaArr : addressArea,
                detailAddress : formData.value.detailAddress
            }
            address = JSON.stringify(address)
            let reqData = new FormData()
                     
            reqData.append('storeName', formData.value.name)
            reqData.append('cover', formData.value.cover)
            reqData.append('address', address)
            if (newStoreFlag.value) {
                const res = await newStore(reqData)
                if (!res) {
                    message.error('开店失败')
                } else {
                    message.success('开店成功')
                    const updateUserInfo = await getUserInfo()
                    userInfoStore.userInfo = updateUserInfo.data
                    router.replace('/store/detail/'+userInfoStore.userInfo.storeId)
                    
                }
            } else {
                if (reqData.get('cover') == 'undefined') {
                    reqData.delete('cover')
                }
                reqData.append('storeId', formData.value.storeId)
                const res = await editStore(reqData)
                if (!res) {
                    message.error('修改店铺信息失败')
                } else {
                    message.success('修改店铺信息成功')
                    router.push('/store/detail/'+formData.value.storeId)
                }
            }

            
        } else {
            return false
        }
    })
}


const coverValidator = (rule, value, callback) => {
    if (newStoreFlag.value && !formData.value.cover) {
        callback(new Error('请上传商店封面'))
    } else {
        callback()
    }
}
const rules = {
    name: [
        { required: true, message: '请输入商店名称', trigger: 'change' },
        { min: 2, max: 20, message: '长度在 2 到 20 个字符', trigger: 'change' }
    ],
    selectedRegion: [
        { required: true, message: '请选择发货地址' }
    ],
    detailAddress: [
        { required: true, message: '请输入详细地址', trigger: 'change' }
    ],
    cover: [
        { validator:coverValidator, trigger: 'change' }
    ]
}
</script>

<style scoped lang="scss">
main{
    padding-top: 60px;
    width: var(--global-width);
    margin: 0 auto;
    .content{
        border-radius: 20px;
        background-color: white;
        overflow: auto;
        padding-bottom: 40px;
        .header{
            font-size: 30px;
            padding-left: 20px;
            margin-top: 20px;
            padding-bottom: 10px;
            border-bottom: 1px solid #cfcfcf;
        }
        .form{
            margin-top: 40px;
            margin-left: 20px;
            
            .el-form-item{
                width: 400px;
                margin-bottom: 40px;
                :deep(.el-form-item__label){
                    font-size: 20px;
                    margin-bottom: 20px;
                }
                :deep(.el-cascader){
                    margin-bottom: 20px;
                }
            }
            
        }
        button {
            margin-left: 20px;
            outline: none;
            border: none;
            background-color: var(--main-color-purple);
            border-radius: 8px;
            height: 40px;
            width: 400px;
            color: white;
            font-size: 20px;
            cursor: pointer;
            &:hover{
            
                background-color: var(--main-color-purple-hover);
            }
        }
    }
}
</style>