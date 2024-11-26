<script lang="ts">
	export let theme: string = 'default';
	export let onClick: () => void;
	export let activeTheme = 'default';
</script>

<div
	class="border-accent-content overflow-hidden p-2 rounded-lg {activeTheme == theme
		? 'border-4'
		: ''}"
	on:click={onClick}
>
	<div class="bg-base-100 text-base-content w-full cursor-pointer font-sans" data-theme={theme}>
		<div class="grid grid-cols-5 grid-rows-3">
			<div class="bg-base-200 col-start-1 row-span-2 row-start-1"></div>
			<div class="bg-base-300 col-start-1 row-start-3"></div>
			<div
				class="bg-base-100 col-span-4 col-start-2 row-span-3 row-start-1 flex flex-col gap-1 p-2"
			>
				<div class="font-bold">{theme}</div>
				<div class="flex flex-wrap gap-1">
					<div class="bg-primary flex aspect-square w-5 items-center justify-center rounded lg:w-6">
						<div class="text-primary-content text-sm font-bold">A</div>
					</div>
					<div
						class="bg-secondary flex aspect-square w-5 items-center justify-center rounded lg:w-6"
					>
						<div class="text-secondary-content text-sm font-bold">A</div>
					</div>
					<div class="bg-accent flex aspect-square w-5 items-center justify-center rounded lg:w-6">
						<div class="text-accent-content text-sm font-bold">A</div>
					</div>
					<div class="bg-neutral flex aspect-square w-5 items-center justify-center rounded lg:w-6">
						<div class="text-neutral-content text-sm font-bold">A</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
