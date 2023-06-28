<template>
  <main>
    <DataList>
      <template #default="{data}">
        <ProductItem :data="data" />
      </template>
    </DataList>
  </main>
</template>

<script setup>
import DataList from '@/component/DataList.vue'
import ProductItem from './ProductItem.vue'
const props = defineProps({
    data: {
        type: Array,
        default: () => []
    }
})
</script>

<style>

</style>
