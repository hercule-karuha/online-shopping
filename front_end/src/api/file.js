import request from '@/utils/request'
export const uploadDetailImage = async (formData) => {
    return await request({
        url: '/file/uploadImage',
        method: 'post',
        data: formData
    })
}
export const getImage = (path) => {
    return '/api/file/getImage/' + path
}