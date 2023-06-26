import request from "@/utils/request"
export const newStore = (data) => {
    return request({
        url: '/store/newStore',
        method: 'post',
        data
    })
}
export const editStore = (data) => {
    return request({
        url: '/store/editStore',
        method: 'post',
        data
    })
}
export const getStoreInfo = (id) => {
    return request({
        url: '/store/getStoreInfo/' + id,
        method: 'get'
    })
}
export const getStoreProductlist = (data) => {
    return request({
        url: '/store/getStoreProductlist',
        method: 'post',
        data
    })
}