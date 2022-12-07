import { createRouter, createWebHistory } from 'vue-router'
import type { RouteLocationNormalized, NavigationGuardNext } from 'vue-router'
import { useWalletStore } from '@/stores/wallet';
import LandingView from '@/views/Landing/LandingView.vue';
import AppView from '@/views/App/AppView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'root',
      redirect: {
        name: 'home',
      },
    },
    {
      path: '/home',
      component: LandingView,
      children: [
        {
          path: '',
          name: 'home',
          component: () => import('../views/Landing/Home/HomeView.vue'),
        },
      ],
    },
    {
      path: '/dapp',
      name: 'dapp',
      component: AppView,
      redirect: {
        name: 'dapp.home',
      },
      children: [
        {
          path: 'home',
          name: 'dapp.home',
          component: () => import('../views/App/DApp/DAppView.vue'),
        },
        {
          path: 'connect',
          name: 'dapp.connect',
          component: () => import('../views/App/Auth/ConnectWallet.vue'),
        },
        {
          path: 'tickets',
          children: [
            {
              path: 'my',
              name: 'dapp.tickets.my',
              component: () => import('../views/App/Tickets/My/MyTicketsView.vue'),
            },
            {
              path: 'validate',
              name: 'dapp.tickets.validate',
              component: () => import('../views/App/Tickets/Validate/ValidateTicketsView.vue'),
            },
          ],
        },
        {
          path: 'zones',
          children: [
            {
              path: 'my',
              name: 'dapp.zones.my',
              component: () => import('../views/App/Zones/My/MyZonesView.vue'),
            },
            {
              path: 'details/:zoneId(\\d+)',
              name: 'dapp.zones.my.details',
              component: () => import('../views/App/Zones/Details/ZoneDetailsView.vue'),
              props: true,
            },
          ],
        },
        {
          path: 'proposals',
          name: 'dapp.proposals',
          redirect: {
            name: 'dapp.proposals.list',
          },
          beforeEnter: (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
            next({
              name: 'root',
            });
          },
          children: [
            {
              path: 'list',
              name: 'dapp.proposals.list',
              component: () => import('../views/App/Proposals/List/ProposalListView.vue'),
            },
            {
              path: 'details/:id(\\d+)',
              name: 'dapp.proposals.details',
              component: () => import('../views/App/Proposals/Details/ProposalDetailsView.vue'),
              props: true,
            },
          ],
        },
        {
          path: 'vouchers',
          name: 'dapp.vouchers',
          redirect: {
            name: 'dapp.vouchers.list',
          },
          children: [
            {
              path: 'list',
              name: 'dapp.vouchers.list',
              component: () => import('../views/App/Vouchers/List/VoucherListView.vue'),
            }
          ]
        }
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
      case toNameString === 'dapp.connect':
        await beforeRoot(to, from, next);
        break;
      case toNameString.startsWith('dapp') && toNameString !== 'dapp.connect':
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
      name: 'dapp.connect',
    });
  } else {
    next();
  }
}
export default router
