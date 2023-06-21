<template>
	<div class="content">
		<main>
			<div class="cover">
				<img :src="productInfo.cover" alt="">
			</div>
			<div class="purchase-info">
				<div class="name">
					{{ productInfo.name }}
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
					<span class="info">{{ productInfo.descriton }}</span>
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
					<button>立即购买</button>
					<button>加入购物车</button>
				</div>
			</div>
		</main>
		<div class="store">
			<div class="store-cover">
				<img src="https://pic.imgdb.cn/item/63f8d55bf144a01007fe3722.jpg" alt="">
			</div>
			<div class="store-info">
				{{ productInfo.storeName }}
			</div>
			<button>进店逛逛</button>
		</div>
		<div class="detail">
			<el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
				<el-tab-pane label="商品详情" name="details">
					<div class="detail-images">
						<div v-for="(item, index) in detailImages" :key="index" class="detail-image">
							<img :src="item" alt="">
						</div>
					</div>
				</el-tab-pane>
				<el-tab-pane label="Config" name="second">
					Config
				</el-tab-pane>
			</el-tabs>
		</div>
	</div>
</template>

<script setup>
import { useRoute } from 'vue-router'
import { ref, watch } from 'vue'
const route = useRoute()
const productId = ref(route.params.id)
watch(() => route.params.id, (val) => {
	productId.value = val
})
const productInfo = {
	productId: 1,
	cover: 'https://pic.imgdb.cn/item/64475b180d2dde5777857756.webp',
	name: 'XXxxxxxxxxxxxxXXXXXXXXXXX',
	descriton: 'xxxxxxxxxxxxxxxxxxxxx xwewxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
	price: 100,
	storeName: 'XX商城',
	storeId: 1,
	address: '北京市朝阳区，XX街道，XX小区，XX号楼，XX单元，XX室',
	sales: 100,
	stock: 100,
	detailImages: `https://pic.imgdb.cn/item/64475aff0d2dde577785597a.jpg,
    https://pic.imgdb.cn/item/63fb0486f144a01007e0e1f1.jpg,
    https://pic.imgdb.cn/item/63f7abaaf144a0100735172d.jpg`
}
const detailImages = productInfo.detailImages.split(',')
const handleClick = (panel) => {
	console.log(panel)
}
const activeName = ref('details')
const num = ref(1)
</script>

<style scoped lang="scss">
.content {
	margin: 0 auto;
	width: var(--global-width);
	padding-top: 80px;

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
		background-color: white;
		padding: 20px;
		border-radius: 20px;
		display: flex;

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

	.detail {
		background-color: white;
		margin-top: 30px;
		border-radius: 20px;
		padding: 20px;

		&:deep(.el-tab-pane) {
			font-size: 20px;
		}

		.detail-images {
			margin-top: 20px;

			.detail-image {
				margin-bottom: 20px;

				img {
					object-fit: cover;
				}
			}
		}
	}
}
</style>
