<script lang="ts">
	import type { PageProps } from './$types';
	import type { BookFilterParams, BookOrder, BookSort } from '$lib/types/book';
	import { goto } from '$app/navigation';
	import BookAside from './components/BookAside.svelte';
	import Pagination from './components/Pagination.svelte';
	import BookCard from './components/BookCard.svelte';
	import { page } from '$app/state';

	let loading: boolean = false;
	let error: string | null = null;

	let { data }: PageProps = $props();

	const order = page.url.searchParams.get('order') as BookOrder;
	const sort_by = page.url.searchParams.get('sort_by') as BookSort;
	let filter: BookFilterParams = $state({
		page: data.pagination.current_page || 1,
		per_page: data.pagination.per_page || 12,
		order: order || 'asc',
		sort_by: sort_by || 'title',
		genre: undefined
	});

	const nevigatePage = () => {
		const { page, per_page, order, sort_by, genre } = filter;
		const params = new URLSearchParams();

		params.append('page', page.toString());
		params.append('per_page', per_page.toString());
		params.append('order', order);
		params.append('sort_by', sort_by);
		genre && params.append('genre', genre);

		const queryString = params.toString();
		goto(`?${queryString}`);
	};

	const changePage = (page: number) => {
		filter.page = page;
		nevigatePage();
	};

	const filterPage = (per_page: number, order: BookOrder, sort_by: BookSort) => {
		filter.page = 1;
		filter.per_page = per_page;
		filter.order = order;
		filter.sort_by = sort_by;
		nevigatePage();
	};

	const changeGenre = (genre: string | undefined) => {
		filter.page = 1;
		filter.genre = genre;
		nevigatePage();
	};

	$effect(() => {
		// reset filter when navigate to base route
		if (page.url.searchParams.size === 0) {
			filter = {
				page: data.pagination.current_page || 1,
				per_page: data.pagination.per_page || 12,
				order: order || 'asc',
				sort_by: sort_by || 'title',
				genre: undefined
			};
		}
	});
</script>

<div class="flex flex-col lg:flex-row gap-8">
	<BookAside {filter} genres={data.genres} {filterPage} {changeGenre} />

	<div class="flex-1">
		<div class="flex items-center justify-between mb-8">
			<h2 class="text-3xl font-bold">{filter.genre ? filter.genre : 'All'} Book</h2>
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
			{#each data.books as book}
				<BookCard {...book} />
			{/each}
		</div>
		<div class="mt-8 w-full flex flex-col items-center">
			{#if data.pagination}
				{#snippet page(items: number, total_items: number)}
					<p class="text-muted-light dark:text-muted-dark">
						Showing {items}
						{items > 1 ? 'items' : 'item'} of {total_items} items
						{total_items > 1 ? 'items' : 'item'}
					</p>
				{/snippet}

				{@render page(data.books.length, data.pagination.total_items)}
				<div class="w-full flex justify-center items-center mt-2">
					<Pagination pagination={data.pagination} onPageChange={changePage} />
				</div>
			{:else}
				<p>Pagination Not Found</p>
			{/if}
		</div>
	</div>
</div>
