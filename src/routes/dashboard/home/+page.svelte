<div class="w-full flex flex-col items-center py-8 px-10 gap-8">
    <img class="max-w-[80%] w-80" src="/assets/stargazer_black_vert_transparent.png" alt="Logo">
    <div class="flex flex-col items-top gap-4 max-w-prose-sm">
        <h2 class="h2">Welcome to Stargazer!</h2>
        <p> Stargazer is your Excel companion for Orion. Its primary and most basic functionality is pulling Orion query data directly
             into Excel via Excel formulas. It also boasts a multi level caching system, a query scheduler, an Excel file scheduler, as
              well as useful tools for debugging and building sheets to work with Stargazer. </p>
        <h3 class="h3">Understanding this App</h3>
        <p>The Stargazer client app </p>      
        <h3 class="h3">Getting Started</h3>
        <p> Make sure you have downloaded and installed the Stargazer Excel Add-in. You can verify it's active either
             by typing <code class="code">=Stargazer</code> in a cell in Excel, and confirming formulas in the Stargazer namespace appear, or by looking for it
            in the Excel menu ribbon.</p>
        <h3 class="h3">Basic Formulas</h3>    
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