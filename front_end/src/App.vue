<template>
  <RouterView />
  <nav>
    <div title="用户中心"><el-icon><User /></el-icon></div>
    <div title="搜索"><el-icon><Search /></el-icon></div>
    <div title="返回上一级"><el-icon><ArrowLeft /></el-icon></div>
    <div title="主页"><el-icon><House /></el-icon></div>
    <div title="购物车"><el-icon><ShoppingCart /></el-icon></div>
  </nav>
</template>
<script setup>
import { RouterView } from 'vue-router'
</script>
<style lang="scss" scoped>
nav{
    position: fixed;
    bottom: 80px;
    left: 40px;
    border-radius: 50px;
    padding: 8px;
    background-color: rgb(255, 255, 255);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    div{
        margin-top: 10px;
        width: 30px;
        height: 30px;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 50%;
        &:hover{
            background-color: rgb(238, 238, 238);
            cursor: pointer;
        }
        .el-icon{
            font-size: 18px;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        &:last-child{
            margin-bottom: 10px;
        }
    }
}
</style>
