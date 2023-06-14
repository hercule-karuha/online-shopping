<template>
  <main>
    <div class="content">
      <div :class="['login', accountType=='register'?'register':'']">
        <p class="title">
          {{ accountType.toUpperCase() }}
        </p>
        <div class="account">
          <input
            v-model="formData.userName"
            type="text"
            placeholder="用户名"
          >
        </div>
        <div class="password">
          <input
            v-model="formData.password"
            type="password"
            placeholder="密码"
          >
        </div>
        <div
          v-if="accountType==='register'"
          class="password"
        >
          <input
            type="password"
            placeholder="确认密码"
            v-model="formData.confirmPassword"
          >
        </div>
        <el-radio-group
            v-if="accountType==='register'"
          v-model="formData.sex"
          class="radio-group"
        >
          <el-radio
            label="1"
            size="large"
          >
            男
          </el-radio>
          <el-radio
            label="2"
            size="large"
          >
            女
          </el-radio>
        </el-radio-group>
        <button @click="loginOrRegister">
          {{ accountType==='login'?'登录':'注册' }}
        </button>
      </div>
      <div :class="['cover',accountType=='register'?'register-cover':'']">
        <p>欢迎来到<span>XX商城</span></p>
        <div class="img">
          <img
            src="https://pic.imgdb.cn/item/6488985b1ddac507cc1f59cd.jpg"
            alt=""
          >
        </div>
        <span>{{ accountType==='login'?'没有':'已有' }}账号?</span>
        <button @click="changeType">
          去{{ accountType==='login'?'注册':'登录' }}
        </button>
      </div>
    </div>
  </main>
</template>

<script setup>
import { useRoute, useRouter } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo.js'
import { ref, watch } from 'vue'
import message from '@/utils/message.js'
import { login, register } from '@/api/user.js'
const route = useRoute()
const router = useRouter()
const userInfoStore = useUserInfoStore()
watch(() => route.params.type, (val) => {
    accountType.value = val
})
const accountType = ref(route.params.type)
const formData = ref({})
const changeType = () => {
    if (accountType.value === 'login') {
        router.replace('/account/register')
    } else {
        router.replace('/account/login')
    }
}

const loginOrRegister = async () => {
    if (!formData.value.userName) {
        message.error('请输入用户名', null)
        return
    }
    if (!formData.value.password) {
        message.error('请输入密码')
        return
    }
    if (accountType.value == 'login') {
        if (formData.value.password.length < 6) {
            message.error('密码长度不能小于6位')
            return
        }
        const request = {
            userName: formData.value.userName,
            password: formData.value.password
        }
        const res = await login(request)
        if (res) {
            userInfoStore.userInfo = res.data
            message.success('登录成功')
            router.push('/')
        }
        
    } else {
        if (formData.value.password.length < 6) {
            message.error('密码长度不能小于6位')
            return
        }
        if (formData.value.password !== formData.value.confirmPassword) {
            message.error('两次密码不一致')
            return
        }
        const request = {
            userName: formData.value.userName,
            password: formData.value.password,
            sex: formData.value.sex
        }
        const res = await register(request)
        if (res) {
            message.success('注册成功')
            router.push('/account/login')
        }
    }
}

</script>

<style lang="scss" scoped>
$main-color: rgb(205, 198, 235);
$white-hover: rgb(243, 255, 247);
main{
    height: 100vh;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    .content{
        height: 500px;
        width: 800px;
        border-radius: 10px;
        background-color: rgb(255, 255, 255);
        display: flex;
        position: relative;
        .login {
            position: absolute;
            border-radius: 10px;
            height: calc(100% + 50px);
            top: -25px;
            left: 40px;
            width: 350px;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            box-shadow: 0 0 14px rgba(0, 0, 0, 0.1);
            background-color: $main-color;
            transition: all 0.5s;
            z-index: 100;
            p{
                font-size: 40px;
                color: white;
                font-weight: 600;
                letter-spacing: 8px;
                margin-bottom: 30px;
            }
            div{
                width: 240px;
                height: 40px;
                margin-top: 20px;
                color: white;
                input{
                    border: none;
                    border-bottom: 1px solid white;
                    height: 100%;
                    width: 100%;
                    color: white;
                    background-color: transparent;
                    padding-left: 10px;
                    outline: none;
                    font-size: 18px;
                    &::placeholder{
                        color: white;
                        font-size: 18px;
                        // font-weight: 600;
                    }
                }
            }
            button{
                width: 240px;
                height: 46px;
                border-radius: 10px;
                margin-top: 30px;
                border: none;
                background-color: white;
                font-size: 18px;
                cursor: pointer;
                transition: all 0.2s;
                &:hover{
                    background-color: $white-hover;
                }
                color: $main-color;
            }
            .radio-group{
                display: flex;
                justify-content: space-around;
            }
        }
        .cover{
            position: absolute;
            right: 0;
            padding-right: 40px;
            width: 350px;
            height: 100%;
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            transition: all 0.5s;
            // align-items: center;
            p{
                width: 100%;
                font-size: 30px;
                color: rgb(116, 116, 116);
                font-weight: 600;
                letter-spacing: 4px;
                text-align: center;
                margin-top: 30px;
                span{
                    color: $main-color;
                }
            }
            .img{
                width: 100%;
                padding: 0 30px;
                height: 280px;
                img{
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
            }
            span{
                width: 100%;
                text-align: center;
                color: #5a5a5a;
            }
            button{
                width: 100px;
                height: 35px;
                border-radius: 8px;
                border: 1px solid $main-color;
                background-color: transparent;
                color: $main-color;
                cursor: pointer;
                &:hover{
                    background-color: $main-color;
                    color: white;
                }
            }
        }
        .register{
            left: calc(100% - 350px - 40px);
        }
        .register-cover{
            right: calc(100% - 350px - 40px);
        }

    }
}
</style>