<script lang="ts">
	import 'tailwindcss/tailwind.css';
	import { onMount, onDestroy } from 'svelte';
	import { load } from '@tauri-apps/plugin-store';
	let { children } = $props();
	import { clientThemeStore as activeTheme } from '../store/theme';
	import { get } from 'svelte/store';
	import { emit, listen } from '@tauri-apps/api/event';
	let unlisten;
	activeTheme.subscribe((theme) => {
		console.log('subscribe:' + theme);
	});
	onMount(async () => {
		unlisten = await listen('theme-changed', (event) => {
			console.log(event);
			activeTheme.set(event.payload as string);
		});
		const store = await load('settings.json', { autoSave: true });
		if (store) {
			if (store) {
				const storeTheme = await store.get<string>('activeTheme');
				if (storeTheme) {
					activeTheme.set(storeTheme);
					await store.set('activeTheme', get(activeTheme));
					await emit('theme-changed', storeTheme);
				}
			}
		}
	});
	onDestroy(async () => {
		unlisten();
	});
</script>

{#if activeTheme}
	<div data-theme={$activeTheme} class="bg-transparent">
		{@render children?.()}
	</div>
{/if}
