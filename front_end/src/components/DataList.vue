<template>
	<div>
		<NoData v-if="dataSource.list.length === 0" />
		<div v-if="loading">
			<el-skeleton :rows="skeletonRows" animated />
		</div>
		<div class="box" v-else>
			<div class="content" :style="{'display': flex?'flex':''}">
				<slot v-for="(item, index) in dataSource.list" :key="index" :data="item" />
			</div>
			<div class="page">
				<el-pagination hide-on-single-page :page-size="Number.parseInt(dataSource.pageSize)" :page-count="Number.parseInt(dataSource.pageCount)"
					layout="prev, pager, next" :total="Number.parseInt(dataSource.total)" @update:current-page="pageNoChange" />
			</div>
		</div>
	</div>
</template>

<script setup>
import NoData from './NoData.vue'
const props = defineProps({
	dataSource: {
		type: Object,
		default: () => {
			return {
				list: []
			}
		}
	},
	loading: {
		type: Boolean,
		default: false
	},
	noDataMsg: {
		type: String,
		default: '空空如也'
	},
	skeletonRows: {
		type: Number,
		default: 5
	},
	flex: {
		type: Boolean,
		default: true
	}

})

const emit = defineEmits([
	'changePage'
])
const pageNoChange = (pageNo) => {
	emit('changePage', pageNo)
}
</script>

<style scoped lang="scss">
.box{
	
	width: 100%;
	.content{
		flex-wrap: wrap;
		width: 100%;
	}
}
.page {
	margin-top: 20px;
	display: flex;
	justify-content: center;
	padding-bottom: 20px;
	
}
</style>
