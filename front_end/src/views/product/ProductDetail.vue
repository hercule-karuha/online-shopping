<template>
	<div class="content">
		<main>
			<div class="cover">
				<img :src="productInfo.productId ? proxy.globalInfo.productCoverUrl + productInfo.productId : ''" alt="">
			</div>
			<div class="purchase-info">
				<div class="name">
					{{ productInfo.productName }}
				</div>
				<div class="sales info-item">
					<span class="label">销量</span>
					<span class="info">{{ productInfo.sales }}</span>
				</div>
				<div class="price info-item">
					<span class="label">价格</span>
					<span class="info">¥{{ productInfo.price }}</span>
				</div>
				<div class="description info-item">
					<span class="label">描述</span>
					<span class="info">{{ productInfo.description }}</span>
				</div>
				<div class="address info-item">
					<span class="label">发货</span>
					<span class="info">{{ productInfo.address }}</span>
				</div>
				<div class="stock info-item">
					<span class="label">库存</span>
					<span class="info">{{ productInfo.stock }}</span>
				</div>
				<div class="number-select">
					<el-input-number v-model="num" :min="1" :max="productInfo.stock" />
				</div>
				<div class="purchase">
					<button :class="[productInfo.canbuy == '0' ? 'canot' : '']" @click="handlePurchase">{{ productInfo.canbuy ==
						'0' ? '已下架' : '立即购买' }}</button>
					<button v-if="productInfo.canbuy == '1'" @click="add2ShoppingCart">加入购物车</button>
				</div>
			</div>
		</main>
		<div class="store" :style="{ 'background-image': 'url(' + proxy.globalInfo.storeCoverUrl + productInfo.storeId + ')' }">
			<div class="bg">
				<div class="store-cover">
					<img :src="productInfo.storeId ? proxy.globalInfo.storeCoverUrl + productInfo.storeId : ''" alt="">
				</div>
				<div class="store-info">
					{{ storeInfo.name }}
				</div>
				<button @click="router.push('/store/detail/' + productInfo.storeId)">进店逛逛</button>
			</div>
		</div>
		<div class="detail">
			<el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
				<el-tab-pane label="商品详情" name="details">
					<div class="detail-images">
						<div v-for="(item, index) in detailImages" :key="index" class="detail-image">
							<img :src="proxy.globalInfo.imgUrl + item" alt="">
						</div>
					</div>
				</el-tab-pane>
				<!-- <el-tab-pane label="Config" name="second">
					Config
				</el-tab-pane> -->
			</el-tabs>
		</div>
	</div>
</template>

<script setup>
import { useRoute, useRouter } from 'vue-router'
import { ref, watch, getCurrentInstance, onBeforeUnmount, onMounted } from 'vue'
import { getProduct } from '@/api/product.js'
import { getStoreInfo } from '@/api/store.js'
import { parseAddressCodeArr } from '@/utils/tools'
import { addShoppingCart } from '@/api/purchase.js'
import { usePurchaseListStore } from '@/stores/purchaseList.js'
import message from '@/utils/message.js'
const purchaseListStore = usePurchaseListStore()
const { proxy } = getCurrentInstance()
const route = useRoute()
const detailImages = ref([])
const storeInfo = ref({})
onMounted(async () => {
	const res = await getProduct(route.params.id)
	if (!res) {
		return
	}
	productInfo.value = res.data
	detailImages.value = res.data.detailImages.split(',')
	const address = JSON.parse(res.data.address)
	productInfo.value.stock = Number.parseInt(productInfo.value.stock)
	productInfo.value.address = parseAddressCodeArr(address.codeArr).join('  ') + address.detailAddress
	getStoreInfo(res.data.storeId).then(res => {
		storeInfo.value = res.data
	})
})

const router = useRouter()
const productInfo = ref({})

const handleClick = (panel) => {
	console.log(panel)
}
const activeName = ref('details')
const num = ref(1)

