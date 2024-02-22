import type { QueryEvent } from './types';
import { updateQueryState } from './stores';

let count = 0;

function parseLog(log: string): QueryEvent[] {
    const cleaned_log = log.replace(/\\/g, "");
    const query_events = JSON.parse(cleaned_log);
    return query_events
}

function processLog(log: string): void {
  const events = parseLog(log);

  events.forEach(event => {
    //console.log(JSON.stringify(event.metadata));
    event.timestamp = event.timestamp + incrementString();
    updateQueryState(event);
  });
}

//make sure no 2 events have the same timestamp
function incrementString() {
  return String(count++);
}

export { processLog };

