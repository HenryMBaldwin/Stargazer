<script lang="ts">
    import { queries } from '$lib/stores';
    import type { QueryState } from '$lib/types';
  
     function getColor(status: string) {
        return status === 'STARTED' ? 'lightyellow' : status === 'SUCCESS' ? 'lightgreen' : 'lightred';
    }
  
    // Optional: A function to display metadata
    function displayMetadata(metadata: Record<string, string>): string {
      // Implement logic to format and display metadata
      // For example, return a JSON string or format specific fields
      return JSON.stringify(metadata);
    }


// Function to convert record to array
  $: queryArray = $queries ? Object.values($queries) : [];
</script>

{#each queryArray as query (query.id)}
  <div class= "query-entry" style="background-color: {getColor(query.status)}">
    {query.id}
    <!-- Display metadata or other properties -->
  </div>
{/each}

<style>
  .query-entry {
    padding: 10px;
    margin: 1px;
    border-radius: 1px;
  }
</style>
  