import { createPinia } from 'pinia';
import { createApp, h } from 'vue';
import App from './App.vue';
import './assets/index.css';
import { ApolloService } from '@/services/apollo-service';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { ApolloClient } from 'apollo-client';
import { createHttpLink } from 'apollo-link-http';
import router from './router';

/**
 * -------- APOLLO CLIENT SETUP --------
 */
const apolloClient = new ApolloClient({
  link: createHttpLink({
    uri: import.meta.env.VITE_APP_GRAPHQL_ENDPOINT,
  }),
  cache: new InMemoryCache(),
});
/** -------- END APOLLO CLIENT SETUP -------- */


/** -------- PINIA SETUP -------- */
const pinia = createPinia();

pinia.use(() => {
  return {
    apolloClient: apolloClient,
  };
});
/** -------- END PINIA SETUP -------- */

/** -------- APP SETUP -------- */
const app = createApp({
  setup() {
    ApolloService.setClient(apolloClient);
  },
  render: () => h(App), // @ts-ignore
});

app.use(pinia);
app.use(router);

app.mount('#app');
/** -------- END APP SETUP -------- */
