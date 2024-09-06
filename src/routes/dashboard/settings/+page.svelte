<script lang=ts>
    import type { Database } from "$lib/types";
	import { ListBox, ListBoxItem, popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
    import {writable} from "svelte/store";

    const databasesStore = writable<Database[]>([]);

    const selectedDatabaseStore = writable<Database>({
        id: "loading",
        name: "Loading...",
        selected: false,
    });

    onMount(async () => {
        databasesStore.set(await get_databases());
    });

    async function get_databases(): Promise<Database[]> {
        console.log("Fetching databases");
        let databases: Database[] = [];
        await invoke("get_databases").then((res) => {
            console.log(res);
            const result = res as [string, string, boolean][];
            result.forEach(item => {
                let db: Database = {
                    id: item[0],
                    name: item[1],
                    selected: item[2],
                }
                if (db.selected) {
                    selectedDatabaseStore.set(db);
                }
                databases.push(db);
            })
        })
        .catch((err) => {
            console.error(err);
            let db: Database = {
                id: "error",
                name: "Error: Could not fetch databases",
                selected: true,
            }
            databases.push(db);
        });
        return databases;
    }

    async function handDatabaseSelection(database: Database) {
        //Optimistically update the selected database
        selectedDatabaseStore.set(database);
        let success = false;
        
        await invoke("switch_database", {id: database.id}).then((res) => {
            const result = res as [string, string, boolean][];
            result.forEach(item => {
                let db: Database = {
                    id: item[0],
                    name: item[1],
                    selected: item[2],
                }
                if (db.selected) {
                    if (db.id === database.id) {
                        success = true;
                    }
                    else{
                        selectedDatabaseStore.set(db);
                    }
                }
            });
        });
    }

    let selectedDatabase: Database = {
        id: "0",
        name: "None",
        selected: false,
    }
    const selectedDatabaseCombobox: PopupSettings = {
        event: "click",
        target: "selectedDatabaseCombobox",
        placement: "bottom",
        closeQuery: ".listbox-item"
    }

    $: selectedDatabase = $selectedDatabaseStore;
</script>
<!-- combo boxes -->
 <div class="card w-48 shadow-xl py-2" data-popup="selectedDatabaseCombobox" >
    <ListBox rounded="rounded-none">
        {#each $databasesStore as database}
            <ListBoxItem bind:group={selectedDatabase} name="medium" value={database} on:click={() => handDatabaseSelection(database)}>
                <span>{database.name}</span></ListBoxItem>
        {/each}
    </ListBox>
 </div>
<!-- /combo boxes -->

<div class="w-full flex flex-col items-start py-8 px-20 gap-8 max-w-prose gap-8 font-aleo">
    <h1 class="h2 font-aleo">Settings</h1>
    <div class="flex flex-col items-start gap-4 w-full card bg-surface-50 p-4 ">
        <div class="flex flex-col items-start  w-full">
            <h3 class="h3 font-aleo">Orion</h3>
            <hr class="!border-t-1 !border-secondary-500 w-full"/>
        </div>
        <div class="flex flex-row items-center justify-between w-full">
            Database
            <button class="btn variant-ringed w-48 justify-between hover:variant-ringed-primary" use:popup={selectedDatabaseCombobox}>
                <span class="truncate text-sm">{$selectedDatabaseStore.name}</span>
                <span>↓</span>
            </button>
        </div>
    </div>

</div>
