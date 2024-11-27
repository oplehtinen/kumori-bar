<script lang="ts">
	import type { SettingsManager } from '$lib/settings';
	import type { Store } from '@tauri-apps/plugin-store';

	interface Props {
		settingsManager: SettingsManager;
		component: any;
		title: string;
		componentProps: any;
	}
	let { title = 'Settings', component, settingsManager, componentProps }: Props = $props();
</script>

<div
	class="collapse overflow-auto join-item bg-primary-content/35 collapse-arrow max-h-96 p-4 border-2 border-primary/40"
>
	<input type="radio" name={'accordion-1'} checked={false} />
	<div class="collapse-title text-xl font-medium">{title}</div>
	<div class="collapse-content overflow-y-hidden [input[type=checkbox]:checked~&]:overflow-visible">
		<!-- svelte-ignore svelte_component_deprecated -->
		<svelte:component this={component} {settingsManager} {...componentProps} />
	</div>
</div>
