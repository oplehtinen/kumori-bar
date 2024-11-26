<script lang="ts">
	import WinTabWidget from './WinTabWidget.svelte';
	import WorkspaceWidget from './WorkspaceWidget.svelte';
	import MediaWidget from './MediaWidget.svelte';
	import FpsMonitor from './FpsMonitor.svelte';
	import { onMount } from 'svelte';
	import SettingsWidget from './SettingsWidget.svelte';
	onMount(async () => {});
</script>

<div
	class="navbar flex-grow-0 align-middle justify-between my-2 overflow-hidden text-white bg-transparent max-h-2 h-full}"
>
	<div class="basis-1/3">
		<WinTabWidget></WinTabWidget>
	</div>
	<div class="basis-1/3 justify-center gap-x-2">
		<WorkspaceWidget></WorkspaceWidget><FpsMonitor></FpsMonitor>
	</div>
	<div class="basis-1/3 justify-end">
		<MediaWidget></MediaWidget>
	</div>
</div>
<SettingsWidget></SettingsWidget>
