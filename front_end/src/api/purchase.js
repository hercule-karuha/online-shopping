import request from '@/utils/request'
export const addShoppingCart = (data) => {
    return request({
        url: '/purchase/addShoppingCart',
        method: 'post',
        data
    })
}
export const editShoppingCart = (data) => {
    return request({
        url: '/purchase/editShoppingCart',
        method: 'post',
        data
    })
}
export const immediatePurchase = (data) => {
    return request({
        url: '/purchase/immediatePurchase',
        method: 'post',
        data
    })
}