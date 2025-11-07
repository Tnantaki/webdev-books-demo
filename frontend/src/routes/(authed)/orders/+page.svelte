<script lang="ts">
	import type { PageProps } from './$types';
	import OrderSection from './OrderSection.svelte';

	let { data }: PageProps = $props();
	console.log(data);
</script>

<div class="flex flex-wrap justify-between gap-3 p-4">
	<p
		class="text-slate-900 dark:text-slate-50 text-4xl font-black leading-tight tracking-[-0.033em] min-w-72"
	>
		Your Orders
	</p>
</div>

<div class="flex flex-col p-4 gap-4">
	{#each data.orders as order (order.id)}
		<OrderSection {order} />
	{/each}
</div>
