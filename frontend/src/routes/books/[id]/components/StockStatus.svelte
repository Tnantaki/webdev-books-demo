<script lang="ts">
	let { bookAvailable } = $props();
</script>

<div class="flex items-center gap-2">
	{#if bookAvailable > 0}
		<span class="w-3 h-3 bg-green-500 rounded-full"></span>
		<p class="text-sm font-medium text-green-600 dark:text-green-400">In Stock</p>
	{:else}
		<span class="w-3 h-3 bg-red-500 rounded-full"></span>
		<p class="text-sm font-medium text-red-600 dark:text-red-400">Out of Stock</p>
	{/if}
</div>
