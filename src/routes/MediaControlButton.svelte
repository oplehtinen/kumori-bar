<script lang="ts">
	export let onClick: () => void;
	export let icon: any;
	export let active: boolean = false;
	export let color = 'white';
</script>

<button class="btn btn-outline border-none btn-{color}" on:click={onClick}>
	<svelte:component this={icon} />
</button>
