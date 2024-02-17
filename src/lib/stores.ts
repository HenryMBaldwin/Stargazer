import { writable } from 'svelte/store';
import type { QueryState, QueryEvent } from './types';

const queries = writable<Record<string, QueryState>>({});


function updateQueryState(event: QueryEvent): void {
  // Check if the query is already in the store with the same status
  queries.update((currentQueries) => {
    const existingQuery = currentQueries[event.id];

    // If the query doesn't exist or its status has changed
    if (!existingQuery || existingQuery.status !== event.status) {
      // Use query.metadata if query exists, otherwise use event.metadata

      const metadata = existingQuery ? existingQuery.metadata : event.metadata;
      
      currentQueries[event.id] = {
        id: event.id,
        status: event.status,
        metadata: metadata
      };
    }

    return currentQueries;
  });
}

export { queries, updateQueryState };
