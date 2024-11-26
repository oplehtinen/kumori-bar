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

<div class="collapse collapse-arrow p-4">
	<input type="checkbox" name="my-accordion-2" checked={false} />
	<div class="collapse-title text-xl font-medium">{title}</div>
	<div class="collapse-content">
		<!-- svelte-ignore svelte_component_deprecated -->
		<svelte:component this={component} {store} {...componentProps} />
	</div>
</div>
