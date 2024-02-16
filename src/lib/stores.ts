import { writable } from 'svelte/store';
import type { QueryState, QueryEvent } from './types';

const queries = writable<Record<string, QueryState>>({});


function updateQueryState(event: QueryEvent): void {
  //check and see if the query is already in the store with the same status
  
  queries.update((currentQueries) => {
    const query = currentQueries[event.id];
    if (!query || query.status !== event.status) {
      currentQueries[event.id] = {
        id: event.id,
        status: event.status,
        metadata: event.metadata,
      };
    }
    return currentQueries;
  });
}

export { queries, updateQueryState };
