<script lang="ts">
	import { onMount } from 'svelte';
	import { Window } from '@tauri-apps/api/window';
	import CloseIcon from '../Icons/CloseIcon.svelte';
	export let currentWindow: Window;
	onMount(() => {});
	const closeSettingsWindow = () => {
		if (currentWindow) currentWindow.hide();
	};
</script>

<div
	class="navbar bg-primary-content/95 z-50 border-b-2 border-primary/50 backdrop-blur-md sticky top-0"
	data-tauri-drag-region
>
	<div class="flex-1" data-tauri-drag-region>
		<a class="btn btn-ghost text-xl">Settings</a>
	</div>
	<div class="flex-none">
		<button class="btn btn-square btn-ghost" on:click={closeSettingsWindow}>
			<CloseIcon></CloseIcon>
		</button>
	</div>
</div>
