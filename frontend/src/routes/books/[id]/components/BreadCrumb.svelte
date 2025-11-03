<script lang="ts">
	let { title, genre } = $props();
</script>

<div class="flex flex-wrap gap-2 mb-8">
	<a class="text-gray-500 dark:text-gray-400 hover:text-primary text-sm font-medium" href="/"
		>Books</a
	>
	<span class="text-gray-500 dark:text-gray-400 text-sm font-medium">/</span>
	<a
		class="text-gray-500 dark:text-gray-400 hover:text-primary text-sm font-medium"
		href="/books?genre={genre}">{genre}</a
	>
	<span class="text-gray-500 dark:text-gray-400 text-sm font-medium">/</span>
	<span class="text-text-light dark:text-text-dark text-sm font-medium">{title}</span>
</div>
