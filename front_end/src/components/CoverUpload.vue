<template>
    <el-upload
    class="avatar-uploader"
    :show-file-list="false"
    :before-upload="beforeAvatarUpload"
  >
    <img v-if="imageUrl" :src="imageUrl" class="avatar" />
    <el-icon v-else class="avatar-uploader-icon"><Plus /></el-icon>
  </el-upload>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import message from '@/utils/message'
import { Plus } from '@element-plus/icons-vue'
import { base64 } from '@/utils/tools'
import { getCurrentInstance } from 'vue'
const { proxy } = getCurrentInstance()
const props = defineProps({
    imageUrl: {
        type: String,
        default: ''
    },
    update: {
        type: Boolean,
        default: false
    }
})

const imageUrl = ref('')
// onMounted(() => {
//     if (props.update) {
//         imageUrl.value = props.imageUrl
//     }
// })
watch(() => props.imageUrl, (val) => {
    imageUrl.value = val
}, { immediate: true })


const emit = defineEmits([
    'uploadImage'
])
const beforeAvatarUpload = async (file) => {
    console.log(file)
    if (file.size > proxy.globalInfo.maxFileSize) {
        message.error('文件大小超过限制')
        return
    }
    await base64(file).then(res => {
        imageUrl.value = res
    })
    emit('uploadImage', file)
}
</script>

<style lang="scss" scoped>
.avatar-uploader {
    width: 200px;
    height: 200px;
    overflow: hidden;
    display: inline-block;
    border-radius: 10px;
    border: #8c939d dashed 1px;
    transition: all 0.3s;
    display: flex;
    justify-content: center;
    .el-upload {
        background-color: gray;
    }
    &:hover{
        border: var(--main-color-purple) dashed 1px;
    }
    .avatar-uploader-icon {
        font-size: 40px;
        color: #8c939d;
    }
    img{
        width: 100%;
        height: 100%;
        object-fit: cover;
    }
    :deep(.el-upload--text){
        width: 100%;
        height: 100%;
    }
}

</style>