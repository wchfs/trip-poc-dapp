import { defineStore } from 'pinia';
import injectedModule from '@web3-onboard/injected-wallets';
import { init, useOnboard } from '@web3-onboard/vue';
import type { OnboardComposable } from '@web3-onboard/vue/dist/types';
import type { OnboardAPI, WalletState } from '@web3-onboard/core';
import { useLocalStorage } from '@vueuse/core';
import type { ConnectOptions } from '@web3-onboard/core';

export const useWalletStore = () => {
  const innerStore = defineStore({
    id: 'wallet',
    state: () => {
      return {
        lastConnectedWallet: useLocalStorage<string | null>('lastConnectedWallet', null, {
          serializer: {
            read(raw: string): string | null {
              return JSON.parse(raw);
            },
            write(value: string | null): string {
              return JSON.stringify(value);
            }
          },
        }),
        onboardAPI: null as OnboardAPI | null,
        connectingWallet: false,
      }
    },
    getters: {
      onboard: (state): OnboardComposable | null => {
        if (state.onboardAPI === null) {
          return null;
        }

        try {
          return useOnboard();
        } catch (e) {
          return null;
        }
      },
      connectedWallet(): WalletState | null {
        const connectedWallet = this.onboard?.connectedWallet;

        return connectedWallet ? connectedWallet.value : null;
      },
      walletAddress(): string|null {
        const accounts = this.connectedWallet?.accounts;

        if (!accounts) {
          return null;
        }

        return accounts[0].address;
      },
    },
    actions: {
      init(forceFresh: boolean = false): void {
        if (!forceFresh && this.onboardAPI !== null) {
          return;
        }

        this.onboardAPI = init({
          wallets: [injectedModule()],
          chains: [
            {
              id: "0x7a69",
              token: "ETH",
              label: "localhost",
              rpcUrl: "http://localhost:8545",
            },
            // currently only on localhost
          ],
          appMetadata: {
            name: "Cartesi Rollups Boilerplate DApp",
            icon: "<svg class=\"w-6 h-6 fill-current\" width=\"24\" height=\"24.818\" viewBox=\"0 0 24 24.818\"><path id=\"Path_141\" data-name=\"Path 141\" d=\"M49.05,155.1a.487.487,0,0,0-.456-.316H43.917l-2.8-7.473a.487.487,0,0,0-.815-.157l-6.963,7.631H28.488a.486.486,0,0,0-.456.657l2.921,7.79a.487.487,0,0,0,.456.316h4.61l3.1,8.266,4.674-5.114,2.873-3.152h4.848a.486.486,0,0,0,.456-.657Zm-8.567-6.7,2.523,6.727-2.822,3.072-1.161-3.1a.487.487,0,0,0-.456-.316H34.655Zm-11.292,7.36h9.038L39.444,159l-3.283,3.573H31.747ZM43.074,166.04l-3.6,3.941-2.55-6.8,2.9-3.161,1.2,3.209a.487.487,0,0,0,.456.316H45.35Zm-1.251-3.469-1.258-3.355,3.179-3.461h4.512l2.556,6.816Z\" transform=\"translate(-28.001 -146.993)\"></path></svg>",
            description: "Vue boilerplate for Cartesi Rollups",
            recommendedInjectedWallets: [
              {
                name: "MetaMask",
                url: "https://metamask.io",
              },
              {
                name: "Coinbase",
                url: "https://wallet.coinbase.com/",
              },
            ],
          },
          accountCenter: {
            desktop: {
              enabled: true,
              position: 'topRight',
              minimal: true,
            },
            mobile: {
              enabled: true,
              position: 'topRight',
              minimal: true,
            },
          },
          connect: {
            showSidebar: false,
          }
        });
      },
      async connectWallet(): Promise<void> {
        if (this.onboard === null) {
          throw new Error('Init onboard first');
        }

        if (this.connectingWallet) {
          console.log('Already connecting wallet');

          return;
        }

        this.connectingWallet = true;

        let options = undefined as ConnectOptions | undefined;

        if (this.lastConnectedWallet !== null) {
          options = {
            autoSelect: {
              label: this.lastConnectedWallet,
              disableModals: true,
            },
          };
        }

        const connectedWallet = await this.onboard.connectWallet(options);

        this.connectingWallet = false;

        return connectedWallet;
      },
      async tryConnectUsingPrevConnectedWallet(): Promise<boolean> {
        if (this.lastConnectedWallet === null) {
          return false;
        }

        await this.connectWallet();

        return this.connectedWallet !== null;
      },
      setLastConnectedWallet(label: string | null): void {
        if (label === null) {
          return;
        }

        this.lastConnectedWallet = label;
      },
      clearLastConnectedWallet(): void {
        this.lastConnectedWallet = null;
      },
    },
  });

  const store = innerStore();

  store.init();

  return store;
};


