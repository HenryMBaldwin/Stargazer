<script>
    // Any necessary script here
  </script>
  
  <style>
    .container {
      display: flex;
      height: 100vh;
      margin: 0; /* Removes default margin */
      padding: 0; /* Removes default padding */
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
  </style>
  
  <div class="container">
    <nav>
      <a href="/dashboard/home">Home</a>
      <a href="/dashboard">About</a>
      <a href="/dashboard">Contact</a>
      <!-- Add more links as necessary -->
    </nav>
  
    <main>
      <slot></slot> <!-- Content from individual pages will be rendered here -->
    </main>
  </div>
  