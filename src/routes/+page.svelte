<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { LogicalSize, PhysicalSize, WindowManager, currentMonitor } from '@tauri-apps/api/window';
	import type { KomorebiMonitor, KomorebiStatus, KomorebiWorkspace } from '$lib/types';
	let status: KomorebiStatus;
	let monitors: KomorebiMonitor[] = [];
	let workspaces: KomorebiWorkspace[] = [];
	let komorebiBusy = false;
	const barHeight = 100;
	const appWindow = new WindowManager('main');
	onMount(async () => {
		const monitor = await currentMonitor();
		if (!monitor) {
			return;
		}
		const screenWidth = monitor.size.width;
		setWindowSize(appWindow, screenWidth, barHeight);

		invoke('get_komorebi_status').then((res) => {
			status = JSON.parse(res as string);
			monitors = status.monitors.elements;
			console.log(monitors);
			// for each monitor, create a workspace
			monitors.forEach((monitor) => {
				console.log(monitor);
				workspaces.push(...monitor.workspaces.elements);
			});
			console.log(workspaces);
		});
		invoke('komorebi_init_event_listener')
			.then((res) => {
				console.log(res);
				console.log('Komorebi event listener initialized');
			})
			.catch((err) => {
				console.error(err);
			});
		invoke('set_komorebi_offset', {
			offset: (barHeight / 2 - 10).toString()
		}).then((res) => {
			console.log(res);
		});
		listen('komorebi_status', (event: any) => {
			console.log(event);
			status = (event.payload.state as KomorebiStatus) || {};
			monitors = status.monitors?.elements || [];
			console.log(monitors);
			// for each monitor, create a workspace
			monitors.forEach((monitor) => {
				workspaces = [];
				console.log(monitor);
				workspaces.push(...monitor.workspaces.elements);
			});
			console.log(workspaces);
		});
	});
	const setWindowSize = async (window: WindowManager, width: number, height: number) => {
		const innerSize = await window.innerSize();
		innerSize.width = width;
		innerSize.height = height;
		window.setSize(new LogicalSize(width, height));
	};
	const openWorkspace = (monitor: number, workspace: number) => {
		if (komorebiBusy) {
			return;
		}
		komorebiBusy = true;
		invoke('switch_to_workspace', {
			monitor: monitor.toString(),
			workspace: workspace.toString()
		})
			.then((res) => {
				console.log(res);
			})
			.catch((err) => {
				console.error(err);
			})
			.finally(() => {
				komorebiBusy = false;
			});
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
		{#if status}
			{#each monitors as monitor, mIdx}
				{#each monitor.workspaces.elements as workspace, wIdx}
					{#if workspace}
						<button
							class={`btn btm-sm  ${monitor.workspaces.focused === wIdx ? 'btn-success' : ''}`}
							on:click|preventDefault|stopPropagation|capture|trusted={() =>
								openWorkspace(mIdx, wIdx)}>{workspace.name ?? (wIdx + 1).toString()}</button
						>
					{/if}
				{/each}
				{#if mIdx < monitors.length - 1}
					<div class="divider divider-horizontal"></div>
				{/if}
			{/each}
		{/if}
	</div>
	<div class="flex-none">Placeholder</div>
</div>
