<template>
	<main>
		<div class="content">
			<div :class="['login', accountType == 'register' ? 'register' : '']">
				<p class="title">
					{{ accountType.toUpperCase() }}
				</p>
				<div class="account">
					<input v-model="formData.userName" type="text" placeholder="用户名">
				</div>
				<div class="password">
					<input @keyup.enter="loginOrRegister" ref="passwordRef" v-model="formData.password" type="password" placeholder="密码">
					<el-icon v-if="formData.password && formData.password.length > 0" @click="switchPwdView('password')">
						<View />
					</el-icon>
					<!-- <el-icon><Hide /></el-icon> -->
				</div>
				<div v-if="accountType === 'register'" class="password">
					<input @keyup.enter="loginOrRegister" v-model="formData.confirmPassword" type="password" ref="confirmPasswordRef" placeholder="确认密码">
					<el-icon v-if="formData.confirmPassword && formData.confirmPassword.length > 0"
						@click="switchPwdView('confirmPassword')">
						<View />
					</el-icon>
				</div>
				<el-radio-group v-if="accountType === 'register'" v-model="formData.gender" class="radio-group">
					<el-radio label="1" size="large">
						男
					</el-radio>
					<el-radio label="2" size="large">
						女
					</el-radio>
				</el-radio-group>
				<button @click="loginOrRegister">
					{{ accountType === 'login' ? '登录' : '注册' }}
				</button>
			</div>
			<div :class="['cover', accountType == 'register' ? 'register-cover' : '']">
				<p>欢迎来到<span>XX商城</span></p>
				<div class="img">
					<img src="https://pic.imgdb.cn/item/6488985b1ddac507cc1f59cd.jpg?w=300" alt="">
				</div>
				<span>{{ accountType === 'login' ? '没有' : '已有' }}账号?</span>
				<button @click="changeType">
					去{{ accountType === 'login' ? '注册' : '登录' }}
				</button>
			</div>
		</div>
	</main>
</template>

<script setup>
import { useRoute, useRouter } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo.js'
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import message from '@/utils/message.js'
import { login, register } from '@/api/user.js'
const route = useRoute()
const router = useRouter()
const userInfoStore = useUserInfoStore()
watch(() => route.params.type, (val) => {
	accountType.value = val
})
onMounted(() => {
	if (userInfoStore.needLogin) {
		useUserInfoStore.needLogin = false
		message.warning('请先登录')
	}	
})
onBeforeUnmount(() => {
	userInfoStore.needLogin = false	
})
const accountType = ref(route.params.type)
const formData = ref({
	gender: '1'
})
const passwordRef = ref(null)
const confirmPasswordRef = ref(null)
const changeType = () => {
	if (accountType.value === 'login') {
		formData.value = { gender: '1' }
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
		if (formData.value.password.length < 8) {
			message.error('密码长度不能小于8位')
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
			router.go(-1)
		}

	} else {
		if (formData.value.password.length < 8) {
			message.error('密码长度不能小于8位')
			return
		}
		if (formData.value.password !== formData.value.confirmPassword) {
			message.error('两次密码不一致')
			return
		}
		const request = {
			userName: formData.value.userName,
			password: formData.value.password,
			gender: formData.value.gender
		}
		const res = await register(request)
		if (res) {
			message.success('注册成功, 请登录')
			router.replace('/account/login')
		}
	}
}
const switchPwdView = (name) => {
	if (name === 'password') {
		passwordRef.value.type = passwordRef.value.type === 'password' ? 'text' : 'password'
	} else {
		confirmPasswordRef.value.type = confirmPasswordRef.value.type === 'password' ? 'text' : 'password'
	}
}  
</script>

<style lang="scss" scoped>
$main-color: rgb(205, 198, 235);
$white-hover: rgb(243, 255, 247);

main {
	height: 100vh;
	width: 100%;
	display: flex;
	justify-content: center;
	align-items: center;

	.content {
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

			p {
				font-size: 40px;
				color: white;
				font-weight: 600;
				letter-spacing: 8px;
				margin-bottom: 30px;
			}

			div {
				width: 240px;
				height: 40px;
				margin-top: 20px;
				color: white;
				position: relative;

				input {
					border: none;
					border-bottom: 1px solid white;
					height: 100%;
					width: 100%;
					color: white;
					background-color: transparent;
					padding-left: 10px;
					outline: none;
					font-size: 18px;

					&::placeholder {
						color: white;
						font-size: 18px;
						// font-weight: 600;
					}
				}

				.el-icon {
					right: 2px;
					top: 14px;
					position: absolute;
					cursor: pointer;
				}
			}

			button {
				width: 240px;
				height: 46px;
				border-radius: 10px;
				margin-top: 30px;
				border: none;
				background-color: white;
				font-size: 18px;
				cursor: pointer;
				transition: all 0.2s;

				&:hover {
					background-color: $white-hover;
				}

				color: $main-color;
			}

			.radio-group {
				display: flex;
				justify-content: space-around;
			}
		}

		.cover {
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
			p {
				width: 100%;
				font-size: 30px;
				color: rgb(116, 116, 116);
				font-weight: 600;
				letter-spacing: 4px;
				text-align: center;
				margin-top: 30px;

				span {
					color: $main-color;
				}
			}

			.img {
				width: 100%;
				padding: 0 30px;
				height: 280px;

				img {
					width: 100%;
					height: 100%;
					object-fit: cover;
				}
			}

			span {
				width: 100%;
				text-align: center;
				color: #5a5a5a;
			}

			button {
				width: 100px;
				height: 35px;
				border-radius: 8px;
				border: 1px solid $main-color;
				background-color: transparent;
				color: $main-color;
				cursor: pointer;

				&:hover {
					background-color: $main-color;
					color: white;
				}
			}
		}

		.register {
			left: calc(100% - 350px - 40px);
		}

		.register-cover {
			right: calc(100% - 350px - 40px);
		}

	}
}
</style>
