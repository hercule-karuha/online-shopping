<template>
	<div class="product-item"> 
		<div class="cover" @click="router.push('/product/detail/' + data.productId)">
			<img :src="proxy.globalInfo.productCoverUrl+data.productId" alt="">
		</div>
		<div class="info">
			<div @click="router.push('/product/detail/' + data.productId)" class="name">
				{{ data.productName }}
			</div>
			<div class="sales" v-if="data.sales">
				总销量: <span>{{ data.sales }}</span>
			</div>
			<div class="price">
				¥ {{ data.price }}
			</div>
		</div>
	</div>
</template>

<script setup>
import { getCurrentInstance } from 'vue'
import { useRouter } from 'vue-router'
const router = useRouter()
const { proxy } = getCurrentInstance()
const props = defineProps({
	data: {
		type: Object,
		default: () => { }
	}
})
</script>

<style scoped lang="scss">
.product-item {
	max-width: 340px;
	height: 180px;
	margin: 20px;
	display: flex;
	justify-content: space-between;
	padding: 10px;
	background-color: #f1f5fa;
	border-radius: 10px;
	transition: all 0.3s;
	border: solid 1px transparent;
	cursor: pointer;

	&:hover {
		background-color: white;
		border: 1px solid var(--main-color-purple);
	}

	.cover {
		width: 160px;
		height: 160px;
		margin-right: 20px;
		img {
			width: 100%;
			height: 100%;
			object-fit: cover;
			border-radius: 10px;
		}
	}

	.info {
		margin-top: 10px;
		margin-right: 10px;
		width: 130px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;

		.name {
			overflow-wrap: break-word;
			font-size: 16px;
			transition: all 0.2s;

			&:hover {
				color: rgb(255, 106, 106)
			}
		}
		.sales {
			font-size: 14px;
			color: #535353;
			span{
				color: rgb(194, 172, 124);
			}
		}
		.price {
			font-size: 24px;
			font-weight: 500;
			color: rgb(255, 106, 106);
		}
	}
}</style>
