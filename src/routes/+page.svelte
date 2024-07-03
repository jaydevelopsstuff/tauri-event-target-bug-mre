<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { getCurrent } from "@tauri-apps/api/window";
    import { onMount } from "svelte";

    let receivedEvent = false;

    onMount(async () => {
        await listen("test-event", () => (receivedEvent = true));
    });

    async function dispatchEvent() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        await invoke("dispatch_event");
    }
</script>

<span>This window's label is "{getCurrent().label}"</span>
<button on:click={dispatchEvent}>
    Dispatch Event to window w/ label "main-1"
</button>
{#if receivedEvent}
    <h2>Event Received</h2>
    <button on:click={() => (receivedEvent = false)}>Reset</button>
{/if}
