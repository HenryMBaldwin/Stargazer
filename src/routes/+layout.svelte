<script lang=ts>
    import '..//app.postcss';

    //floating UI
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
    import { storePopup } from '@skeletonlabs/skeleton';
    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    import { onMount } from 'svelte';
    import { check, type DownloadEvent } from '@tauri-apps/plugin-updater';
    import { relaunch } from '@tauri-apps/plugin-process';
    //chcek for updates
    onMount(() => {
        const runUpdateCheck = async () => {
            console.log('checking for updates');
            const update = await check();
            if (update) {
            console.log(`found update ${update.version} from ${update.date}`);
            let downloaded = 0;
            let contentLength: number | undefined = 0;
            await update.downloadAndInstall((event: DownloadEvent) => {
                switch (event.event) {
                case 'Started':
                    contentLength = event.data.contentLength;
                    console.log(`started downloading ${contentLength} bytes`);
                    break;
                case 'Progress':
                    downloaded += event.data.chunkLength;
                    console.log(`downloaded ${downloaded} from ${contentLength}`);
                    break;
                case 'Finished':
                    console.log('download finished');
                    break;
                }
            });

            console.log('update installed');
            await relaunch();
            }
        };

        // Run update check immediately
        runUpdateCheck();

        // Run update check every hour (3600000 ms)
        const intervalId = setInterval(runUpdateCheck, 3600000);

        // Clean up interval on unmount
        return () => clearInterval(intervalId);
        });
</script>

<slot />