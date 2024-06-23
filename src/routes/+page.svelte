<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { KomorebiMonitor, KomorebiStatus, KomorebiWorkspace } from '$lib/types';
	let status: KomorebiStatus;
	let monitors: KomorebiMonitor[] = [];
	let workspaces: KomorebiWorkspace[] = [];
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
		listen('komorebi_status', (event) => {
			console.log(event);
			status = JSON.parse(event.payload as string);
			monitors = status.monitors.elements;
			console.log(monitors);
			// for each monitor, create a workspace
			monitors.forEach((monitor) => {
				console.log(monitor);
				workspaces.push(...monitor.workspaces.elements);
			});
			console.log(workspaces);
		});
	});
	const openWorkspace = (name: string) => {
		invoke('switch_to_workspace', { workspace: name }).then((res) => {
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
			{#each monitors as monitor}
				{#each monitor.workspaces.elements as workspace, i}
					{#if workspace.name}
						<button
							class={`btn  ${monitor.workspaces.focused === i ? 'btn-success' : ''}`}
							on:click={openWorkspace(workspace.name)}>{workspace.name}</button
						>
					{/if}
				{/each}
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
