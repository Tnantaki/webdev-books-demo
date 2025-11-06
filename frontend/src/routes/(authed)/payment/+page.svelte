<script lang="ts">
	import type { PageProps } from './$types';
	import OrderCard from './OrderCard.svelte';

	let { data }: PageProps = $props();
	console.log(data);
</script>

{#if data.orders}
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
			<button
				class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary/20 text-primary text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/30 transition-colors"
			>
				<span class="truncate">Browse Books</span>
			</button>
		</div>
	</div>
{/if}
