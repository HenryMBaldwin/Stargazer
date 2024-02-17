import type { QueryEvent } from './types';
import { updateQueryState } from './stores';

function parseLog(log: string): QueryEvent[] {
    const cleaned_log = log.replace(/\\/g, "");
    const query_events = JSON.parse(cleaned_log)
    
    return query_events// Replace with appropriate parsing logic
}

function processLog(log: string): void {
  const events = parseLog(log);

  events.forEach(event => {
    //console.log(JSON.stringify(event.metadata));
    updateQueryState(event);
  });
}

export { processLog };

