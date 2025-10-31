<script lang="ts">
	import type { PageProps } from './$types';
	import BookDetail from './components/BookDetail.svelte';
	import BreadCrumb from './components/BreadCrumb.svelte';
	import Review from './components/Review.svelte';

	let { data }: PageProps = $props();
	const book = data.book;
</script>

<div class="max-w-6xl mx-auto">
	<BreadCrumb genre={book.genre} title={book.title} />
	<div class="grid grid-cols-1 md:grid-cols-2 gap-12">
		<BookDetail {book} />
	</div>
	<div class="mt-16 border-t border-gray-200 dark:border-gray-700 pt-10">
		<Review />
	</div>
</div>
