<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components';
	import { orderStore } from '$lib/store/order.svelte';
	import type { PageProps } from './$types';
	import OrderCard from './OrderCard.svelte';

	let { data }: PageProps = $props();
</script>

{#if data.orders && data.orders.length > 0}
	<div
		class="flex flex-col gap-6 p-4 sm:p-6 md:p-10 bg-surface-light dark:bg-surface-dark rounded-lg"
	>
		<div class="flex flex-wrap justify-between gap-3">
			<h1
				class="text-text-light dark:text-text-dark text-4xl font-black leading-tight tracking-[-0.033em] min-w-72"
			>
				Pending Payments
			</h1>
		</div>
		<!-- Order Cards Section -->
		<div class="flex flex-col gap-6">
			{#each data.orders as order (order.id)}
				<OrderCard {order} />
			{/each}
		</div>
	</div>
{:else}
	<!-- EmptyState Component - (Uncomment this section to see the empty state) -->
	<div
		class="flex flex-col px-4 py-12 border-2 border-dashed border-border-light dark:border-border-dark rounded-lg"
	>
		<div class="flex flex-col items-center gap-6">
			<div class="text-primary">
				<span class="material-symbols-outlined !text-7xl"> check_circle </span>
			</div>
			<div class="flex max-w-[480px] flex-col items-center gap-2">
				<p
					class="text-text-light dark:text-text-dark text-lg font-bold leading-tight tracking-[-0.015em] max-w-[480px] text-center"
				>
					You have no pending payments
				</p>
				<p
					class="text-text-muted-light dark:text-muted-dark text-sm font-normal leading-normal max-w-[480px] text-center"
				>
					All your orders are up to date. Ready to find your next read?
				</p>
			</div>
			<Button class="min-w-[84px] max-w-[360px]" onclick={() => goto('/')}>Browse Books</Button>
		</div>
	</div>
{/if}
