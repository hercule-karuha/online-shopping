import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'

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
            path: '/product/detail/:id',
            name: 'ProductDetail',
            component: () => import('@/views/product/ProductDetail.vue')
        }, 
        {
            path: '/store/detail/:id',
            name: 'StoreDetail',
            component: () => import('@/views/store/StoreDetail.vue')
        },{
            path: '/store/newStore',
            name: 'newStore',
            component: () => import('@/views/store/StoreEditor.vue')
        }, {
            path: '/store/edit',
            name: 'storeEdit',
            component: () => import('@/views/store/StoreEditor.vue')
        },
            {
            path: '/store/newProduct',
            name: 'newProduct',
            component: () => import('@/views/store/ProductEditor.vue')
        }, {
            path: '/product/edit/:id',
            name: 'productEdit',
            component: () => import('@/views/store/ProductEditor.vue')
        },
        {
            path: '/product/list',
            name: 'productList',
            component: () => import('@/views/product/ProductList.vue')
        }, {
            path: '/user',
            name: 'userCenter',
            redirect: '/user/shoppingCart',
            component: () => import('@/views/user/UserCenter.vue'),
            children: [
                {
                    path: 'shoppingCart',
                    name: 'shoppingCart',
                    component: () => import('@/views/user/ShoppingCart.vue')
                }, {
                    path: 'order',
                    name: 'order',
                    component: () => import('@/views/user/OrderList.vue')
                }, {
                    path: 'stores',
                    name: 'stores',
                    component: () => import('@/views/user/StoreList.vue')
                }
            ]
        }, {
            path: '/purchase',
            name: 'purchase',
            component: () => import('@/views/purchase/PurchaseView.vue')
        }
    

    ]
})

export default router