const handlePurchase = () => {
	if (productInfo.value.canbuy == '0') {
		return
	}
	purchaseListStore.add({
		productId: productInfo.value.productId,
		num: num.value,
		price: productInfo.value.price,
		name: productInfo.value.name,
	})

	router.push('/purchase')
}
const add2ShoppingCart = async () => {
	let request = {
		list: [
			{
				productId: productInfo.value.productId,
				num: num.value.toString()
			}
		]
	}
	const res = await addShoppingCart(request)
	if (!res) {
		return
	}
	message.success('添加成功')
}
</script>

<style scoped lang="scss">
.content {
	margin: 0 auto;
	width: var(--global-width);
	padding-top: 40px;

	main {
		display: flex;
		background-color: white;
		padding: 20px;
		border-radius: 20px;

		.cover {
			width: 400px;
			height: 400px;
			margin-right: 60px;

			img {
				width: 100%;
				height: 100%;
				object-fit: cover;
				border-radius: 10px;
				box-shadow: 0 0 18px rgba(0, 0, 0, 0.2);
			}
		}

		.purchase-info {

			.name {
				font-size: 30px;
				padding-bottom: 20px;
				border-bottom: 1px solid #dbdbdb;
			}

			.info-item {
				margin-top: 16px;
				overflow-wrap: break-word;

				.label {
					display: inline-block;
					width: 60px;
					font-size: 18px;
				}

				.info {
					font-size: 18px;
				}
			}

			.price {
				.info {
					font-size: 24px;
					color: #ff5f5f;
				}
			}

			.number-select {
				margin-top: 20px;
			}

			.purchase {
				margin-top: 20px;

				button {
					width: 150px;
					height: 40px;
					border-radius: 10px;
					border: none;
					outline: none;
					font-size: 20px;
					color: white;
					cursor: pointer;

					&:first-child {
						margin-right: 20px;
						background-color: #ff5f5f;

						&:hover {
							background-color: #ff7f7f;
						}
					}

					&:last-child {
						background-color: #ff9f5f;

						&:hover {
							background-color: #ffbf7f;
						}
					}

				}
			}
		}

	}

	.store {
		margin-top: 30px;
		border-radius: 20px;
		background-size: cover;
		overflow: hidden;

		.bg {
			padding: 20px;
			display: flex;
			backdrop-filter: blur(5px);
			position: relative;

			&::after {
				content: '';
				position: absolute;
				background-image: linear-gradient(to right, transparent 0%, #ffffff 50%);
				height: 100%;
				width: 100%;
				top: 0;
				left: 0;
				z-index: -1;
			}

			.store-cover {
				height: 100px;
				width: 100px;

				img {
					height: 100%;
					width: 100%;
					object-fit: cover;
					border-radius: 10px;
				}
			}

			.store-info {
				margin-left: 30px;
				font-size: 30px;
				display: flex;
				align-items: center;
				letter-spacing: 10px;
				position: relative;

				// cursor: none;
				&::after {
					transition: all 0.3s ease;
					content: '';
					position: absolute;
					background-color: var(--main-color-purple);
					height: 3px;
					width: 0;
					bottom: 20px;
					left: -4px;
				}

				&:hover {
					&::after {
						width: 100%;
					}
				}
			}

			button {
				margin-left: auto;
				width: 150px;
				background-color: var(--main-color-purple);
				border-radius: 10px;
				border: none;
				font-size: 20px;
				color: white;
				cursor: pointer;

				&:hover {
					background-color: var(--main-color-purple-hover);
				}
			}
		}
	}

	.detail {
		background-color: white;
		margin-top: 30px;
		border-radius: 20px;
		padding: 20px;

		&:deep(.el-tab-pane) {
			font-size: 20px;
		}

		.detail-images {
			margin: 20px;

			.detail-image {
				margin-bottom: 20px;

				img {
					width: 100%;
					object-fit: cover;
				}
			}
		}
	}
}

.canot {
	background-color: #ddd !important;
}</style>
