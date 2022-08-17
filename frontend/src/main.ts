import { createApp, h } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia';
import { createHttpLink } from 'apollo-link-http';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { ApolloClient } from 'apollo-client';


const apolloClient = new ApolloClient({
    link: createHttpLink({
        uri: process.env.VUE_APP_GRAPHQL_ENDPOINT,
    }),
    cache: new InMemoryCache(),
});

const app = createApp({
    setup() {},
    render: () => h(App),
});

const pinia = createPinia();

pinia.use(() => {
    return {
        apolloClient: apolloClient,
    };
});

app
    .use(pinia)
    .use(router)
    .mount('#app');
