<script lang="ts">
	import type { PageProps } from './$types';
	import BookDetail from './components/BookDetail.svelte';
	import Review from './components/Review.svelte';

	let { data }: PageProps = $props();
	const book = data.book;
</script>

<main class="flex-1 px-4 sm:px-6 lg:px-8 py-8">
	<div class="max-w-6xl mx-auto">
		<div class="flex flex-wrap gap-2 mb-8">
			<a
				class="text-gray-500 dark:text-gray-400 hover:text-primary text-sm font-medium"
				href="/books">Books</a
			>
			<span class="text-gray-500 dark:text-gray-400 text-sm font-medium">/</span>
			<a
				class="text-gray-500 dark:text-gray-400 hover:text-primary text-sm font-medium"
				href="/books?genre={book.genre}">{book.genre}</a
			>
			<span class="text-gray-500 dark:text-gray-400 text-sm font-medium">/</span>
			<span class="text-text-dark dark:text-white text-sm font-medium">{book.title}</span>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-12">
			<BookDetail {book} />
		</div>
		<div class="mt-16 border-t border-gray-200 dark:border-gray-700 pt-10">
			<Review />
		</div>
	</div>
</main>
