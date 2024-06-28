<script lang="ts">
	import WorkspaceWidget from './WorkspaceWidget.svelte';

	import { onMount } from 'svelte';
	import { LogicalSize, WindowManager, currentMonitor } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	const barHeight = 100;
	const appWindow = new WindowManager('main');
	onMount(async () => {
		const monitor = await currentMonitor();
		if (!monitor) {
			return;
		}
		const screenWidth = monitor.size.width;
		setWindowSize(appWindow, screenWidth, barHeight);
	});
	invoke('set_komorebi_offset', {
		offset: (barHeight / 2 - 10).toString()
	}).then((res) => {
		console.log(res);
	});
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
	});
	const setWindowSize = async (window: WindowManager, width: number, height: number) => {
		const innerSize = await window.innerSize();
		innerSize.width = width;
		innerSize.height = height;
		window.setSize(new LogicalSize(width, height));
	};
</script>

<div class="navbar flex-grow-0 overflow-hidden text-white bg-transparent max-h-2 h-full}">
	<div class="flex-1">
		<button class="btn btn-ghost text-xl"
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
	<div class="flex-grow gap-2">
		<WorkspaceWidget></WorkspaceWidget>
	</div>
	<div class="flex-none">Placeholder</div>
</div>
