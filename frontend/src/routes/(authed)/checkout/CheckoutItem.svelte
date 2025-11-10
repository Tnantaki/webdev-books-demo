<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import type { CartItem } from '$lib/types/cart';

	interface Props {
		item: CartItem;
	}

	let { item }: Props = $props();
	let price = item.quantity * item.book.price;
</script>

<div
	class="h-16 w-12 flex-shrink-0 overflow-hidden rounded-md border border-gray-200 dark:border-gray-700"
>
	<img
		class="h-full w-full object-cover object-center"
		alt="{item.book.title} book cover"
		src={PUBLIC_API_BASE + item.book.img_path}
	/>
</div>
<div class="ml-4 flex flex-1 flex-col justify-center">
	<div class="flex justify-between font-medium text-text-light dark:text-text-dark">
		<h3 class="text-lg">
			<a href="/books/{item.book.id}">{item.book.title}</a>
		</h3>
	</div>
	<div class="flex flex-col sm:flex-row justify-between">
		<div class="grid grid-cols-1 sm:grid-cols-[100px_180px]">
			<p class="mt-1 text-[#4c809a] dark:text-slate-400">Qty: {item.quantity}</p>
			<p class="mt-1 text-[#4c809a] dark:text-slate-400">Unit Price: ${item.book.price}</p>
		</div>
		<div class="grid grid-cols-2 mt-2 sm:mt-0">
			<p class="text-[#4c809a] dark:text-slate-400">Subtotal:</p>
			<p class="ml-2 text-end font-medium">${price}</p>
		</div>
	</div>
</div>
