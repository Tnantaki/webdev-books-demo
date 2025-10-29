<script lang="ts">
	import type { BookPagination } from '$lib/types';
	import { Pagination } from 'bits-ui';

	interface Props {
		pagination: BookPagination;
		onPageChange: (page: number) => void;
	}

	let { pagination, onPageChange }: Props = $props();
</script>

<Pagination.Root count={pagination.total_pages} page={pagination.current_page} {onPageChange}>
	{#snippet child({ pages, range })}
		<div class="flex items-center gap-1">
			<Pagination.PrevButton
				class="text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary flex size-10 items-center justify-center cursor-pointer"
			>
				<span class="icon-[material-symbols--chevron-left] size-6"></span>
			</Pagination.PrevButton>
			{#each pages as page (page.key)}
				{#if page.type === 'ellipsis'}
					<div class="flex items-center justify-center text-sm leading-normal font-normal px-1">
						...
					</div>
				{:else}
					<Pagination.Page
						{page}
						class="data-selected:bg-primary hover:bg-primary/20 dark:hover:bg-primary/30 flex size-10 items-center justify-center rounded-full leading-normal font-normal data-selected:font-bold data-selected:text-white cursor-pointer"
					>
						{page.value}
					</Pagination.Page>
				{/if}
			{/each}
			<Pagination.NextButton
				class="text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary flex size-10 items-center justify-center cursor-pointer"
			>
				<span class="icon-[material-symbols--chevron-right] size-6"></span>
			</Pagination.NextButton>
		</div>
	{/snippet}
</Pagination.Root>
