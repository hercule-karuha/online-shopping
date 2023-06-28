<template>
    <main>
        <header>
            <div class="info">
                <div class="avatar">
                    <img :src="proxy.globalInfo.avatarUrl+userInfoStore.userInfo.userId" alt="">
                </div>
                <div class="name">
                    <span>{{ userInfoStore.userInfo.userName }}</span>
                    <span>
                        <el-icon v-if="userInfoStore.userInfo.sex == 1"><Male /></el-icon>
                        <el-icon v-else><Female /></el-icon>
                    </span>
                </div>
                <span class="edit" @click="toEditUserInfo"><el-icon><Edit /></el-icon>编辑个人信息</span>
            </div>
            <nav>
                <ul class="tabs">
                    <li><a @click="changeTab('/user/shoppingCart')"
                    :class="[route.name=='shoppingCart'?'active_tab':'']">购物车</a></li>
                    <li><a @click="changeTab('/user/order')" 
                        :class="[route.name=='order'?'active_tab':'']">购买记录</a></li>
                    <!-- <li><a href="">买过的店</a></li> -->
                    <li><a @click="changeTab('/user/stores')" 
                        :class="[route.name=='stores'?'active_tab':'']">我的店铺</a></li>
                </ul>
            </nav>
        </header>
        <div class="content">
            <router-view></router-view>
        </div>
        <DialogModule :show="dialogConfig.show"
        :title="dialogConfig.title"
        @close="dialogConfig.show=false"
        :showCancel="false"
        :buttons="dialogConfig.buttons">
        <div class="dialog-body">
                <el-form ref="formDataRef" :model="formData" :rules="rules" label-position="top"
                >
                    <el-form-item
                        label="请上传你的头像" prop="avatar">
                    <CoverUpload @upload-image="uploadCover"
                    :imageUrl="userInfoStore.userInfo.userId?proxy.globalInfo.avatarUrl+userInfoStore.userInfo.userId:''"
                    :update="true"></CoverUpload>
                    </el-form-item>
                    <el-form-item label="性别" prop="gender">
                        <el-radio-group v-model="formData.gender" class="ml-4">
                    <el-radio label="1" size="large">男</el-radio>
                    <el-radio label="2" size="large">女</el-radio>
                    </el-radio-group>
                    </el-form-item>
                    <el-form-item label="手机号" prop="phone">
                        <el-input v-model="formData.phone"></el-input>
                    </el-form-item>
                    <el-form-item label="默认收货地区" prop="area">
                        <el-cascader :options="options" v-model="formData.area"></el-cascader>
                    </el-form-item>
                    <el-form-item label="默认详细地址" prop="detailAddress">
                        <el-input v-model="formData.detailAddress"></el-input>
                    </el-form-item>
                </el-form>
            </div>
        </DialogModule>
    </main>
</template>

<script setup>
import { ref, watch, reactive, getCurrentInstance } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import DialogModule from '@/components/DialogModule.vue'
import { parseAddressCodeArr } from '@/utils/tools'
import message from '@/utils/message'
import { regionData } from 'element-china-area-data'
import { useUserInfoStore } from '@/stores/userInfo'
import { getDetailUserInfo, editUserInfo } from '@/api/user'
const { proxy } = getCurrentInstance()
const options = regionData
const userInfoStore = useUserInfoStore()
console.log(userInfoStore.userinfo)
const route = useRoute()
const router = useRouter()


const dialogConfig = reactive({
    show: false,
    title: '编辑个人信息',
    buttons: [
        {
            text: '取消',
            type: 'default',
            click: () => {
                dialogConfig.show = false
            }
        },
        {
            text: '确定',
            type: 'primary',
            click: async () => {
                if (!formDataRef.value.validate()) return
                const uploadForm = new FormData()
                if (formData.value.avatar) {
                    uploadForm.append('avatar', formData.value.avatar)
                }
                uploadForm.append('userName', formData.value.userName)
                uploadForm.append('gender', formData.value.gender)
                uploadForm.append('phone', formData.value.phone)
                uploadForm.append('address', JSON.stringify({
                    codeArr: formData.value.area,
                    areaArr: parseAddressCodeArr(formData.value.area),
                    detailAddress: formData.value.detailAddress
                }))
                const res = await editUserInfo(uploadForm)
                if (!res) return
                message.success('修改成功')
                dialogConfig.show = false
            }
        }
    ]
})


const formDataRef = ref(null)
const formData = ref({})

const toEditUserInfo = async () => {
    const res = await getDetailUserInfo()
    if (!res) return
    const resData = res.data
    formData.value.userName = resData.userName
    formData.value.gender = resData.sex
    formData.value.phone = resData.phone
    const address = JSON.parse(resData.address)
    formData.value.area = address.codeArr
    
    formData.value.detailAddress = address.detailAddress
    dialogConfig.show = true
}

const uploadCover = (file) => {
    formData.value.cover = file
}


const changeTab = (to) => {
    router.replace(to)
}

const rules = {
    userName: [
        {required: true, message: '请输入用户名', trigger: 'blur'},
    ],
    gender: [
        {required: true, message: '请选择性别', trigger: 'blur'},
    ],
    phone: [
        {required: true, message: '请输入手机号', trigger: 'blur'},
        {pattern: /^1[3456789]\d{9}$/, message: '请输入正确的手机号', trigger: 'blur'}
    ],
    area: [
        {required: true, message: '请选择地区', trigger: 'blur'},
    ],
    detailAddress: [
        {required: true, message: '请输入详细地址', trigger: 'blur'},
    ],
}
</script>

<style scoped lang="scss">
main{
    width: var(--global-width);
    margin: 0 auto;
    header {
        margin-top: 40px;
        background-color: var(--main-color-purple);
        height: 160px;
        border-radius: 20px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        overflow: hidden;
        .info{
            display: flex;
            margin-left: 40px;
            margin-top: 40px;
            flex: 1;
            color: white;
            align-items: start;
            .avatar{
                width: 60px;
                height: 60px;
                border-radius: 50%;
                overflow: hidden;
                cursor: pointer;
                transition: all 0.4s;
                img{
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
                &:hover{
                    transform: rotate(180deg);
                }
            }
            .name{
                margin-left: 20px;
                font-size: 24px;
                display: flex;
                flex-direction: column;
                line-height: auto !important;
                span{
                    margin-top: 0;
                    .el-icon{
                        font-size: 20px;
                    }
                    .male{
                        color: rgb(143, 178, 255);
                    }
                    .female{
                        color: rgb(255, 143, 143);
                    }
                }
            }
            .edit{
                margin: 6px 30px;
                font-size: 18px;
                display: flex;
                align-items: center;
                cursor: pointer;
                &:hover{
                    color: rgb(240, 255, 211);
                    text-decoration: underline;
                }
            }
        }
        nav{
            justify-self: flex-end;
            display: flex;
            justify-content: center;
            ul {
                list-style-type: none;
                display: flex;
                li{
                    cursor: pointer;
                    margin-bottom: 5px;
                    a{
                        text-decoration: none;
                        padding: 10px;
                        color: white;
                        font-size: 18px;
                        border-radius: 10px 10px 0 0;
                    }
                    .active_tab{
                        background-color: white;
                        color: var(--main-color-purple);
                    }
                }
            }
            
        }
    }
    .content{
        margin-top: 40px;
    }
}
.dialog-body{
    max-width: 400px;
    // width: 100%;
}
</style>