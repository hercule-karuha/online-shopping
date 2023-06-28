<template>
    <div class="no-data">
        <div><el-icon><MessageBox /></el-icon></div>
        <div class="msg">{{ props.msg }}</div>
    </div>
</template>

<script setup>
const props = defineProps({
    msg: {
        type: String,
        default: '空空如也'
    }
})
</script>

<style lang="scss" scoped>
.no-data{
    text-align: center;
    padding: 10px 0 ;
    .el-icon{
        font-size: 50px;
        color: #bbb;
    }
    .msg{
        margin-top: 10px;
        color: #909399;
        font-size: 16px;
    }
}

</style>
