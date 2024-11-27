<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { load, Store } from '@tauri-apps/plugin-store';
	import { getCurrentWindow, Window } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import ThemeController from './ThemeController.svelte';
	import SettingsMenuItem from './SettingsMenuItem.svelte';
	import SettingsBar from './SettingsBar.svelte';
	import FeatureController from './FeatureController.svelte';
	import { SettingsManager } from '$lib/settings';
	let unlisten: UnlistenFn;
	let settingsManager: SettingsManager;
	let currentWindow: Window;
	onMount(async () => {
		const store = await load('settings.json', { autoSave: true });
		settingsManager = await SettingsManager.create(store);
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
	<div class="card-body h-full text-primary">
		{#if settingsManager}<div class="join join-vertical w-full">
				<SettingsMenuItem
					title="Features"
					component={FeatureController}
					{settingsManager}
					componentProps={{}}
				></SettingsMenuItem>
				<SettingsMenuItem
					title="Themes"
					component={ThemeController}
					{settingsManager}
					componentProps={{}}
				/>
			</div>
		{/if}
	</div>
</div>
