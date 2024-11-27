<script lang="ts">
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/plugin-shell';
	import type { SettingsManager } from '$lib/settings';

	interface Props {
		settingsManager: SettingsManager;
	}

	let { settingsManager }: Props = $props();
	let spotifySettings = $state(settingsManager.get('features', 'spotify'));

	const spotifyHelp =
		'https://developer.spotify.com/documentation/web-api/concepts/apps#:~:text=among%20others%2C%20the-,Client%20ID,-and%20Client%20Secret';

	async function updateSpotifyEnabled(enabled: boolean) {
		if (enabled) {
			await settingsManager.set('features', 'spotify', {
				enabled: true,
				clientId: spotifySettings.clientId || ''
			});
		} else {
			await settingsManager.set('features', 'spotify', {
				enabled: false
			});
		}
		spotifySettings = settingsManager.get('features', 'spotify');
	}

	async function updateClientId(clientId: string) {
		if (spotifySettings.enabled) {
			await settingsManager.set('features', 'spotify', {
				enabled: true,
				clientId
			});
			spotifySettings = settingsManager.get('features', 'spotify');
		}
	}

	const openLink = (url: string) => {
		open(url);
	};
</script>

<div class="divider"></div>
<div class="form-control">
	<label class="label cursor-pointer">
		<span class="label-text text-lg">Spotify integration</span>
		<input
			type="checkbox"
			class="toggle"
			checked={spotifySettings.enabled}
			onchange={(e) => updateSpotifyEnabled(e.currentTarget.checked)}
		/>
	</label>
	{#if spotifySettings.enabled}
		<label class="input input-bordered flex items-center gap-2">
			Client Id
			<input
				type="text"
				class="grow"
				placeholder="078fbc2ae68e49cb84c2381edabdf1b5"
				value={spotifySettings.clientId || ''}
				oninput={(e) => updateClientId(e.currentTarget.value)}
			/>
		</label>
		<div class="label">
			<span class="label-text-alt"></span>
			<span class="label-text-alt cursor-pointer" onclick={() => openLink(spotifyHelp)}>
				How do I get a ClientId?
			</span>
		</div>
	{/if}
</div>
<div class="divider"></div>
