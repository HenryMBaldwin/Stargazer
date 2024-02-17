<script lang="ts">
  import { queries } from '$lib/stores';
  import type { QueryState, StartedMetadata } from '$lib/types';
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

  function getMetadata(metadata: StartedMetadata | string): StartedMetadata {
    if (typeof metadata === 'object' && 'id' in metadata) {
      return metadata
    } else {
      //return empty started metadata;
      return {id: '', args: []};
    }
  }

  $: queryArray = $queries ? Object.values($queries) : [];
  $: queryArray.forEach(query => {
    if (!(query.id in detailsVisible)) {
      detailsVisible[query.id] = false;
    }
  });

  function toggleDetails(id: string) {
    detailsVisible[id] = !detailsVisible[id];
  }
</script>

{#each queryArray as query (query.id)}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="query-entry" style="background-color: {getColor(query.status)}" on:click={() => toggleDetails(query.id)}>
    {#if detailsVisible[query.id]}
      <FontAwesomeIcon icon={faCaretDown} />
    {:else}
      <FontAwesomeIcon icon={faCaretRight} />
    {/if}
    <span class="query-text">{getMetadata(query.metadata).id}</span>
    
  </div>
  {#if detailsVisible[query.id]}
    <div class="query-details">
      {#each getMetadata(query.metadata).args as arg}
        <span class="query-detail">{arg}</span>
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
</style>
