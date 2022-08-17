import 'pinia';
import { ApolloClient } from 'apollo-client';
import { NormalizedCacheObject } from 'apollo-cache-inmemory';

declare module 'pinia' {
    export interface PiniaCustomProperties {
        apolloClient: ApolloClient<NormalizedCacheObject>,
    }
}
