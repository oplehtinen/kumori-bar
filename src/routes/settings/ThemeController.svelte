<script lang="ts">
	import { Store } from '@tauri-apps/plugin-store';
	import { onMount } from 'svelte';
	type Theme = string;
	export let store: Store;
	import { clientThemeStore as activeTheme } from '../../store/theme';
	import { emit } from '@tauri-apps/api/event';
	import ThemePreview from './ThemePreview.svelte';
	onMount(async () => {
		if (store) {
			const storeTheme = await store.get<Theme>('activeTheme');
			if (storeTheme) {
				activeTheme.update((n) => (n = storeTheme));
			} else {
				try {
					await store.set('activeTheme', activeTheme);
				} catch {
					console.error('Cant change theme!');
				}
			}
		}
	});
	const themes: Theme[] = [
		'light',
		'dark',
		'cupcake',
		'bumblebee',
		'emerald',
		'corporate',
		'synthwave',
		'retro',
		'cyberpunk',
		'valentine',
		'halloween',
		'garden',
		'forest',
		'aqua',
		'lofi',
		'pastel',
		'fantasy',
		'wireframe',
		'black',
		'luxury',
		'dracula',
		'cmyk',
		'autumn',
		'business',
		'acid',
		'lemonade',
		'night',
		'coffee',
		'winter',
		'dim',
		'nord',
		'sunset'
	];
	const setActiveTheme = async (theme: Theme) => {
		console.log('changing theme to:' + theme);
		activeTheme.update((n) => (n = theme));
		await store.set('activeTheme', theme);
		await emit('theme-changed', theme);
	};
</script>

<div
	class="rounded-box grid grid-cols-2 gap-2 p-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 bg-neutral"
>
	{#each themes as theme}
		<ThemePreview onClick={() => setActiveTheme(theme)} {theme} activeTheme={$activeTheme}
		></ThemePreview>
	{/each}
</div>
