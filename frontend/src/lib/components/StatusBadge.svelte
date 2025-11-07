<script lang="ts">
	type OrderStatus = 'pending' | 'paid' | 'completed' | 'cancelled';
	interface Props {
		order_status: string;
	}

	const { order_status }: Props = $props();
</script>

{#if order_status === 'pending'}
	<span
		class="badge bg-yellow-100 dark:bg-yellow-900 text-yellow-700 dark:text-yellow-200 ring-1 ring-inset ring-yellow-600/20 dark:ring-yellow-500/30"
		>Pending</span
	>
{:else if order_status === 'paid'}
	<span
		class="badge bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 ring-1 ring-inset ring-blue-600/20 dark:ring-blue-500/30"
		>Paid</span
	>
{:else if order_status === 'completed'}
	<span
		class="badge bg-emerald-100 dark:bg-emerald-900 text-emerald-700 dark:text-emerald-200 ring-1 ring-inset ring-emerald-600/20 dark:ring-emerald-500/30"
		>Delivered</span
	>
{:else if order_status === 'cancelled'}
	<span
		class="badge bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 ring-1 ring-inset ring-red-600/20 dark:ring-red-500/30"
		>Cancelled</span
	>
{:else}
	<span
		class="badge bg-gray-100 dark:bg-gray-900 text-gray-700 dark:text-gray-200 ring-1 ring-inset ring-gray-600/20 dark:ring-gray-500/30"
		>Undefined</span
	>
{/if}

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		border-radius: var(--radius-full);
		padding-inline: calc(var(--spacing) * 3);
		padding-block: calc(var(--spacing) * 1);
		font-weight: var(--font-weight-medium);
		font-size: var(--text-sm);
	}
</style>
