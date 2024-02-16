import { writable } from 'svelte/store';
import type { QueryState, QueryEvent } from './types';

const queries = writable<Record<string, QueryState>>({});

function updateQueryState(event: QueryEvent): void {
  queries.update(currentQueries => {
    const { id, status, metadata } = event;
    currentQueries[id] = { ...currentQueries[id], status, metadata };
    return currentQueries;
  });
}

export { queries, updateQueryState };
