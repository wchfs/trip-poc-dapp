import { createApp, h } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import './assets/index.css'

/** -------- FONT AWESOME SETUP -------- */
import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { fas } from '@fortawesome/free-solid-svg-icons'
import { fab } from '@fortawesome/free-brands-svg-icons';
import { far } from '@fortawesome/free-regular-svg-icons';
import { ApolloClient } from 'apollo-client';
import { createHttpLink } from 'apollo-link-http';
import { InMemoryCache } from 'apollo-cache-inmemory';

library.add(fas, far, fab);
dom.watch();
/** -------- END FONT AWESOME SETUP -------- **/


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
    // ...
  },
  render: () => h(App),
});

app.use(pinia);
app.use(router);

app.mount('#app');
/** -------- END APP SETUP -------- */
