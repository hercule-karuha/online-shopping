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
        url: '/user/userInfo',
        method: 'get',
        params
    })
}
