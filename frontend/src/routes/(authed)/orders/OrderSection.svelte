<script lang="ts">
	import StatusBadge from '$lib/components/StatusBadge.svelte';
	import type { OrderDetail } from '$lib/types/order';
	import { getThailandDatetime } from '$lib/utils';
	import BookCard from './BookCard.svelte';

	let { order }: { order: OrderDetail } = $props();
	const { date, time } = getThailandDatetime(order.created_at);
</script>

<details
	class="flex flex-col rounded-xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-background-dark px-4 group"
	open={true}
>
	<summary class="flex cursor-pointer items-center justify-between gap-6 py-4 list-none">
		<div class="flex flex-col sm:flex-row sm:items-center gap-x-8 gap-y-2 flex-wrap">
			<div class="flex flex-col">
				<p class="text-sm text-slate-500 dark:text-slate-400">Order ID</p>
				<p class="text-slate-800 dark:text-slate-200 font-medium leading-normal">
					#{order.id}
				</p>
			</div>
			<div class="flex flex-col">
				<p class="text-sm text-slate-500 dark:text-slate-400">Date</p>
				<p class="text-slate-800 dark:text-slate-200 font-medium leading-normal">
					{date}
					{time}
				</p>
			</div>
			<div class="flex flex-col">
				<p class="text-sm text-slate-500 dark:text-slate-400">Total</p>
				<p class="text-slate-800 dark:text-slate-200 font-medium leading-normal">
					${order.total_price}
				</p>
			</div>
			<div class="flex items-center">
				<StatusBadge order_status={order.order_status} />
			</div>
		</div>
		<div class="text-slate-800 dark:text-slate-200 group-open:rotate-180 transition-transform">
			<span class="material-symbols-outlined" style="font-size: 24px;">expand_more</span>
		</div>
	</summary>
	<div class="border-t border-slate-200 dark:border-slate-700 -mx-4">
		<ul class="py-4 px-4 flex flex-col gap-4">
			{#each order.items as item (item.book_id)}
				<BookCard book={item} />
			{/each}
		</ul>
	</div>
</details>
