<template>
  <NoData v-if="dataSource.list.length === 0" />
  <div v-if="loading">
    <el-skeleton
      :rows="skeletonRows"
      animated
    />
  </div>
  <main v-else>
    <div class="content">
      <slot
        v-for="(item, index) in dataSource.list"
        :key="index"
        :data="item"
      />
    </div>
    <div class="page">
      <el-pagination
        :page-size="list.pageSize"
        :pager-count="list.pageCount"
        layout="prev, pager, next"
        :total="list.total"
        @update:current-page="pageNoChange"
      />
    </div>
  </main>
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

</style>