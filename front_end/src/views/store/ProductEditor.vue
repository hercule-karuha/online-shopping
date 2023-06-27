<template>
    <main>
        <div class="content">
            <div class="header">新品</div>
            <el-form :model="formData" ref="formDataRef" :rules="rules" class="form" label-position="top" status-icon>
                <el-form-item label="请上传你的商品封面" prop="cover">
                    <CoverUpload @upload-image="uploadCover"></CoverUpload>
                </el-form-item>
                <el-form-item label="请输入你的商品名称" prop="name">
                    <el-input v-model="formData.name" type="text" clearable></el-input>
                </el-form-item>
                <el-form-item label="请输入你的商品描述" prop="description">
                    <el-input v-model="formData.description" :maxlength="200" show-word-limit type="textarea"
                        clearable></el-input>
                </el-form-item>
                <el-form-item class="detail-images-form-item" label="请上传商品详情图片">
                    <el-upload ref="uploadDetailImagesRef" class="upload-detail-images"
                        :before-remove="handleRemoveDetailImg" multiple list-type="picture-card"
                        :http-request="handleUploadDetailImg">
                        <el-icon>
                            <Plus />
                        </el-icon>
                    </el-upload>
                </el-form-item>
                <el-form-item label="请输入你的商品单价" prop="price">
                    <el-input-number size="large" v-model="formData.price" :precision="2" :step="0.1" />
                </el-form-item>
                <el-form-item label="请输入你的库存量" prop="stock">
                    <el-input-number v-model="formData.stock" size="large" />
                </el-form-item>
                <el-form-item label="请选择你的发货区县" prop="selectedRegion">
                    <el-cascader :options="options" v-model="formData.selectedRegion"></el-cascader>
                </el-form-item>
                <el-form-item prop="detailAddress" label="请输入详细地址">
                    <el-input v-model="formData.detailAddress" type="text" clearable></el-input>
                </el-form-item>
            </el-form>

            <button @click="submit">确认</button>
        </div>
    </main>
</template>

<script setup>
import CoverUpload from '@/components/CoverUpload.vue'
import { ref, onMounted } from 'vue'
import { uploadDetailImage, getImage } from '@/api/file.js'
import { newProduct } from '@/api/product'
import message from '@/utils/message'
import { useRouter } from 'vue-router'
import { parseAddressCodeArr } from '@/utils/tools'
import { regionData } from 'element-china-area-data'
const options = regionData

import { useUserInfoStore } from '@/stores/userInfo'
const router = useRouter()
const userInfoStore = useUserInfoStore()
const formDataRef = ref(null)
const formData = ref({
    detailImages: []
})
const uploadCover = (file) => {
    formData.value.cover = file
}
const newProductFlag = ref(false)
const uploadDetailImagesRef = ref(null)
// 上传详情图片
const handleUploadDetailImg = async (image) => {
    const formData4upload = new FormData()
    formData4upload.append('image', image.file)
    const res = await uploadDetailImage(formData4upload)
    if (!res) {
        message.error('上传失败')
        return Promise.reject()
    }
    formData.value.detailImages.push(res.data.path)
    // return Promise.resolve()
}
onMounted(() => {
    if (!userInfoStore.userInfo || !userInfoStore.userInfo.userType) {
        router.push('/')
    }
})
const handleRemoveDetailImg = (file, fileList) => {
    const index = fileList.indexOf(file)
    console.log(index)
    formData.value.detailImages.splice(index, 1)
}
const validateNum = (rule, value, callback) => {
    if (value < 0) {
        callback(new Error('请输入大于0的数字'))
    } else {
        callback()
    }
}
const rules = {
    name: [
        { required: true, message: '请输入商品名称', trigger: 'blur' },
        { min: 1, max: 40, message: '长度在 1 到 40 个字符', trigger: 'blur' }
    ],
    cover: [
        { required: true, message: '请上传商品封面', trigger: 'blur' }
    ],
    description: [
        { required: true, message: '请输入商品描述', trigger: 'blur' },
        { min: 1, max: 200, message: '长度在 1 到 200 个字符', trigger: 'blur' }
    ],
    price: [
        { required: true, message: '请输入商品单价', trigger: 'blur' }
    ],
    stock: [
        { required: true, message: '请输入商品库存', trigger: 'blur' },
        { validator: validateNum, trigger: 'blur' }
    ],
    selectedRegion: [
        { required: true, message: '请选择发货地址' }
    ],
    detailAddress: [
        { required: true, message: '请输入详细地址', trigger: 'change' }
    ],
}

const submit = async () => {
    if (!formDataRef.value.validate()) {
        return
    }
    const detailImages = formData.value.detailImages.join(',')
    const uploadFormData = new FormData()
    uploadFormData.append('cover', formData.value.cover)
    uploadFormData.append('name', formData.value.name)
    uploadFormData.append('description', formData.value.description)
    uploadFormData.append('detailImages', detailImages)
    uploadFormData.append('price', formData.value.price)
    uploadFormData.append('stock', formData.value.stock)
    const addressArea = parseAddressCodeArr(formData.value.selectedRegion)
    let address = {
        codeArr: formData.value.selectedRegion,
        areaArr: addressArea,
        detailAddress: formData.value.detailAddress
    }
    address = JSON.stringify(address)
    uploadFormData.append('address', address)
    const result = await newProduct(uploadFormData)
    if (!result) {
        return
    }
    router.push('/product/detail/' + result.data.productId)
}
</script>

<style scoped lang="scss">
main {
    padding-top: 60px;
    width: var(--global-width);
    margin: 0 auto;

    .content {
        border-radius: 20px;
        background-color: white;
        overflow: auto;
        padding-bottom: 40px;

        .header {
            font-size: 30px;
            padding-left: 20px;
            margin-top: 20px;
            padding-bottom: 10px;
            border-bottom: 1px solid #cfcfcf;
        }

        .form {
            margin-top: 40px;
            margin-left: 20px;

            .el-form-item {
                width: 400px;
                margin-bottom: 40px;

                :deep(.el-form-item__label) {
                    font-size: 20px;
                    margin-bottom: 20px;
                }
            }

            .detail-images-form-item {
                width: calc(var(--global-width) - 100px);
            }
        }

        button {
            margin-left: 20px;
            outline: none;
            border: none;
            background-color: var(--main-color-purple);
            border-radius: 8px;
            height: 40px;
            width: 400px;
            color: white;
            font-size: 20px;
            cursor: pointer;

            &:hover {

                background-color: var(--main-color-purple-hover);
            }
        }
    }
}
</style>