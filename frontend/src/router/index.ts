import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dapp_summary',
      component: () => import('../views/Home/HomeView.vue')
    },
    {
      path: '/dapp',
      name: 'dapp',
      component: () => import('../views/DApp/DAppView.vue')
    },
  ]
})

export default router
