<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { KomorebiMonitor, KomorebiStatus, KomorebiWorkspace } from '$lib/types';
	let status: KomorebiStatus;
	let monitors: KomorebiMonitor[] = [];
	let workspaces: KomorebiWorkspace[] = [];
	let focusedMonitor = 0;
	onMount(() => {
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
		invoke('komorebi_init_event_listener').then((res) => {
			console.log(res);
		});
		listen('komorebi_status', (event: any) => {
			console.log(event);
			status = (event.payload.state as KomorebiStatus) || {};
			monitors = status.monitors?.elements || [];
			focusedMonitor = status.monitors?.focused || 0;
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
	const openWorkspace = (monitor: number, workspace: number) => {
		invoke('switch_to_workspace', {
			monitor: monitor.toString(),
			workspace: workspace.toString()
		}).then((res) => {
			console.log(res);
		});
	};
</script>

<div class="navbar bg-base-100 text-white">
	<div class="flex-1">
		<a class="btn btn-ghost text-xl">daisyUI</a>
	</div>
	<div class="flex-grow">
		{#if status}
			{#each monitors as monitor, mIdx}
				{#each monitor.workspaces.elements as workspace, wIdx}
					{#if workspace}
						<button
							class={`btn  ${monitor.workspaces.focused === wIdx ? 'btn-success' : ''}`}
							on:click={() => openWorkspace(mIdx, wIdx)}
							>{workspace.name ?? (wIdx + 1).toString()}</button
						>
					{/if}
				{/each}
				{#if mIdx < monitors.length - 1}
					<div class="divider divider-horizontal"></div>
				{/if}
			{/each}
		{/if}
	</div>
	<div class="flex-none">
		<ul class="menu menu-horizontal px-1">
			<li><a>Link</a></li>
			<li>
				<details>
					<summary>Parent</summary>
					<ul class="bg-base-100 rounded-t-none p-2">
						<li><a>Link 1</a></li>
						<li><a>Link 2</a></li>
					</ul>
				</details>
			</li>
		</ul>
	</div>
</div>
