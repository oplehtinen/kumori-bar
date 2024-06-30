<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import PrevIcon from './Icons/PrevIcon.svelte';
	import PauseIcon from './Icons/PauseIcon.svelte';
	import NextIcon from './Icons/NextIcon.svelte';
	import { hide } from '@tauri-apps/api/app';
	let metadata;
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
			// process the album art to an image
			let albumArtData = event.payload.art_data.data;
			let mimetype = event.payload.art_data.mimetype;
			let albumArt = new Blob([new Uint8Array(albumArtData)], { type: mimetype });
			let url = URL.createObjectURL(albumArt);
			metadata = event.payload;
			metadata.albumArt = url;
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
</script>

{#if metadata}
	<div on:mouseenter={showControls} on:mouseleave={hideControls}>
		<div
			class="absolute {controls
				? ''
				: 'hidden'} top-4 right-8 flex justify-end items-center gap-x-2 z-10"
		>
			<button class="btn btn-square btn-outline">
				<PrevIcon></PrevIcon>
			</button>
			<button class="btn btn-square btn-outline">
				<PauseIcon></PauseIcon>
			</button>
			<button class="btn btn-square btn-outline">
				<NextIcon></NextIcon>
			</button>
		</div>
		<div class="stat relative z-0 justify-items-end {controls ? 'blur-sm' : ''}">
			<div class="stat-figure text-secondary">
				<div class="avatar">
					<div class="w-8 rounded-full">
						<img class="img-sm" src={metadata.albumArt} />
					</div>
				</div>
			</div>
			<div class="text-lg">{metadata.artist} - {metadata.title}</div>
			<div class="stat-title">
				{metadata.album}
			</div>
		</div>
	</div>
{/if}
