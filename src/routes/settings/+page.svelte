<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { load, Store } from '@tauri-apps/plugin-store';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import ThemeController from './ThemeController.svelte';
	let unlisten: UnlistenFn;
	let store: Store;
	onMount(async () => {
		store = await load('settings.json', { autoSave: true });
		unlisten = await getCurrentWindow().onCloseRequested(async (event) => {
			console.error('Trying to close Window, not allowed');
			event.preventDefault();
		});
	});
	onDestroy(async () => {
		unlisten();
	});
</script>

<div class="card bg-primary text-primary-content w-full h-full">
	<div class="card-body">
		<ThemeController {store}></ThemeController>
	</div>
</div>
