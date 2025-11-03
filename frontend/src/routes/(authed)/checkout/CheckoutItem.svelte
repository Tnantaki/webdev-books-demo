<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import type { CartItem } from '$lib/types/cart';

	interface Props {
		item: CartItem;
	}

	let { item }: Props = $props();
	let price = item.quantity * item.book.price_in_pound;
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
		<p class="ml-4">${price}</p>
	</div>
	<p class="mt-1 text-[#4c809a] dark:text-slate-400">Qty: {item.quantity}</p>
</div>
