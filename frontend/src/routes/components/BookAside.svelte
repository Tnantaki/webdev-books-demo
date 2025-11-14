<script lang="ts">
	import { Button } from '$lib/components';
	import type { BookFilterParams, BookOrder, BookSort } from '$lib/types/book';
	import { ArrowDown, ArrowUp } from '@lucide/svelte';

	interface FilterState {
		per_page: number;
		order: BookOrder;
		sort_by: BookSort;
		genre: string | undefined;
	}

	interface Props {
		filter: BookFilterParams;
		genres: string[];
		filterPage: (per_page: number, order: BookOrder, sort_by: BookSort) => void;
		changeGenre: (genre: string | undefined) => void;
	}

	const { filter, genres, filterPage, changeGenre }: Props = $props();

	const sortByOption: { value: BookSort; title: string }[] = [
		{ value: 'title', title: 'Title' },
		{ value: 'genre', title: 'Genre' },
		{ value: 'price', title: 'Price' },
		{ value: 'average_rating', title: 'Rating' }
	];

	const perPageOption = [
		{ value: 12, title: 12 },
		{ value: 24, title: 24 },
		{ value: 36, title: 36 },
		{ value: 48, title: 48 }
	];

	const orderByOption: { value: BookOrder; title: string }[] = [
		{ value: 'asc', title: 'Asc' },
		{ value: 'desc', title: 'Desc' }
	];

	let filterState: FilterState = $state({
		per_page: filter.per_page,
		order: filter.order,
		sort_by: filter.sort_by,
		genre: filter.genre
	});

	$effect(() => {
		filterState = {
			per_page: filter.per_page,
			order: filter.order,
			sort_by: filter.sort_by,
			genre: filter.genre
		};
	});
</script>

<aside class="w-full lg:w-64 flex-shrink-0">
	<div class="bg-card-light dark:bg-card-dark p-6 rounded-xl shadow-sm space-y-2 sticky top-8">
		<div>
			<h3 class="text-lg font-semibold mb-4 border-b pb-2 border-gray-200 dark:border-gray-700">
				Filter &amp; Sort
			</h3>
		</div>
		<div class="space-y-4">
			<label class="text-sm font-medium text-muted-light dark:text-muted-dark" for="items-per-page"
				>Items per page</label
			>
			<select
				class="w-full mt-1 p-2.5 border-r-8 border-transparent rounded-lg ring-1 ring-gray-300 dark:ring-gray-600 bg-background-light dark:bg-background-dark focus:ring-primary text-sm"
				id="items-per-page"
				bind:value={filterState.per_page}
			>
				{#each perPageOption as { value, title }}
					<option {value}>{title}</option>
				{/each}
			</select>
		</div>
		<div class="space-y-3">
			<label class="text-sm font-medium text-muted-light dark:text-muted-dark" for="sort-by"
				>Sort by</label
			>
			<select
				class="w-full mt-1 p-2.5 border-r-8 border-transparent rounded-lg ring-1 ring-gray-300 dark:ring-gray-600 bg-background-light dark:bg-background-dark focus:ring-primary text-sm"
				id="sort-by"
				bind:value={filterState.sort_by}
			>
				{#each sortByOption as { value, title }}
					<option {value}>{title}</option>
				{/each}
			</select>
		</div>
		<div class="space-y-3">
			<p class="text-sm font-medium text-muted-light dark:text-muted-dark">Sort order</p>
			<div class="flex items-center gap-2">
				{#each orderByOption as { title, value }}
					<label
						class="flex-1 flex items-center justify-center gap-2 p-1.5 rounded-lg cursor-pointer {filterState.order ===
						value
							? 'bg-primary/10 dark:bg-primary/20 text-primary'
							: 'hover:bg-gray-200 dark:hover:bg-gray-700'}"
					>
						<input type="radio" class="sr-only" {value} bind:group={filterState.order} />
						{#if value === 'asc'}
							<ArrowUp size={20} />
						{:else}
							<ArrowDown size={20} />
						{/if}
						{title}
					</label>
				{/each}
			</div>
		</div>
		<Button
			class="mt-4 p-1.5"
			onclick={() => filterPage(filterState.per_page, filterState.order, filterState.sort_by)}
			>Apply Filters</Button
		>
		<div class="space-y-4 pt-4 border-t border-gray-200 dark:border-gray-700">
			<h4 class="text-sm font-medium text-muted-light dark:text-muted-dark">Genres</h4>
			<div class="space-y-2">
				<button
					class="block text-sm cursor-pointer {filterState.genre === undefined &&
						'font-medium text-primary'}"
					onclick={() => changeGenre(undefined)}>All</button
				>
				{#each genres as genre}
					<button
						class="block text-sm hover:text-primary cursor-pointer {filterState.genre === genre &&
							'font-medium text-primary'}"
						onclick={() => changeGenre(genre)}>{genre}</button
					>
				{/each}
			</div>
		</div>
	</div>
</aside>
