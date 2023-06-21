import axios from 'axios'
import message from './message'
import { useUserInfoStore } from '@/stores/userInfo'
import nProgress from 'nprogress'
const userInfoStore = useUserInfoStore()
const instance = axios.create({
    baseURL: '/api',
    timeout: 10 * 1000
})
let isLoading = false
// 请求拦截器
instance.interceptors.request.use(config => {
    if (config.showLoading && !isLoading) {
        nProgress.start()
        isLoading = true
    }
    return config
}, error => {
    if (isLoading) {
        nProgress.done()
        isLoading = false
    }
    message.error('请求失败')
    return Promise.reject(error)
})

// 响应拦截器
instance.interceptors.response.use(response => {
    const { showLoading, errorCallback, successCallback, showError } = response.config
    // 防止多个请求时，提前关闭loading
    if (isLoading && showLoading) {
        nProgress.done()
        isLoading = false
    }
    const responseData = response.data
    if (responseData.code === 200) {
    // 请求成功
        if (successCallback) {
            successCallback(responseData)
        }
        return responseData
    } else if (responseData.code === 900) {
    // 需要登录
        userInfoStore.userInfo = {}
        userInfoStore.needLogin = true
        return Promise.reject(responseData)
    } else {
    // 其他错误
        if (errorCallback) {
            errorCallback(responseData)
        }
        if (showError) {
            message.error(responseData.msg)
        }
        return Promise.reject(responseData)
    }
}, error => {
    if (isLoading) {
        nProgress.done()
        isLoading = false
    }
    message.error(error.message)
    return Promise.reject(error)
})

const request = async config => {
    const { url, data, params, method, showLoading = true, errorCallback, successCallback, showError = true } = config
    return await instance({
        url,
        method,
        data,
        params,
        showLoading,
        errorCallback,
        successCallback,
        showError
    })
}

export default request
