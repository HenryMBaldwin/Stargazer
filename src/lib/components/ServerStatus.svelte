<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import Loader from '$lib/components/Loader.svelte'; // Import your Loader component
    import { invoke } from '@tauri-apps/api/core';

    type ServerStatusType = 'unknown' | 'online' | 'offline';
    type AuthStatus = 'unknown' | 'authorized' | 'unauthorized';
    type Status = 'unknown' | 'unauthorized' | 'online' | 'offline';

    let serverStatus: ServerStatusType = 'unknown';
    let authStatus: AuthStatus = 'unknown';
    let status: Status = 'unknown';

    async function checkStatus(): Promise<void>  {
      status = 'unknown'; // reset to unknown before checking
      //determine individual statuses
      await checkServerStatus();
      if (serverStatus === 'online') {
        await checkAuthStatus();
      }

      //determine overall status
      if (serverStatus === 'online'){
        if (authStatus === 'authorized'){
          status = 'online';
        }
        else {
          status = 'unauthorized';
        }
      }
      else {
        status = 'offline';
      }
    }
    async function checkServerStatus(): Promise<void>  {
      serverStatus = 'unknown'; // reset to unknown before checking
      await invoke('check_alive').then((res) => {
        console.log('Server status: ' + res);
        serverStatus = res ? 'online' : 'offline';
      });
    };
    
    async function checkAuthStatus(): Promise<void>  {
      authStatus = 'unknown'; // reset to unknown before checking
      await invoke('check_auth').then((res) => {
        authStatus = res ? 'authorized' : 'unauthorized';
      });
    };


    let interval: ReturnType<typeof setInterval>;
  
    onMount(() => {
      checkStatus(); // check immediately on mount
      interval = setInterval(checkStatus, 10000); // and every 10 second
    });
  
    onDestroy(() => {
      clearInterval(interval); // cleanup the interval when the component is unmounted
    });
  </script>
  
  
  <div class="server-status">
    Status&nbsp; <span class="gold">|</span>
    {#if status === 'unknown'}
      <div class = "loading-indicator">
        <Loader size="10px"/>
      </div>
    {:else}
      <span class="status-indicator" class:online={status === 'online'} class:unauthorized={status === 'unauthorized'} class:offline={status === 'offline'}></span>
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

    .unauthorized {
      background-color: orange;
    }
    @font-face {
    font-family: 'Aleo';
    src: url('/fonts/Aleo-Regular.ttf') format('truetype');
    font-weight: normal;
    font-style: normal;
    }
  </style>
  