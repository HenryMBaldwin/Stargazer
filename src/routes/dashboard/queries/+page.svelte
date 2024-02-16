<script lang="ts">
    import {invoke} from '@tauri-apps/api';
    import {processLog} from '$lib/logProcessor';
    import { onMount, onDestroy} from 'svelte';
	import QueryList from '$lib/components/QueryList.svelte';

    async function getQueryLogs() {
        let log: string = await invoke('get_query_log');
        processLog(log);
    }
    
    let interval: ReturnType<typeof setInterval>;
  
    onMount(() => { // check immediately on mount
        interval = setInterval(getQueryLogs, 1000); // and every !0 seconds
    });

    onDestroy(() => {
        clearInterval(interval); // cleanup the interval when the component is unmounted
    });

</script>

<div class= "full-container">
    <QueryList />
</div>

<style>

</style>