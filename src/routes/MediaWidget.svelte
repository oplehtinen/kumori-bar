<script lang="ts">
	import MediaControlButton from './ClickButton.svelte';
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';
	import PrevIcon from './Icons/PrevIcon.svelte';
	import PauseIcon from './Icons/PauseIcon.svelte';
	import NextIcon from './Icons/NextIcon.svelte';
	import LoadingIcon from './Icons/LoadingIcon.svelte';
	import PlayIcon from './Icons/PlayIcon.svelte';
	import Spotify from './Spotify.svelte';
	import { updateSignal } from '../store/update';
	let metadata: any = $state();
	let newMetadata: any;
	let processing = $state(false);
	onMount(async () => {
		invoke('get_player_status')
			.then((res) => {
				console.log(res);
				console.log('getting player status');
			})
			.catch((err) => {
				console.error(err);
			});
		listen('player_status', (event: any) => {
			let albumArtData = event.payload.art_data.data;
			let mimetype = event.payload.art_data.mimetype;
			let albumArt = new Blob([new Uint8Array(albumArtData)], { type: mimetype });
			let url = URL.createObjectURL(albumArt);
			newMetadata = event.payload;
			console.log(newMetadata.title);
			newMetadata.albumArt = url;
			if (
				metadata == undefined ||
				metadata.title != newMetadata.title ||
				metadata.playing != newMetadata.playing
			) {
				metadata = newMetadata;
				processing = false;
				updateSignal.update((n) => n + 1);
			}
		});
	});
	let controls = $state(false);
	const showControls = () => {
		controls = true;
		console.log('showing controls');
	};
	const hideControls = () => {
		controls = false;
		console.log('hiding controls');
	};
	const controlCmd = (cmd: string, aumid: string) => {
		if (cmd !== 'play_pause') {
			processing = true;
		} else {
			metadata.playing = !metadata.playing;
		}

		invoke(cmd, {
			aumid: aumid.toString()
		})
			.then((res) => {
				console.log(res);
			})
			.catch((err) => {
				console.error(err);
			});
		setTimeout(() => {
			processing = false;
		}, 2000);
	};
	/* $: metadata, (processing = false); */
</script>

{#if metadata}
	<div
		onmouseenter={showControls}
		onmouseleave={hideControls}
		role="banner"
		aria-roledescription="button"
	>
		<div
			class="absolute {controls
				? ''
				: 'hidden'} top-4 right-8 flex justify-end items-center gap-x-2 z-10"
		>
			<MediaControlButton
				onClick={() => controlCmd('previous', metadata.player_aumid)}
				icon={PrevIcon}
			/>
			<MediaControlButton
				onClick={() => controlCmd('play_pause', metadata.player_aumid)}
				icon={metadata.playing ? PauseIcon : PlayIcon}
			/>
			<MediaControlButton
				onClick={() => controlCmd('next', metadata.player_aumid)}
				icon={NextIcon}
			/>
			<div class="divider divider-horizontal"></div>
			<Spotify></Spotify>
		</div>
		<div class="stat relative z-0 justify-items-end {controls ? 'blur-sm' : ''}">
			<div class="stat-figure text-secondary">
				<div class="avatar">
					{#if processing}
						<div class="skeleton h-8 w-8"></div>
					{:else}
						<div class="w-8 rounded-full">
							<!-- svelte-ignore a11y_missing_attribute -->
							<img class="img-sm" src={metadata.albumArt} />
						</div>
					{/if}
				</div>
			</div>
			<div class="text-lg truncate">
				{#if processing}
					<LoadingIcon />
				{:else}
					{metadata.artist} - {metadata.title}{/if}
			</div>
			<div class="stat-title">
				{#if processing}
					<LoadingIcon />
				{:else}
					{metadata.album}
				{/if}
			</div>
		</div>
	</div>
{/if}
