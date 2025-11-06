<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { Button } from '$lib/components';
	import type { OrderDetail } from '$lib/types/order';
	import { getThailandDatetime } from '$lib/utils';

	let { order }: { order: OrderDetail } = $props();
	const { date, time } = getThailandDatetime(order.created_at);
</script>

<div
	class="p-4 @container bg-background-light dark:bg-background-dark rounded-lg border border-border-light dark:border-border-dark flex flex-col"
>
	<div class="flex flex-col lg:flex-row">
		<p class="text-text-muted-light dark:text-muted-dark text-xl font-medium leading-normal">
			Order Id #{order.id} 
		</p>
		<p class="text-text-muted-light dark:text-muted-dark text-xl font-normal hidden lg:block">&nbsp;|&nbsp;</p>
		<p class="text-text-muted-light dark:text-muted-dark text-lg lg:text-xl font-normal">
			Placed on: {date}
			{time}
		</p>
	</div>
	<div class="border-b border-border-light dark:border-border-dark my-2"></div>
	<ul class="-my-4 divide-y divide-gray-200 dark:divide-gray-700">
		{#each order.items as book (book.book_id)}
			<li class="flex py-4">
				<div
					class="h-16 w-12 sm:h-26 sm:w-20 flex-shrink-0 overflow-hidden rounded-md border border-gray-200 dark:border-gray-700"
				>
					<img
						class="h-full w-full object-cover object-center"
						alt="{book.title} book cover"
						src={PUBLIC_API_BASE + book.img_path}
					/>
				</div>
				<div class="ml-4 flex flex-1 flex-col justify-between">
					<div class="flex justify-between font-medium text-text-light dark:text-text-dark">
						<h3 class="text-lg">
							<a href="/books/{book.book_id}">{book.title}</a>
						</h3>
					</div>
					<div class="flex flex-col sm:flex-row justify-between">
						<div class="grid grid-cols-1 sm:grid-cols-[100px_180px]">
							<p class="mt-1 text-[#4c809a] dark:text-slate-400">Qty: {book.quantity}</p>
							<p class="mt-1 text-[#4c809a] dark:text-slate-400">
								Unit Price: ${book.price_at_purchase}
							</p>
						</div>
						<div class="flex sm:w-[200px] justify-between mt-2 sm:mt-0">
							<p class="text-[#4c809a] dark:text-slate-400">Subtotal:</p>
							<p class="ml-2 text-end font-medium">${book.price_at_purchase * book.quantity}</p>
						</div>
					</div>
				</div>
			</li>
		{/each}
	</ul>
	<div class="border-b border-border-light dark:border-border-dark my-6"></div>
	<div class="flex flex-col items-stretch justify-start gap-4 w-full sm:w-[420px] self-end">
		<div class="flex justify-between items-center text-xl">
			<p class="font-semibold text-text-light dark:text-text-dark">Total</p>
			<p class="font-semibold text-text-light dark:text-text-dark text-right">
				${order.total_price}
			</p>
		</div>
		<div class="flex items-end gap-3 mt-5">
			<Button class="font-bold text-xl">Pay Now</Button>
		</div>
	</div>
</div>
