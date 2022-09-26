import type { ApolloClient } from 'apollo-client';
import type { NormalizedCacheObject } from 'apollo-cache-inmemory';

type Client = ApolloClient<NormalizedCacheObject>;

export abstract class ApolloService {
  private static client: Client;

  public static setClient(client: Client) {
    this.client = client;
  }

  public static getClient(): Client {
    if (!this.client) {
      throw new Error('Please initialize apollo client');
    }

    return this.client;
  }
}
