import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia';
import { createHttpLink } from 'apollo-link-http';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { ApolloClient } from 'apollo-client';
import VueApollo from 'vue-apollo';


const apolloProvider = new VueApollo({
    defaultClient: new ApolloClient({
        link: createHttpLink({
            uri: process.env.VUE_APP_GRAPHQL_ENDPOINT,
        }),
        cache: new InMemoryCache(),
    }),
});

const app = createApp(App)
    .use(createPinia())
    .use(router)
    .use(apolloProvider)
    .mount('#app');
