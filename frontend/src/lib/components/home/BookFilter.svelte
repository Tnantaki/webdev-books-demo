<script lang="ts">
	const genres = $props();
</script>

<div class="flex gap-2 overflow-x-auto p-1">
	<button
		class="bg-primary flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg pr-4 pl-4 text-sm font-medium text-white"
	>
		All
	</button>
	{#each genres as genre}
		<button
			class="bg-card-light dark:bg-card-dark hover:bg-primary/20 dark:hover:bg-primary/30 flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg pr-4 pl-4 text-sm font-medium"
		>
			{genre}
		</button>
	{/each}
</div>
