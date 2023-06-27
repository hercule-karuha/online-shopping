import { defineStore } from "pinia"
import { ref } from 'vue'
export const usePurchaseListStore = defineStore('purchaseList', () => {
    const list = ref([])
    function add (item) {
        if (!list.value.includes(item)) {
            list.value.push(item)
        }
    }
    function remove (item) {
        const index = list.value.indexOf(item)
        if (index > -1) {
            list.value.splice(index, 1)
        }
    }
    return { list, add, remove }
})