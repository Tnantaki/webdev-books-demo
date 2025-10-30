<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { Quantity, StockStatus } from '$lib/components';
	import type { Book } from '$lib/types';

	let { book }: { book: Book } = $props();

	const TEXT_LIMIT = 250;
	let isExpand = $state(false);
	let amountBook = $state(1);
</script>

<div class="flex flex-col items-center">
	<div class="w-full max-w-md aspect-[2/3] rounded-lg overflow-hidden shadow-lg">
		<img
			src={`${PUBLIC_API_BASE}${book.img_path}`}
			alt="Cover of {book.title} book"
			class="w-full h-full object-center object-contain"
		/>
	</div>
</div>
<div>
	<h1 class="text-3xl sm:text-4xl font-bold dark:text-text-dark mb-2">{book.title}</h1>
	<!-- <p class="text-lg text-gray-600 dark:text-gray-300 mb-4">
		by <a class="text-primary hover:underline" href="#">Matt Haig</a>
	</p> -->
	<div class="flex flex-wrap gap-2 mb-6">
		<span
			class="text-xs font-semibold inline-block py-1 px-2.5 uppercase rounded-full text-primary bg-primary/20"
			>{book.genre}</span
		>
	</div>
	<div class="prose prose-sm sm:prose dark:prose-invert text-gray-600 dark:text-gray-400 mb-6">
		<p>{isExpand ? book.description : book.description.substring(0, TEXT_LIMIT) + '...'}</p>
		<button
			class="cursor-pointer text-primary hover:underline"
			onclick={() => (isExpand = !isExpand)}>{isExpand ? 'Read Less' : 'Read more'}</button
		>
	</div>
	<div class="flex items-center justify-between mb-6">
		<p class="text-3xl font-bold dark:text-text-dark">${book.price_in_pound}</p>
		<StockStatus bookAvailable={book.available} />
	</div>
	<div class="flex flex-col sm:flex-row gap-4 mb-6 items-center">
		<Quantity bind:value={amountBook} available={book.available} />
		<button
			class="flex-1 bg-accent hover:bg-accent/90 text-text-dark font-bold py-2 px-6 rounded-lg shadow-md transition-transform transform hover:scale-105"
		>
			Add to Cart
		</button>
		<button
			class="p-2 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-text-light dark:text-gray-300"
		>
			<span class="icon-[line-md--heart] size-6">favorite</span>
		</button>
	</div>
	<div class="text-sm text-gray-500 dark:text-gray-400">
		<p>Free shipping on orders over $50.</p>
		<p>Expected delivery: 3-5 business days.</p>
	</div>
</div>
