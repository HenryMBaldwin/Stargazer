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
        getQueryLogs();
        interval = setInterval(getQueryLogs, 500); // and every !0 seconds
    });

    onDestroy(() => {
        clearInterval(interval); // cleanup the interval when the component is unmounted
    });

</script>

<div class= "full-container">
    <h3> Queries </h3>
    <div class="query-list">   
        <QueryList />
    </div>
</div>

<style>
    .full-container {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 10px;
    }

    .query-list {
        flex-grow: 1;
        width: 85%;
        display: flex;
        padding: 5px;
        flex-direction: column;
        background-color: #fafafa;
        border-radius: 5px;
        border: 1px solid #D4AF37;
    }
</style>