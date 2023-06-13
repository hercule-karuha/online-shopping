import './assets/main.scss'
import 'animate.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import App from './App.vue'
import router from './router'
import 'element-plus/dist/index.css'
const app = createApp(App)

app.use(createPinia())
app.use(router)

// 注册element-plus的图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

app.config.globalProperties.globalInfo = {
    avatarUrl: '/api/file/getAvatar',
    uploadUrl: '/api/file/getImage',
    maxFileSize: 1024 * 1024 * 4,
}


app.mount('#app')
