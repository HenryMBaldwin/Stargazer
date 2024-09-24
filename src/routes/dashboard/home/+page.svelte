<div class="w-full flex flex-col items-center py-8 px-10 gap-8">
    <img class="max-w-[80%] w-80" src="/assets/stargazer_black_vert_transparent.png" alt="Logo">
    <div class="flex flex-col items-top gap-4 max-w-prose-sm">
        <h2 class="h2">Welcome to Stargazer!</h2>
        <p> Stargazer is your Excel companion for Orion. Its primary and most basic functionality is pulling Orion query data directly
             into Excel via Excel formulas. This is an alpha version, so please be patient as more features become stable and roll out. </p>
        <h3 class="h3">Understanding this App</h3>
        <p>The Stargazer client currently exists to enable authentication, as well as host this informational page. Packaged with this app
             is the service that handles all of stargazers systems that allow the Excel add-in to function. New and roadmapped features may
              appear on the left bar, though some may stay greyed out until stable. Stargazer includes an updater, so these will fill in over time.</p>      
        <h3 class="h3">Getting Started</h3>
        <p> Make sure you have downloaded and installed the Stargazer Excel Add-in. You can verify it's active either
             by typing <code class="code">=Stargazer</code> in a cell in Excel, and confirming formulas in the Stargazer namespace appear, or by looking for it
            in the Excel menu ribbon. There also exists a task pane for Stargazer, however this is still in development</p>
        <h3 class="h3">Basic Formulas</h3> 
        <p> There are currently two basic formulas to be aware of in Stargazer:</p>
        <ol class="list-decimal list-inside">
            <li><code class="code">=Stargazer.GET_QUERY_PROMPTS(id)</code> - This formula takes the id of a query, and returns the query prompts in the folling pattern:
                <ul class="list-disc list-inside text-sm pl-4">
                    <li>Prompt1</li>
                    <li>Prompt1 Description</li>
                    <li>Empty cell</li>
                    <li>Prompt2</li>
                    <li>Prompt2 Description</li>
                    <li>Empty cell</li>
                    <li>...</li> 
            </li>
            <li><code class="code">=Stargazer.QUERY(id, cache, [args])</code> - This formula takes in a query id, a boolean, then all args as strings. Some things to note:
                <ul class="list-disc list-inside text-sm pl-4">
                    <li><strong>cache</strong> - If true, the formula will attempt to retrieve a cached query. Queries are cached base on their ID and arguments. This is extremely useful if
                         you need to use the same query multiple times in the same day, and don't need moment to moment changes. If you need to ensure query data is
                          always up to date to the moment the formula calculates, then set this to false. </li>
                    <li><strong>args</strong> - These are the arguments for the query. They must be in the same order as the prompts, and must be strings. Any dates in excel should be converted
                         to formated strings. This formula can accept an arbitrary amount of arguments.</li>
            </li>
        </ol>
        <h3 class="h3">Examples</h3>
        <p>Nothing to see here yet...</p>
    </div>
</div>