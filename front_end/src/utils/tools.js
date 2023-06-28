import { codeToText } from 'element-china-area-data'
export const base64 = (file) => {
    return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.readAsDataURL(file)
        reader.onload = function () {
            resolve(reader.result)
        }
        reader.onerror = function (error) {
            reject(error)
        }
    })
}

export const parseAddressCodeArr = (codeArr) => {
    const province = codeToText[codeArr[0]]
    const city = codeToText[codeArr[1]]
    const area = codeToText[codeArr[2]]
    return [province, city, area]
}