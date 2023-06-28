<template>
	<div class="product-item"> 
		<div class="cover" @click="goDetail(data.productId)">
			<img :src="proxy.globalInfo.productCoverUrl+data.productId" alt="">
		</div>
		<div class="info">
			<div @click="goDetail(data.productId)" class="name">
				{{ data.productName }}
			</div>
			<div class="edit" v-if="showEdit" >
				<span @click="goEdit(data.productId)"><el-icon><Edit /></el-icon>编辑商品</span>
				<span @click="deleteProduct(data.productId)">删除</span>
			</div>
			<div class="sales" v-if="data.sales">
				总销量: <span>{{ data.sales }}</span>
				
			</div>
			<div class="price">
				¥ {{ Number.parseFloat(data.price).toFixed(2) }}
			</div>
		</div>
	</div>
</template>

<script setup>
import { getCurrentInstance } from 'vue'
import { useRouter } from 'vue-router'
import { useUserInfoStore } from '@/stores/userInfo.js'
const userInfoStore = useUserInfoStore()
const router = useRouter()
const { proxy } = getCurrentInstance()
const props = defineProps({
	data: {
		type: Object,
		default: () => { }
	},
	showEdit: {
		type: Boolean,
		default: false
	}
})
const emit = defineEmits(['delProduct'])
const goEdit = (productId) => {
	router.push('/product/edit/' + productId)
}
const deleteProduct = (productId) => {
	emit('delProduct', productId)
}
const goDetail = (productId) => {
	window.open('/product/detail/' + productId, '_blank')
}
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
		.edit{
			display: flex;
			align-items: center;
			justify-content: space-between;
			span{
				display: flex;
				align-items: center;
				cursor: pointer;
				&:hover{
					color: var(--main-color-purple);
				}
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
