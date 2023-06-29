import request from '@/utils/request'

export const login = (data) => {
    return request({
        url: '/user/login',
        method: 'post',
        data
    })
}

export const register = (data) => {
    return request({
        url: '/user/register',
        method: 'post',
        data
    })
}

export const getUserInfo = (params) => {
    return request({
        url: '/user/getUserInfo',
        method: 'get',
        params
    })
}

export const getShoppingCart = (data) => {
    return request({
        url: '/user/getShoppingCart',
        method: 'post',
        data
    })
}

export const getDetailUserInfo = () => {
    return request({
        url: '/user/getDetailUserInfo',
        method: 'get'
    })
}

export const editUserInfo = (data) => {
    return request({
        url: '/user/editUserInfo',
        method: 'post',
        data
    })
}

export const getOrderList = (data) => {
    return request({
        url: '/user/getOrderList',
        method: 'post',
        data
    })
}

export const logout = () => {
    return request({
        url: '/user/logout',
        method: 'post'
    })
}