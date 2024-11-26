<script lang="ts">
	import { onMount } from 'svelte';
	import { load } from '@tauri-apps/plugin-store';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	onMount(async () => {
		const store = await load('settings.json');
		const unlisten = await getCurrentWindow().onCloseRequested(async (event) => {
			console.error('Trying to close Window, not allowed');
			event.preventDefault();
		});
	});
</script>

<div class="card bg-primary text-primary-content w-96">
	<div class="card-body"></div>
</div>
