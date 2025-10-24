<script lang="ts">
	import { BookCard, BookFilter, Pagination } from '$lib/components/home';
	import { mockup_books, mockup_genres } from '$lib/data/mockup_books';
	import type { PageProps } from './$types';
	import type { Book, BookFilterParams } from '$lib/types';
	import { goto } from '$app/navigation';

	let books: Book[] = [];
	let current_page: number = 1;
	let loading: boolean = false;
	let error: string | null = null;

	let { data }: PageProps = $props();

	let pagination = {
		current_page: 1,
		has_next: true,
		has_previous: false,
		total_pages: 9,
		per_page: 12,
		total_items: 108
	};

	let filter: BookFilterParams = $state({
		page: 1,
		per_page: 12,
		order: 'asc',
		sort_by: 'title'
	});

	const changePage = (page: number) => {
		filter.page = page;
		goto(`?page=${page}`);
	};
</script>

<main class="flex-1">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<div class="flex items-center justify-between mb-8">
			<h2 class="text-2xl font-bold">List Books</h2>
			<BookFilter genres={mockup_genres} />
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
			{#each mockup_books as book}
				<BookCard {...book} />
			{/each}
		</div>
		<Pagination {pagination} onPageChange={changePage} />
	</div>
</main>
