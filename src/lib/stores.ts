import { writable } from 'svelte/store';
import type { QueryState, QueryEvent } from './types';

const queries = writable<Record<string, QueryState>>({});


function updateQueryState(event: QueryEvent): void {
  // Check if the query is already in the store with the same status
  queries.update((currentQueries) => {
    const existingQuery = currentQueries[event.id];

    
    // If the query doesn't exist or its status has changed
    if (!existingQuery) {
      currentQueries[event.id] = {
        id: event.id,
        timestamp: event.timestamp, //concatenate with random string to avoid duplicates
        status: event.status,
        events: [event],
      };
    } else if(existingQuery.status !== event.status && existingQuery.status === 'STARTED') {
        existingQuery.status = event.status;
        existingQuery.events.push(event);
    }
    //console.log(currentQueries);
    return currentQueries;
  });
}



export { queries, updateQueryState };
