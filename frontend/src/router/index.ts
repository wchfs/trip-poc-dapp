import { createRouter, createWebHistory } from 'vue-router'
import type { RouteLocationNormalized, NavigationGuardNext } from 'vue-router'
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
      name: 'dapp',
      redirect: {
        name: 'dapp.home',
      },
      children: [
        {
          path: '',
          name: 'dapp.home',
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
        {
          path: 'zones',
          children: [
            {
              path: 'my',
              name: 'dapp.zones.my',
              component: () => import('../views/Zones/My/MyZonesView.vue'),
            },
          ],
        },
        {
          path: 'proposals',
          name: 'dapp.proposals',
          redirect: {
            name: 'dapp.proposals.list',
          },
          children: [
            {
              path: 'list',
              name: 'dapp.proposals.list',
              component: () => import('../views/Proposals/List/ProposalListView.vue'),
            },
            {
              path: 'details/:id',
              name: 'dapp.proposals.details',
              component: () => import('../views/Proposals/Details/ProposalDetailsView.vue'),
              props: true,
            },
          ],
        },
      ],
    },
  ]
});

router.beforeEach(async (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
  const toName = to.name;

  if (!toName) {
    next(false);
  } else {
    const toNameString = toName.toString();

    switch (true) {
      case toNameString.startsWith('root'):
        await beforeRoot(to, from, next);
        break;
      case toNameString.startsWith('dapp'):
        await beforeDApp(to, from, next);
        break;
      default:
        next();
    }
  }
});

async function beforeRoot(to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) {
  const walletStore = useWalletStore();

  if (walletStore.connectedWallet !== null || await walletStore.tryConnectUsingPrevConnectedWallet()) {
    next({
      name: 'dapp.home',
    });
  } else {
    next();
  }
}

async function beforeDApp(to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) {
  const walletStore = useWalletStore();

  if (walletStore.connectedWallet === null && !await walletStore.tryConnectUsingPrevConnectedWallet()) {
    next({
      name: 'root',
    });
  } else {
    next();
  }
}
export default router
