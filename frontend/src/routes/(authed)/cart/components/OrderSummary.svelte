<script lang="ts">
	interface Props {
		total_price: number;
		shipping_price: number;
	}
	let { total_price, shipping_price }: Props = $props();
</script>

<h2 class="text-lg font-bold text-[#0d171b] dark:text-slate-50 mb-4">Order Summary</h2>
<div class="border-t border-gray-200 dark:border-gray-700 pt-4">
	<div class="flex justify-between gap-x-6 py-2">
		<p class="text-[#4c809a] dark:text-slate-400 text-sm font-normal leading-normal">Subtotal</p>
		<p class="text-[#0d171b] dark:text-slate-300 text-sm font-normal leading-normal text-right">
			${total_price}
		</p>
	</div>
	<div class="flex justify-between gap-x-6 py-2">
		<p class="text-[#4c809a] dark:text-slate-400 text-sm font-normal leading-normal">Shipping</p>
		<p class="text-[#0d171b] dark:text-slate-300 text-sm font-normal leading-normal text-right">
			{shipping_price === 0 ? 'FREE' : '$' + shipping_price}
		</p>
	</div>
	<div class="flex justify-between gap-x-6 py-4 border-t border-gray-200 dark:border-gray-700 mt-2">
		<p class="text-base font-bold text-[#0d171b] dark:text-slate-50">Total</p>
		<p class="text-base font-bold text-[#0d171b] dark:text-slate-50 text-right">$42.00</p>
	</div>
</div>
