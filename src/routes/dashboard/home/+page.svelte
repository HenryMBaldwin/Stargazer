<div class=page-container>
    <img class="logo" src="/assets/stargazer_black_vert_transparent.png" alt="Logo">
    <div class="text-container">
        <h3>Welcome to Stargazer!</h3>
        <p> Stargazer is your Orion companion for Excel. Its primary and most basic functionality is pulling Orion query data directly
             into Excel via Excel formulas. It also boasts a multi level caching system, a query scheduler, an Excel file scheduler, as
              well as useful tools for debugging and building sheets to work with Stargazer. </p>
        <h4>Understanding this App</h4>
        <p>The Stargazer client app </p>      
        <h4>Getting Started</h4>
        <p> Make sure you have downloaded and installed the Stargazer Excel Add-in. You can verify it's active either
             by typing <code>=Stargazer</code> in a cell in Excel, and confirming formulas in the Stargazer namespace appear, or by looking for it
            in the Excel menu ribbon.</p>
        <h4>Basic Formulas</h4>    
    </div>
</div>

<style>
    .page-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        padding: 0;
        margin: 0;
    }
    .logo {
        width: 300px;
        max-width: 80%;
        height: auto;
        margin: 20px;
    }
    .text-container{
        flex-grow: 1;
        display: flex;
        flex-direction: column;
        align-items: top;
        justify-content: left;
        width: 80%;
    }
</style>