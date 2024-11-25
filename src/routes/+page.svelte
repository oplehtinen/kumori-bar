<script lang="ts">
	import WorkspaceWidget from './WorkspaceWidget.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import MediaWidget from './MediaWidget.svelte';
	import FpsMonitor from './FpsMonitor.svelte';
	import { onMount } from 'svelte';
	onMount(async () => {});
	const winTabPress = () => {
		invoke('simulate_windows_tab');
	};
</script>

<svelte:window on:contextmenu|preventDefault />
<div
	class="navbar flex-grow-0 align-middle justify-between my-2 overflow-hidden text-white bg-transparent max-h-2 h-full}"
>
	<div class="basis-1/3">
		<button class="btn btn-ghost text-xl" on:click={winTabPress}
			><svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-6"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z"
				/>
			</svg>
		</button>
	</div>
	<div class="basis-1/3 justify-center gap-x-2">
		<WorkspaceWidget></WorkspaceWidget><FpsMonitor></FpsMonitor>
	</div>
	<div class="basis-1/3 justify-end">
		<MediaWidget></MediaWidget>
	</div>
</div>
