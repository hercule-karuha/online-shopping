import request from '@/utils/request'
export const newProduct = (data) => {
    return request({
        url: '/product/newProduct',
        method: 'post',
        data
    })
}
export const getProduct = (id) => {
    return request({
        url: '/product/getProduct/' + id,
        method: 'get'
    })
}

export const getRecommend = (data) => {
    return request({
        url: '/product/getRecommend',
        method: 'post',
        data
    })
}