<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { Quantity, StockStatus } from '$lib/components';
	import type { Book } from '$lib/types/book';
	interface Props {
		book: Book;
		quantity: number;
	}

	let { book, quantity }: Props = $props();

	let amount = $state(quantity);
	let price = $derived(amount * book.price_in_pound);
</script>

<div
	class="h-28 w-20 flex-shrink-0 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700"
>
	<img
		class="h-full w-full object-cover object-center"
		alt="{book.title} book cover"
		src={PUBLIC_API_BASE + book.img_path}
	/>
</div>
<div class="ml-4 flex flex-1 flex-col">
	<div>
		<div class="flex justify-between text-base font-medium text-[#0d171b] dark:text-slate-50">
			<h3>
				<a href="/books/{book.id}">{book.title}</a>
			</h3>
			<p class="ml-4">${price}</p>
		</div>
		<!-- <p class="mt-1 text-sm text-[#4c809a] dark:text-slate-400">by Matt Haig</p> -->
	</div>
	<div class="flex flex-1 items-end justify-between text-sm">
		<Quantity bind:value={amount} available={book.available} />
		<div class="flex">
			<button class="font-medium text-orange-500 hover:text-orange-600" type="button">Remove</button
			>
		</div>
	</div>
</div>
