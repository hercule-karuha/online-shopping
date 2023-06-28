<template>
	<div>
		<router-view v-slot="{ Component }">
			<transition enter-active-class="animate__animated animate__fadeIn animate__faster">
					<component :is="Component" />
			</transition>
		</router-view>
		<nav>
			<div title="返回顶部" @click="goTop">
				<el-icon>
					<ArrowUp />
				</el-icon>
			</div>
			<div title="用户中心" @click="goWithCheck('/user')">
				<el-icon>
					<User />
				</el-icon>
			</div>
			<div title="搜索" @click="router.push('/product/list')">
				<el-icon>
					<Search />
				</el-icon>
			</div>
			<div title="返回上一级" @click="router.go(-1)">
				<el-icon>
					<ArrowLeft />
				</el-icon>
			</div>
			<div title="主页" @click="router.push('/')">
				<el-icon>
					<House />
				</el-icon>
			</div>
			<div title="购物车" @click="goWithCheck('/user/shoppingCart')">
				<el-icon>
					<ShoppingCart />
				</el-icon>
			</div>
		</nav>
	</div>
</template>
<script setup>
import { watch, onBeforeMount, nextTick } from 'vue'
import { RouterView, useRouter, useRoute } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo.js'
const router = useRouter()
const route = useRoute()
const userInfoStore = useUserInfoStore()
let getUserInfoFunc = null
const goTop = () => {
	window.scrollTo({
		top: 0,
		behavior: 'smooth'
	})
}
watch(() => userInfoStore.needLogin, (val) => {
	if (val) {
		if (route.path === '/'){
			userInfoStore.needLogin = false
			return
		}
		router.push('/account/login')
	}
})

const goWithCheck = (path) => {
	if (userInfoStore.userInfo.userId) {
		router.push(path)
	} else {
		// router.push('/account/login')
		userInfoStore.needLogin = true
	}
}
onBeforeMount(async () => {
	// await异步加载 requst.js 否则会提示没有use(pinia)
	const { getUserInfo } = await import('@/api/user.js')
	getUserInfoFunc = getUserInfo
	const res = await getUserInfo()
	if (res.code === 200) {
		userInfoStore.userInfo = res.data
	}
})
</script>
<style lang="scss" scoped>
nav {
	position: fixed;
	bottom: 80px;
	left: 40px;
	border-radius: 50px;
	padding: 8px;
	background-color: rgb(255, 255, 255);
	display: flex;
	justify-content: center;
	align-items: center;
	flex-direction: column;
	box-shadow: 0 0 16px rgba(0, 0, 0, 0.2);

	div {
		margin-top: 10px;
		width: 30px;
		height: 30px;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 50%;

		&:hover {
			background-color: rgb(238, 238, 238);
			cursor: pointer;
		}

		.el-icon {
			font-size: 18px;
			display: flex;
			justify-content: center;
			align-items: center;
		}

		&:last-child {
			margin-bottom: 10px;
		}
	}
}</style>
