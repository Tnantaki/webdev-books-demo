<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { StockStatus } from '$lib/components';
	import Alert from '$lib/components/Alert.svelte';
	import { cartStore } from '$lib/store/cart.svelte';
	import type { Book } from '$lib/types/book';
	import { Check, X } from '@lucide/svelte';
	import BookDescription from './BookDescription.svelte';
	import QuantityBook from './QuantityBook.svelte';
	import { authStore } from '$lib/store/auth.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { book }: { book: Book } = $props();
	let maxAvailable = book.available - cartStore.getBookQuantity(book.id);

	let isPopupAddItem = $state(false);
	let isPopupError = $state(false);
	let reachMaxQuantity = $state(false);
	let quantity = $state<number>(maxAvailable > 0 ? 1 : 0);

	async function handleAddToCart() {
		if (!authStore.user) {
			goto(`/login?redirectTo=${page.url.pathname}`)
		}
		
		const result = await cartStore.addToCart(book.id, quantity);

		if (result.success) {
			isPopupAddItem = true;
			setTimeout(() => (isPopupAddItem = false), 3000);
		} else {
			isPopupError = true;
			setTimeout(() => (isPopupError = false), 4000);
		}
	}
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
	<BookDescription description={book.description} />
	<div class="flex items-center justify-between mb-6">
		<p class="text-3xl font-bold dark:text-text-dark">${book.price}</p>
		<StockStatus bookAvailable={book.available} />
	</div>
	<div>
		<p class="p-1 text-gray-500 dark:text-gray-400">
			{book.available}
			{book.available > 1 ? 'books' : 'book'} available
		</p>
	</div>
	<div class="flex flex-col sm:flex-row gap-4 mb-2 items-center">
		<QuantityBook bind:quantity {maxAvailable} bind:reachMaxQuantity />
		<button
			onclick={handleAddToCart}
			disabled={!quantity}
			class="flex-1 bg-accent hover:bg-accent/90 text-text-dark font-bold py-2.5 px-4 rounded-lg shadow-md transition-transform transform hover:scale-105 cursor-pointer disabled:pointer-events-none disabled:opacity-50 disabled:bg-muted-light disabled:dark:bg-muted-dark aria-disabled:pointer-events-none"
		>
			Add to Cart
		</button>
		<button
			class="p-2 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-text-light dark:text-gray-300"
		>
			<span class="icon-[line-md--heart] size-6">favorite</span>
		</button>
	</div>
	{#if reachMaxQuantity}
		<div class="font-medium text-error pl-1">
			You have reached the maximum quantity available for this item
		</div>
	{/if}
	<div class="text-sm text-gray-500 dark:text-gray-400 mt-6">
		<p>Free shipping on orders over $50.</p>
		<p>Expected delivery: 3-5 business days.</p>
	</div>
</div>

<Alert bind:open={isPopupAddItem}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/50"
		>
			<Check class="text-green-500 dark:text-green-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-text-light dark:text-text-dark">Item has been added to your cart</p>
	{/snippet}
</Alert>

<Alert bind:open={isPopupError}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-red-100 dark:bg-red-900/50"
		>
			<X class="text-red-500 dark:text-red-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-error">
			{cartStore.error}
		</p>
	{/snippet}
</Alert>
