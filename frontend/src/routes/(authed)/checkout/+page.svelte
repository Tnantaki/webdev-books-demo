<script lang="ts">
	import { Alert, Button } from '$lib/components';
	import { cartStore } from '$lib/store/cart.svelte';
	import { BanknoteArrowUp, X } from '@lucide/svelte';
	import CheckoutItem from './CheckoutItem.svelte';
	import InputRadio from './InputRadio.svelte';

	let cart = $derived(cartStore.cart);
	let isPopupError = $state(false);
	let isPopupCheckout = $state(false);

	async function handleCheckout() {
		const result = await cartStore.checkout();
		if (result.success) {
			isPopupCheckout = true;
		} else {
			isPopupError = true;
			setTimeout(() => (isPopupError = false), 5000);
		}
	}
</script>

{#if cart}
	<div class="max-w-7xl mx-auto">
		<div class="flex flex-wrap justify-between gap-6 p-4">
			<div class="flex min-w-72 flex-col gap-3">
				<h1
					class="text-text-light dark:text-text-dark text-4xl font-black leading-tight tracking-[-0.033em]"
				>
					Checkout
				</h1>
				<p class="text-[#4c809a] dark:text-slate-400 text-base font-normal leading-normal">
					Complete your order by providing the following information.
				</p>
			</div>
			<a class="text-primary font-medium leading-normal self-end pb-1" href="/cart"
				>Return to Cart</a
			>
		</div>
		<div class="mt-8 flex justify-center">
			<div class="w-full max-w-4xl">
				<div class="rounded-xl bg-card-light dark:bg-card-dark/50 shadow-md p-6">
					<h2 class="text-lg font-bold text-text-light dark:text-text-dark mb-4">Order Summary</h2>
					<ul class="-my-4 divide-y divide-gray-200 dark:divide-gray-700">
						{#each cart.items as item (item.id)}
							<li class="flex py-4">
								<CheckoutItem {item} />
							</li>
						{/each}
					</ul>
				</div>
				<div class="bg-card-light dark:bg-card-dark/50 p-6 rounded-xl shadow-md mt-8">
					<h2 class="text-lg font-bold text-text-light dark:text-text-dark mb-4">Payment Method</h2>
					<div class="space-y-4">
						<InputRadio id="google-pay" name="payment-method">
							<span class="icon-[logos--google-pay] mr-3"></span>
							Google Pay
						</InputRadio>
						<InputRadio id="paypal" name="payment-method">
							<span class="icon-[logos--paypal] mr-10"></span>
							PayPal
						</InputRadio>
					</div>
					<div
						class="border-t border-gray-200 dark:border-gray-700 pt-4 mt-4 grid md:grid-cols-2 items-end"
					>
						<div></div>
						<div>
							<div class="summary-grid">
								<p class="text-[#4c809a] dark:text-slate-400 font-normal leading-normal">
									Subtotal
								</p>
								<p
									class="text-text-light dark:text-slate-300 font-normal leading-normal text-right"
								>
									${cart.total_price}
								</p>
							</div>
							<div class="summary-grid">
								<p class="text-[#4c809a] dark:text-slate-400 font-normal leading-normal">
									Shipping
								</p>
								<p
									class="text-text-light dark:text-slate-300 font-normal leading-normal text-right"
								>
									{cart.shipping_price === 0 ? 'FREE' : '$' + cart.shipping_price}
								</p>
							</div>
							<div class="summary-grid border-t border-gray-200 dark:border-gray-700 mt-2">
								<p class="text-base font-bold text-text-light dark:text-text-dark">Total</p>
								<p class="text-base font-bold text-text-light dark:text-text-dark text-right">
									${cart.total_price + cart.shipping_price}
								</p>
							</div>
							<div class="mt-6 min-w-2xs">
								<Button onclick={handleCheckout}>Place Order</Button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

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
			{cartStore.error}
		</p>
	{/snippet}
</Alert>

<Alert bind:open={isPopupCheckout}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/50"
		>
			<BanknoteArrowUp class="text-green-500 dark:text-green-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-text-light dark:text-text-dark">Checkout success, Pay your order</p>
	{/snippet}
</Alert>

<style>
	.summary-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		column-gap: 24px;
		min-width: var(--container-2xs);
		padding-block: 8px;
	}
</style>
