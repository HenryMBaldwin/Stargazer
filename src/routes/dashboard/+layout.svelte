<script lang="ts">
    import ServerStatus from "$lib/components/ServerStatus.svelte";
    import { onMount } from "svelte";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { AppShell, AppRail} from "@skeletonlabs/skeleton"
    import { page } from "$app/stores";
    const appWindow = getCurrentWebviewWindow()
		
    onMount(() => {
        appWindow.setResizable(true);
        appWindow.setMaximizable(true);
    });


</script>

<AppShell>
    <svelte:fragment slot="sidebarLeft">
        <AppRail regionLead="font-aleo flex flex-col underline items-center pt-5" background="bg-surface-50" width="w-[125px]" height="h-full" border="border-r border-primary-500">
            <svelte:fragment slot="lead">
                <img class="w-[55px]" src="/assets/telescope.svg" alt="Telescope">
                <strong class="text-xl">Stargazer</strong>
            </svelte:fragment>
            <div class="flex flex-col items-start w-full gap-8 py-10 font-aleo items-center">
                <div class="flex flex-col items-center gap-2 w-full">
                    <a href="/dashboard/home" class="btn !w-5/6 hover:!w-full text-md hover:text-surface-50 hover:bg-secondary-500 hover:underline w-full rounded-none {$page.url.pathname === '/dashboard/home' ? 'border-y border-secondary-500': ''}">Home</a>
                    <a href="/dashboard/queries" class="btn !w-5/6 hover:!w-full text-md hover:text-surface-50 hover:bg-secondary-500 hover:underline w-full rounded-none {$page.url.pathname === '/dashboard/queries' ? 'border-y border-secondary-500': ''}">Queries</a>
                    <a href="/dashboard/scheduler" class="btn !w-5/6 hover:!w-full text-md hover:text-surface-50 hover:bg-secondary-500 hover:underline w-full rounded-none {$page.url.pathname === '/dashboard/scheduler' ? 'border-y border-secondary-500': ''}">Scheduler</a>
                    <a href="/dashboard/settings" class="btn !w-5/6 hover:!w-full text-md hover:text-surface-50 hover:bg-secondary-500 hover:underline w-full rounded-none {$page.url.pathname === '/dashboard/settings' ? 'border-y border-secondary-500': ''}">Settings</a>
                </div>
            </div>
            <svelte:fragment slot="trail">
            <div class="w-5/6 m-auto pb-4">   
              <ServerStatus />
            </div>
            </svelte:fragment>
        </AppRail>
    </svelte:fragment>
    <slot />
</AppShell>