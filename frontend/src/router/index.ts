import { createRouter, createWebHistory } from 'vue-router'
import type { RouteLocationNormalized } from 'vue-router'
import { useWalletStore } from '@/stores/wallet';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'root',
      component: () => import('../views/Auth/ConnectWallet.vue')
    },
    {
      path: '/dapp',
      beforeEnter: async (to: RouteLocationNormalized, from: RouteLocationNormalized) => {
        const walletStore = useWalletStore();

        if (walletStore.onboard?.connectedWallet?.value === null) {
          await router.push({
            name: 'root',
          });
        }

        return true;
      },
      children: [
        {
          path: '',
          name: 'dapp',
          component: () => import('../views/DApp/DAppView.vue'),
        },
        {
          path: 'summary',
          name: 'dapp.summary',
          component: () => import('../views/Summary/SummaryView.vue'),
        },
        {
          path: 'tickets',
          children: [
            {
              path: 'my',
              name: 'dapp.tickets.my',
              component: () => import('../views/Tickets/My/MyTicketsView.vue'),
            },
            {
              path: 'validate',
              name: 'dapp.tickets.validate',
              component: () => import('../views/Tickets/Validate/ValidateTicketsView.vue'),
            },
          ],
        },
      ],
    },
  ]
})

export default router
