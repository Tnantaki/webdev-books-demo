<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { Quantity, StockStatus } from '$lib/components';
	import type { CartItem } from '$lib/types/cart';

	interface Props {
		item: CartItem;
		handleRemoveItem: (id: string) => void;
		handleEditQuantityItem: (id: string, quantity: number) => void;
	}

	let { item, handleRemoveItem, handleEditQuantityItem }: Props = $props();

	let quantity = $state(item.quantity);
	let price = $derived(quantity * item.book.price_in_pound);
	
	$effect(() => {
		handleEditQuantityItem(item.id, quantity);
	})
	
</script>

<div
	class="h-28 w-20 flex-shrink-0 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700"
>
	<img
		class="h-full w-full object-cover object-center"
		alt="{item.book.title} book cover"
		src={PUBLIC_API_BASE + item.book.img_path}
	/>
</div>
<div class="ml-4 flex flex-1 flex-col">
	<div>
		<div class="flex justify-between text-base font-medium text-[#0d171b] dark:text-slate-50">
			<h3>
				<a href="/books/{item.book.id}">{item.book.title}</a>
			</h3>
			<p class="ml-4">${price}</p>
		</div>
		<!-- <p class="mt-1 text-sm text-[#4c809a] dark:text-slate-400">by Matt Haig</p> -->
	</div>
	<div class="flex flex-1 items-end justify-between text-sm">
		<Quantity
			bind:value={quantity}
			available={item.book.available}
		/>
		<div class="flex">
			<button
				onclick={() => handleRemoveItem(item.id)}
				class="font-medium text-orange-500 hover:text-orange-600 cursor-pointer"
				type="button">Remove</button
			>
		</div>
	</div>
</div>
