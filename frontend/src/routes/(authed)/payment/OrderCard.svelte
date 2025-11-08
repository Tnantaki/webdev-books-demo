<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { PUBLIC_API_BASE } from '$env/static/public';
	import { Alert, Button } from '$lib/components';
	import { orderStore } from '$lib/store/order.svelte';
	import type { OrderDetail } from '$lib/types/order';
	import { getThailandDatetime } from '$lib/utils';
	import { Check, X } from '@lucide/svelte';

	let { order }: { order: OrderDetail } = $props();
	let isPopupError = $state(false);
	let isPopupPay = $state(false);

	const { date, time } = getThailandDatetime(order.created_at);

	async function handlePay(order_id: string) {
		const result = await orderStore.pay(order_id);
		if (result.success) {
			isPopupPay = true;
			setTimeout(() => {
				invalidateAll(); // re-run load function in current active pages
				isPopupPay = false;
			}, 3000);
		} else {
			isPopupError = true;
			setTimeout(() => (isPopupError = false), 5000);
		}
	}
</script>

<div
	class="p-4 @container bg-background-light dark:bg-background-dark rounded-lg border border-border-light dark:border-border-dark flex flex-col"
>
	<div class="flex flex-col sm:flex-row sm:items-center gap-x-8 gap-y-2 flex-wrap">
		<div class="flex flex-col">
			<p class="text-base text-slate-500 dark:text-slate-400">Order ID</p>
			<p class="text-slate-800 dark:text-slate-200 text-lg font-medium leading-normal">
				#{order.id}
			</p>
		</div>
		<div class="flex flex-col">
			<p class="text-base text-slate-500 dark:text-slate-400">Date</p>
			<p class="text-slate-800 dark:text-slate-200 text-lg font-medium leading-normal">
				{date}
				{time}
			</p>
		</div>
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
			<Button class="font-bold text-xl" onclick={() => handlePay(order.id)}>Pay Now</Button>
		</div>
	</div>
</div>

<Alert bind:open={isPopupPay}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/50"
		>
			<Check class="text-green-500 dark:text-green-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-text-light dark:text-text-dark">Pay order successfully</p>
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
			{orderStore.error}
		</p>
	{/snippet}
</Alert>
