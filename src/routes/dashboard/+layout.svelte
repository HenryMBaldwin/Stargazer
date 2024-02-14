<script>
    import ServerStatus from "$lib/components/ServerStatus.svelte";
    import NavEntry from "$lib/components/NavEntry.svelte";
    import { onMount } from "svelte";
    import { appWindow } from "@tauri-apps/api/window";
	import { app } from "@tauri-apps/api";

    onMount(() => {
        appWindow.setResizable(true);
        appWindow.setMaximizable(true);
    });
</script>
  
  <div class="full-container">
    <nav class="nav-container">
    <div class="nav-header">
        <img class="header-img" src="/assets/telescope.svg" alt="Telescope">
        Stargazer</div>
      <NavEntry path="/dashboard/home" name="Home" top={true}/>
      <NavEntry path="/dashboard/queries" name="Queries" />
      <NavEntry path="/dashboard/scheduler" name="Scheduler" />
      <NavEntry path="/dashboard/settings" name="Settings"/>
      
      <ServerStatus />
      <!-- Add more links as necessary -->
    </nav>
  
    <div class=page-container>
      <slot></slot> <!-- Content from individual pages will be rendered here -->
    </div>
  </div>
  
  <style>
    .page-container {
      overflow-y: auto;
      width: 100%;
    }
    .full-container {
      display: flex;
      height: 100vh;
      width: 100vw;
      margin: 0; /* Removes default margin */
      padding: 0; /* Removes default padding */
       
    }
    
    .nav-container {
        border-right: 1px solid #D4AF37;
        padding: 20px 0;
        width: 125px;
        min-width: 125px;
        height: 100vh
    }
    .header-img {
      width: 55px;
    }
    .nav-header {
      display: flex;
      flex-direction: column;
      font-size: 22px;
      font-weight: bold;
      margin-bottom: 20px;
      font-family: "Aleo", sans-serif;
      text-decoration: underline;
      color:#050505;
      align-items: center;
    }
    nav {
      background-color: #fafafa; /* slightly off-white color */
      width: 125px; /* adjust the width as needed */
      display: flex;
      flex-direction: column;
      justify-content: start;
      align-items: center;
      padding: 20px;
    }
  
    main {
      flex-grow: 1;
      padding: 20px;
    }

    @font-face {
    font-family: 'Aleo';
    src: url('/fonts/Aleo-Regular.ttf') format('truetype');
    font-weight: normal;
    font-style: normal;
}
  </style>