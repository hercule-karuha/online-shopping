import { defineStore } from "pinia"
import { ref } from 'vue'
export const useUserInfoStore = defineStore('userInfo', () => {
    const userInfo = ref({})
    const needLogin = ref(false)
    return { userInfo, needLogin }
})