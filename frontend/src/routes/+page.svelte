<script lang="ts">
	import { BookCard, BookFilter, Pagination } from '$lib/components/home';
	import { mockup_genres } from '$lib/data/mockup_books';
	import type { PageProps } from './$types';
	import type { Book, BookFilterParams } from '$lib/types';
	import { goto } from '$app/navigation';

	let loading: boolean = false;
	let error: string | null = null;

	let { data }: PageProps = $props();

	let filter: BookFilterParams = $state({
		page: 1,
		per_page: 12,
		order: 'asc',
		sort_by: 'title',
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
</script>

<main class="flex-1">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<div class="flex items-center justify-between mb-8">
			<h2 class="text-2xl font-bold">List Books</h2>
			<BookFilter genres={mockup_genres} />
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
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
</main>
