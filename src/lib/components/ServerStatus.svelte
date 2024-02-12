<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import Loader from '$lib/components/Loader.svelte'; // Import your Loader component
	import { invoke } from '@tauri-apps/api';
  
    type ServerStatusType = 'unknown' | 'online' | 'offline';
    let serverStatus: ServerStatusType = 'unknown';
  
    async function checkServerStatus(): Promise<void>  {
      serverStatus = 'unknown'; // reset to unknown before checking
      invoke('check_alive').then((res) => {
        serverStatus = res ? 'online' : 'offline';
      });
    };
  
    let interval: ReturnType<typeof setInterval>;
  
    onMount(() => {
      checkServerStatus(); // check immediately on mount
      interval = setInterval(checkServerStatus, 30000); // and every 30 seconds
    });
  
    onDestroy(() => {
      clearInterval(interval); // cleanup the interval when the component is unmounted
    });
  </script>
  
  <div class="server-status">
    Status&nbsp; <span class="gold">|</span>
    {#if serverStatus === 'unknown'}
      <div class = "loading-indicator">
        <Loader size="10px"/>
      </div>
    {:else}
      <span class="status-indicator" class:online={serverStatus === 'online'} class:offline={serverStatus === 'offline'}></span>
    {/if}
  </div>
  
  <style>
    .server-status {
      align-items: center; 
      justify-content: center;
      display: flex;
      margin-top: auto; /* Pushes to the bottom */
      padding: 10px;
      border-radius: 25px;
      border: 1px solid #D4AF37;
      font-size: 14px; /* Adjust as needed */
      line-height: 14px;
      font-family: Aleo;
    }

    .gold {
      color: #D4AF37;
      font-size:16px;
    }
    
    .status-indicator {
      height: 10px;
      width: 10px;
      border-radius: 50%;
      border: 1px solid #D4AF37;
      margin-left: 5px;
      /* Aligns the dot with the text */
      align-self: center;

    }
  
    .loading-indicator {
      display: flex;
      align-items: center;
      justify-content: center;
      margin-left: 5px;
    }
    
    .online {
      background-color: green;
    }
    
    .offline {
      background-color: red;
    }
    @font-face {
    font-family: 'Aleo';
    src: url('/fonts/Aleo-Regular.ttf') format('truetype');
    font-weight: normal;
    font-style: normal;
    }
  </style>
  