<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';
	import type { KomorebiMonitor, KomorebiStatus, KomorebiWorkspace } from '$lib/types';
	let status: KomorebiStatus;
	let monitors: KomorebiMonitor[] = [];
	let workspaces: KomorebiWorkspace[] = [];
	let komorebiBusy = false;
	onMount(async () => {
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

		listen('komorebi_status', (event: any) => {
			console.log(event);
			status = (event.payload.state as KomorebiStatus) || {};
			monitors = status.monitors?.elements || [];
			console.log(monitors);
		});
	});
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

{#if status}
	{#each monitors as monitor, mIdx}
		{#each monitor.workspaces.elements as workspace, wIdx}
			{#if workspace}
				<button
					class={`btn btm-sm  ${monitor.workspaces.focused === wIdx ? 'btn-success' : ''}`}
					on:click|preventDefault|stopPropagation|capture|trusted={() => openWorkspace(mIdx, wIdx)}
					>{workspace.name ?? (wIdx + 1).toString()}</button
				>
			{/if}
		{/each}
		{#if mIdx < monitors.length - 1}
			<div class="divider divider-horizontal"></div>
		{/if}
	{/each}
{/if}
