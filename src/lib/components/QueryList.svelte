<script lang="ts">
  import { queries } from '$lib/stores';
  import type { ErrorMetadata, QueryEvent, QueryState, StartedMetadata, SuccessMetadata } from '$lib/types';
  import { flip } from 'svelte/animate';
  import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
  import { faCaretRight, faCaretDown } from '@fortawesome/free-solid-svg-icons';

  let detailsVisible: Record<string, boolean> = {};

  function getColor(status: string) {
      return status === 'STARTED' ? 'lightyellow' : status === 'SUCCESS' ? 'lightgreen' : 'pink';
  }

  function displayMetadata(metadata: StartedMetadata | string): string {
    // Your display metadata logic
    return JSON.stringify(metadata);
  }

  function getMetadata(metadata: StartedMetadata | ErrorMetadata | SuccessMetadata): StartedMetadata {
    if (typeof metadata === 'object' && 'id' in metadata) {
      return metadata
    } else {
      //return empty started metadata;
      return {id: '', args: []};
    }
  }

  function getQueryId(query: QueryState): string {
    let ret = "";
    query.events.forEach(event => {
      if (event.status === 'STARTED') {
        ret = getMetadata(event.metadata).id;
      }
    });
    return ret;
  }

  function getEvents(query: QueryState): QueryEvent[] {
    return query.events;
  }

  function getTimestamp(timestamp: string): string {
    //get only the first 8 characters of the timestamp to cut off random id
    return timestamp.substring(0, 8);
  }

  $: queryArray = $queries ? Object.values($queries).sort((a, b) => a.timestamp.localeCompare(b.timestamp)) : [];
  $: queryArray.forEach(query => {
    if (!(query.id in detailsVisible)) {
      detailsVisible[query.id] = false;
    }
  });

  function toggleDetails(id: string) {
    detailsVisible[id] = !detailsVisible[id];
  }
</script>

{#each queryArray as query (query.timestamp)}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="query-entry" style="background-color: {getColor(query.status)}" on:click={() => toggleDetails(query.id)}>
    {#if detailsVisible[query.id]}
      <FontAwesomeIcon icon={faCaretDown} />
    {:else}
      <FontAwesomeIcon icon={faCaretRight} />
    {/if}
    <span class="query-text">[{getTimestamp(query.timestamp)}]</span>
    <span class="query-text">{getQueryId(query)}</span>
    
    
  </div>
  {#if detailsVisible[query.id]}
    <div class="query-details">
      {#each getEvents(query) as event (event.timestamp)}
        <div class="query-detail">{event.timestamp}: {event.status}</div>
      {/each}
    </div>
  {/if}
{/each}


<style>
.query-entry {
  padding: 5px;
  margin-top: 1px;
  border-radius: 1px;
  height: auto;
  display: flex;
  align-items: center;
}

.query-text {
  font-size: 14px;
  padding-left: 5px;
}

.query-details {
  margin-top: 0px;
  padding: 10px;
  background-color: #f0f0f0;
  border: 1px solid #050505;
  border-top: 0;
  display: flex;
  flex-direction: column;
  

}

.query-detail {
  font-size: 12px;
  margin: 1px;
  padding: 1px;
  background-color: #f0f0f0;
}

.query-event-entry {
  padding: 5px;
  margin-top: 1px;
  border-radius: 1px;
  height: auto;
  display: flex;
  align-items: center;
}
</style>
