<script>
    import ServerStatus from "$lib/components/ServerStatus.svelte";
    import NavEntry from "$lib/components/NavEntry.svelte";
</script>
  
  <div class="container">
    <nav class="nav-container">
    <div class="nav-header">
        <img class="header-img" src="/assets/telescope.svg" alt="Telescope">
        Stargazer</div>
      <NavEntry path="/dashboard/home" name="Home" top={true}/>
      <NavEntry path="/dashboard/scheduler" name="Scheduler" />
      <NavEntry path="/dashboard/settings" name="Settings"/>
      
      <ServerStatus />
      <!-- Add more links as necessary -->
    </nav>
  
    <main>
      <slot></slot> <!-- Content from individual pages will be rendered here -->
    </main>
  </div>
  
  <style>
    
    .container {
      display: flex;
      height: 100vh;
      margin: 0; /* Removes default margin */
      padding: 0; /* Removes default padding */
       
    }
    
    .nav-container {
        border-right: 1px solid #D4AF37;
        padding: 20px 0;
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
  
    a {
      color: black;
      text-decoration: none;
      margin-bottom: 10px;
    }
  
    a:hover {
      text-decoration: underline;
    }
  
    /* Add this to ensure no space around the layout */
    body {
      margin: 0;
      padding: 0;
    }

    @font-face {
    font-family: 'Aleo';
    src: url('/fonts/Aleo-Regular.ttf') format('truetype');
    font-weight: normal;
    font-style: normal;
}
  </style>