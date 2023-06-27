<template>
  <div>
    <el-dialog
    class="dialog-container"
    :model-value="props.show"
    :show-close="props.showClose"
    :close-on-click-modal="false"
    :title="props.title"
    :width="props.width"
    :lock-scroll="false"
    :top="props.top"
    @close="close"
  >
    <div class="dialog-body">
        <!-- 这里是内容 -->
        <slot></slot>

    </div>
    <template v-if="buttons&&buttons.length>0 || showCancel" >
        <div class="dialog-footer">
            <el-button type="warning" @click="close" v-if="showCancel">取消</el-button>
            <el-button v-for="(btn,index) in buttons" :key="index" :type="btn.type" @click="btn.click">
                {{ btn.text }}
            </el-button>
        </div>
    </template>
    </el-dialog>
  </div>
</template>

<script setup>
const props = defineProps({
    show: {
        type: Boolean,
        default: true
    },
    title: {
        type: String,
        default: ''
    },
    showClose: {
        type: Boolean,
        default: true
    },
    width: {
        type: String,
        default: '30%'
    },
    top: {
        type: String,
        default: '100px'
    },
    buttons: {
        type: Array
    },
    showCancel: {
        type: Boolean,
        default: true
    }
})
const emits = defineEmits(['close'])
const close = () => {
    emits('close')
}
</script>

<style scoped lang="scss" >
// 深度选择器修改el-dialog样式
:deep(.el-dialog){
    border-radius: 14px;
}
.dialog-container{
    text-align: center;
    border-radius: 10px;
    overflow: hidden;
    .dialog-body{
        min-height: 80px;
        max-height: calc(100vh - 300px);
        overflow-y: auto;
    }
    .dialog-footer{
        margin-top: 20px;
        text-align: right;
    }
}

</style>
