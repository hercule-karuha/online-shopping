import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('@/views/HomeView.vue')
        }, {
            path: '/account/:type',
            name: 'account',
            component: () => import('@/views/AccountView.vue')
        }, {
            path: '/product/:id',
            name: 'ProductDetail',
            component: () => import('@/views/product/ProductDetail.vue')
        }, {
            path: '/store/newStore',
            name: 'newStore',
            component: () => import('@/views/store/StoreEditor.vue')
        }, {
            path: '/store/newProduct',
            name: 'newProduct',
            component: () => import('@/views/store/ProductEditor.vue')
        }, {
            path: '/store/detail/:id',
            name: 'storeDetail',
            component: () => import('@/views/store/StoreDetail.vue')
        }, {
            path: '/store/edit/:id',
            name: 'storeEdit',
            component: () => import('@/views/store/StoreEditor.vue')
        }, {
            path: '/product/edit/:id',
            name: 'productEdit',
            component: () => import('@/views/store/ProductEditor.vue')
        }, {
            path: '/user',
            name: 'userCenter',
            component: () => import('@/views/user/UserCenter.vue'),
            children: [
                {
                    path: 'shoppingCart',
                    name: 'shoppingCart',
                    component: () => import('@/views/user/ShoppingCart.vue')
                }
            ]
        }

    ]
})

export default router
