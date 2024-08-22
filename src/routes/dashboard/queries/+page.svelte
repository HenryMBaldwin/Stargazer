<script lang="ts">
    import {invoke} from '@tauri-apps/api/core';
    import {processLog} from '$lib/logProcessor';
    import { onMount, onDestroy} from 'svelte';
    import QueryList from '$lib/components/QueryList.svelte';
    import { queries } from '$lib/stores';

    let queryListContainer: HTMLDivElement;
    let log_length: number = 0;
    async function getQueryLogs() {
        let log: string = await invoke('get_query_log');
        if (log.length > log_length) {
            log_length = log.length;
            processLog(log);
        }
    }

    async function scrollToBottom() {
        //wait a bit for the new query to be added to the list
        await new Promise(r => setTimeout(r, 100));
        if (queryListContainer) {
            queryListContainer.scrollTop = queryListContainer.scrollHeight;
        }
    }
    
    let interval: ReturnType<typeof setInterval>;
  
    onMount(() => {
        getQueryLogs();
        interval = setInterval(getQueryLogs, 500); // Check every 500 milliseconds
    });

    onDestroy(() => {
        clearInterval(interval);
    });

    $: $queries, scrollToBottom();
    
</script>

<div class="full-container">
    <h3> Queries </h3>
    <div class="query-list" bind:this={queryListContainer}>   
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
        overflow-y: auto;
    }
</style>