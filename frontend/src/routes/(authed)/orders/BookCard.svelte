<script lang="ts">
	import { PUBLIC_API_BASE } from '$env/static/public';
	import type { BookAtPurchase } from '$lib/types/order';

	const { book }: { book: BookAtPurchase } = $props();
</script>

<div class="flex items-center gap-4 bg-transparent px-4 min-h-[72px] py-1 justify-between">
	<div class="flex items-center gap-4 h-16">
		<div
			class="h-16 w-12 flex-shrink-0 overflow-hidden rounded-md border border-gray-200 dark:border-gray-700"
		>
			<img
				class="h-full w-full object-cover object-center"
				alt="{book.title} book cover"
				src={PUBLIC_API_BASE + book.img_path}
			/>
		</div>
		<div class="flex flex-col justify-between h-full">
			<p
				class="text-slate-900 dark:text-slate-50 text-lg font-medium"
			>
				<a href="/books/{book.book_id}">{book.title}</a>
			</p>
			<p class="text-slate-500 dark:text-slate-400 font-normal">
				Qty: {book.quantity}
			</p>
		</div>
	</div>
	<div class="shrink-0">
		<p class="text-slate-900 dark:text-slate-50 text-base font-medium leading-normal">
			${book.price_at_purchase}
		</p>
	</div>
</div>
