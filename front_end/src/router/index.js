import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('../views/HomeView.vue')
        },{
            path: '/account/:type',
            name: 'account',
            component: () => import('../views/AccountView.vue')
        }
    ]
})

export default router
