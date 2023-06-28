<template>
	<div class="content" :style="{ 'width': size + 'px', 'height': size + 'px' }">
		<img v-if="userId !== ''" :src="proxy.globalInfo.avatarUrl + userId" @click="goUserCenter">
		<div v-else class="fake-avatar">
			未登录
		</div>
	</div>
</template>

<script setup>
import { getCurrentInstance } from 'vue'
import { useRouter } from 'vue-router'
const { proxy } = getCurrentInstance()
const router = useRouter()
const props = defineProps({
	userId: {
		type: Number,
		default: ''
	},
	size: {
		type: Number,
		default: 50
	},
	addLink: {
		type: Boolean,
		default: true
	}
})
const goUserCenter = () => {
	if (props.addLink) {
		router.push({
			path: '/user',
			query: {
				userId: props.userId
			}
		})
	}
}
</script>

<style lang="scss" scoped>
.content {
	border-radius: 50%;
	overflow: hidden;

	img {
		width: 100%;
		height: 100%;
		cursor: pointer;
	}

	.fake-avatar {
		width: 100%;
		height: 100%;
		background-color: #ddd;
		border-radius: 50%;
		display: flex;
		justify-content: center;
		align-items: center;
		user-select: none;
	}
}
</style>
