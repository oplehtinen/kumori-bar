<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import PrevIcon from './Icons/PrevIcon.svelte';
	import PauseIcon from './Icons/PauseIcon.svelte';
	import NextIcon from './Icons/NextIcon.svelte';
	import { hide } from '@tauri-apps/api/app';
	import LoadingIcon from './Icons/LoadingIcon.svelte';
	import PlayIcon from './Icons/PlayIcon.svelte';
	let metadata: any;
	let processing = false;
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
			console.log(event);

			if (
				!event.payload ||
				(metadata != undefined &&
					event.payload.title == metadata.title &&
					event.payload.playing === metadata.playing)
			) {
				console.log('returning');
				return;
			}
			// process the album art to an image
			let albumArtData = event.payload.art_data.data;
			let mimetype = event.payload.art_data.mimetype;
			let albumArt = new Blob([new Uint8Array(albumArtData)], { type: mimetype });
			let url = URL.createObjectURL(albumArt);
			metadata = event.payload;
			metadata.albumArt = url;
			console.log(metadata);
		});
	});
	let controls = false;
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
	};
	$: metadata, (processing = false);
</script>

{#if metadata}
	<div
		on:mouseenter={showControls}
		on:mouseleave={hideControls}
		role="banner"
		aria-roledescription="button"
	>
		<div
			class="absolute {controls
				? ''
				: 'hidden'} top-4 right-8 flex justify-end items-center gap-x-2 z-10"
		>
			<button
				class="btn btn-square btn-outline"
				on:click={() => controlCmd('previous', metadata.player_aumid)}
			>
				<PrevIcon></PrevIcon>
			</button>
			<button
				class="btn btn-square btn-outline"
				on:click={() => controlCmd('play_pause', metadata.player_aumid)}
			>
				{#if metadata.playing}
					<PauseIcon></PauseIcon>
				{:else}<PlayIcon></PlayIcon>
				{/if}
			</button>
			<button
				class="btn btn-square btn-outline"
				on:click={() => controlCmd('next', metadata.player_aumid)}
			>
				<NextIcon></NextIcon>
			</button>
		</div>
		<div class="stat relative z-0 justify-items-end {controls ? 'blur-sm' : ''}">
			<div class="stat-figure text-secondary">
				<div class="avatar">
					{#if processing}
						<div class="skeleton h-8 w-8"></div>
					{:else}
						<div class="w-8 rounded-full">
							<!-- svelte-ignore a11y-missing-attribute -->
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
