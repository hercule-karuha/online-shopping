import './assets/main.scss'
import 'animate.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import pinia from './stores/store'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import App from './App.vue'
import router from './router'
import 'element-plus/dist/index.css'
// import { getUserInfo } from './api/user'
// import request from './utils/request'

const app = createApp(App)

app.use(pinia)
app.use(router)

// 注册element-plus的图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

app.config.globalProperties.globalInfo = {
    avatarUrl: '/api/file/getAvatar/',
    imgUrl: '/api/file/getImage/',
    storeCoverUrl: '/api/file/getStoreCover/',
    productCoverUrl: '/api/file/getProductCover/',
    maxFileSize: 1024 * 1024 * 10
}
// app.config.globalProperties.$getUserInfo = getUserInfo
app.mount('#app')
