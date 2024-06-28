<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
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
</script>

{#if metadata}
	<div class="stat">
		<div class="stat-figure text-secondary">
			<div class="avatar">
				<div class="w-8 rounded-full">
					<img class="img-sm" src={metadata.albumArt} />
				</div>
			</div>
		</div>
		<div class="text-lg">{metadata.title}</div>
		<div class="stat-title">
			{metadata.artist}
			<span class="text-sm text-primary">{metadata.album}</span>
		</div>
	</div>
{/if}
