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
                <span class="edit" @click="dialogConfig.show=true"><el-icon><Edit /></el-icon>编辑个人信息</span>
            </div>
            <nav>
                <ul class="tabs">
                    <li><router-link to="/user/shoppingCart" 
                    :class="[route.name=='shoppingCart'?'active_tab':'']">购物车</router-link></li>
                    <li><router-link to="/user/order"
                        :class="[route.name=='order'?'active_tab':'']">购买记录</router-link></li>
                    <!-- <li><a href="">买过的店</a></li> -->
                    <li><router-link to="/user/stores"
                        :class="[route.name=='stores'?'active_tab':'']">我的店铺</router-link></li>
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
                        label="请上传你的头像" prop="cover">
                    <CoverUpload @upload-image="uploadCover"></CoverUpload>
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
import { useRoute } from 'vue-router'
import DialogModule from '@/components/DialogModule.vue'
import { parseAddressCodeArr } from '@/utils/tools'
import { regionData } from 'element-china-area-data'
import { useUserInfoStore } from '@/stores/userInfo'
const { proxy } = getCurrentInstance()
const options = regionData
const userInfoStore = useUserInfoStore()
console.log(userInfoStore.userinfo)
const route = useRoute()
watch(route, (to, from) => {
    console.log(to, from)
})

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
            click: () => {
                dialogConfig.show = false
            }
        }
    ]
})


const formDataRef = ref(null)
const formData = ref({})

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