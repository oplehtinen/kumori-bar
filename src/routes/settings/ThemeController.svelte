<script lang="ts">
	import { onMount } from 'svelte';
	import { emit } from '@tauri-apps/api/event';
	import type { SettingsManager } from '$lib/settings';
	import ThemePreview from './ThemePreview.svelte';

	interface Props {
		settingsManager: SettingsManager;
	}

	let { settingsManager }: Props = $props();
	let activeTheme = settingsManager.get('appearance', 'activeTheme');

	async function updateTheme(theme: string) {
		await settingsManager.set('appearance', 'activeTheme', theme);
		activeTheme = theme;
		await emit('theme-changed', theme);
	}

	const themes = [
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
		'acid'
	] as const;
</script>

<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
	{#each themes as theme}
		<ThemePreview {theme} {activeTheme} onClick={() => updateTheme(theme)} />
	{/each}
</div>
