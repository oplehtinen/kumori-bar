<script lang="ts">
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';

	const fps = writable(0); // Create the FPS store

	let lastFrameTime = performance.now();

	function calculateFPS() {
		const now = performance.now();
		const delta = now - lastFrameTime;
		lastFrameTime = now;
		const currentFPS = 1000 / delta;
		fps.set(Math.round(currentFPS));
		requestAnimationFrame(calculateFPS);
	}

	onMount(() => {
		requestAnimationFrame(calculateFPS);
	});
</script>

<div class="fps-display w-64">
	<p>FPS: {$fps}</p>
</div>

<style>
</style>
