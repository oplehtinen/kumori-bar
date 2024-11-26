<script lang="ts">
	import { Store } from '@tauri-apps/plugin-store';
	import { onMount } from 'svelte';
	type Theme = string;
	export let store: Store;
	import { clientThemeStore as activeTheme } from '../../store/theme';
	import { emit } from '@tauri-apps/api/event';
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

<div class="dropdown mb-72">
	<div tabindex="0" role="button" class="btn m-1">
		{$activeTheme}
		<svg
			width="12px"
			height="12px"
			class="inline-block h-2 w-2 fill-current opacity-60"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 2048 2048"
		>
			<path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
		</svg>
	</div>
	<ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl">
		{#each themes as theme}
			<li>
				<input
					on:change={() => setActiveTheme(theme)}
					type="radio"
					name="theme-dropdown"
					class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
					aria-label={theme}
					value={theme}
				/>
			</li>
		{/each}
	</ul>
</div>
