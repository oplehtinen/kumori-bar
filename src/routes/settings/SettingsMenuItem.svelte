<script lang="ts">
	import type { Store } from '@tauri-apps/plugin-store';
	interface Props {
		store: Store;
		component: any;
		title: string;
		componentProps: any;
	}
	let { title = 'Settings', component, store, componentProps }: Props = $props();
	console.log(store);
</script>

<div
	class="collapse overflow-auto join-item bg-primary-content/35 collapse-arrow max-h-96 p-4 border-2 border-primary/40"
>
	<input type="checkbox" name={'accordion-1'} checked={false} />
	<div class="collapse-title text-xl font-medium">{title}</div>
	<div class="collapse-content overflow-y-hidden [input[type=checkbox]:checked~&]:overflow-visible">
		<!-- svelte-ignore svelte_component_deprecated -->
		<svelte:component this={component} {store} {...componentProps} />
	</div>
</div>
