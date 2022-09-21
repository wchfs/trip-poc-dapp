import { createRouter, createWebHistory } from 'vue-router'
import type { RouteLocationNormalized } from 'vue-router'
import { useWalletStore } from '@/stores/wallet';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'root',
      redirect: {
        name: 'dapp',
      }
    },
    {
      path: '/summary',
      name: 'dapp_summary',
      component: () => import('../views/Home/HomeView.vue')
    },
    {
      path: '/dapp',
      name: 'dapp',
      beforeEnter: (to: RouteLocationNormalized, from: RouteLocationNormalized) => {
        const walletStore = useWalletStore();

        if (walletStore.onboard?.connectedWallet?.value === null) {
          console.log('before enter /dapp');
        }

        return true;
      },
      component: () => import('../views/DApp/DAppView.vue'),
      children: [

      ],
    },
  ]
})

export default router
