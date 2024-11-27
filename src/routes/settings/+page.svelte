<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { load, Store } from '@tauri-apps/plugin-store';
	import { getCurrentWindow, Window } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import ThemeController from './ThemeController.svelte';
	import SettingsMenuItem from './SettingsMenuItem.svelte';
	import SettingsBar from './SettingsBar.svelte';
	let unlisten: UnlistenFn;
	let store: Store;
	let currentWindow: Window;
	onMount(async () => {
		store = await load('settings.json', { autoSave: true });
		currentWindow = getCurrentWindow();
		unlisten = await currentWindow.onCloseRequested(async (event) => {
			console.error('Trying to close Window, not allowed');
			event.preventDefault();
		});
	});
	onDestroy(async () => {
		unlisten();
	});
</script>

<div class="bg-primary-content/95 text-primary w-full h-svh overflow-hidden">
	<SettingsBar {currentWindow}></SettingsBar>
	<div class="card-body h-2/3 text-primary">
		<div
			class="collapse overflow-auto bg-primary-content/35 collapse-arrow p-4 border-2 border-primary/40"
		>
			{#if store}
				<SettingsMenuItem title="Themes" component={ThemeController} {store} componentProps={{}} />
			{/if}
		</div>
	</div>
</div>
