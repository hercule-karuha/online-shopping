<template>
    <main>
        <header>
            <div class="info">
                <div class="avatar">
                    <img src="https://pic.imgdb.cn/item/64475aac0d2dde5777850410.webp" alt="">
                </div>
                <div class="name">
                    <span>XXXXXX</span>
                    <span><el-icon><Male /></el-icon></span>
                </div>
                <span class="edit"><el-icon><Edit /></el-icon>编辑个人信息</span>
            </div>
            <nav>
                <ul class="tabs">
                    <li><router-link to="/user/shoppingCart" class="active_tab">购物车</router-link></li>
                    <li><a href="">购买记录</a></li>
                    <li><a href="">买过的店</a></li>
                    <li><a href="">我的店铺</a></li>
                </ul>
            </nav>
        </header>
        <div class="content">
            <router-view></router-view>
        </div>
    </main>
</template>

<script setup>

</script>

<style scoped lang="scss">
main{
    width: var(--global-width);
    margin: 0 auto;
    header {
        margin-top: 40px;
        background-color: var(--main-color-purple);
        height: 160px;
        border-radius: 20px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        overflow: hidden;
        .info{
            display: flex;
            margin-left: 40px;
            margin-top: 40px;
            flex: 1;
            color: white;
            align-items: start;
            .avatar{
                width: 60px;
                height: 60px;
                border-radius: 50%;
                overflow: hidden;
                cursor: pointer;
                transition: all 0.4s;
                img{
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
                &:hover{
                    transform: rotate(180deg);
                }
            }
            .name{
                margin-left: 20px;
                font-size: 24px;
                display: flex;
                flex-direction: column;
                line-height: auto !important;
                span{
                    margin-top: 0;
                    .el-icon{
                        font-size: 20px;
                    }
                    .male{
                        color: rgb(143, 178, 255);
                    }
                    .female{
                        color: rgb(255, 143, 143);
                    }
                }
            }
            .edit{
                margin: 6px 30px;
                font-size: 18px;
                display: flex;
                align-items: center;
                cursor: pointer;
                &:hover{
                    color: rgb(240, 255, 211);
                    text-decoration: underline;
                }
            }
        }
        nav{
            justify-self: flex-end;
            display: flex;
            justify-content: center;
            ul {
                list-style-type: none;
                display: flex;
                li{
                    margin-bottom: 5px;
                    a{
                        text-decoration: none;
                        padding: 10px;
                        color: white;
                        font-size: 18px;
                        border-radius: 10px 10px 0 0;
                    }
                    .active_tab{
                        background-color: white;
                        color: var(--main-color-purple);
                    }
                }
            }
            
        }
    }
    .content{
        margin-top: 40px;
    }
}
</style>