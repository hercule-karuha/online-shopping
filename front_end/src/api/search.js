import request from "@/utils/request"
export const searchProduct = (data) => {
    return request({
        url: '/search/searchProduct',
        method: 'post',
        data
    })
}

export const searchOrders = (data) => {
    return request({
        url: '/search/searchOrders',
        method: 'post',
        data
    })
}
export const searchSales = (data) => {
    return request({
        url: '/search/searchSales',
        method: 'post',
        data
    })
}