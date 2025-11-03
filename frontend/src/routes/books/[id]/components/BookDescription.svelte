<script lang="ts">
	interface Props {
		description: string;
	}
	const TEXT_LIMIT = 250;
	let isExpand = $state(false);

	let { description }: Props = $props();
</script>

<div class="prose prose-sm sm:prose dark:prose-invert text-gray-600 dark:text-gray-400 mb-6">
	{#if description.length > TEXT_LIMIT}
		<p>{isExpand ? description : description.substring(0, TEXT_LIMIT) + '...'}</p>
		<button
			class="cursor-pointer text-primary hover:underline"
			onclick={() => (isExpand = !isExpand)}>{isExpand ? 'Read Less' : 'Read more'}</button
		>
	{:else}
		<p>{description}</p>
	{/if}
</div>
