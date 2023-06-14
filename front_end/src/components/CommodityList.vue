<template>
  <main>
    <DataList>
      <template #default="{data}">
        <CommodityItem :data="data" />
      </template>
    </DataList>
  </main>
</template>

<script setup>
import DataList from './DataList.vue'
import CommodityItem from './CommodityItem.vue'
const props = defineProps({
    data: {
        type: Array,
        default: []
    }
})
</script>

<style>

</style>