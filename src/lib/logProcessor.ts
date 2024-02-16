import type { QueryEvent } from './types';
import { updateQueryState } from './stores';

function parseLog(log: string): QueryEvent[] {
    const cleaned_log = log.replace(/\\/g, "");
    console.log('Parsing log:', log)
    const query_events: QueryEvent[] = JSON.parse(cleaned_log); 
    console.log('Parsed log:', query_events);
    //iterate through each query event and log its id
    query_events.forEach(event => {
        console.log('Query event id:', event.id);
    });
    return query_events// Replace with appropriate parsing logic
}

function processLog(log: string): void {
  const events = parseLog(log);
  events.forEach(event => {
    updateQueryState(event);
  });
}

export { processLog };

