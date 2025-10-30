<script lang="ts">
	import BookDetail from './components/BookDetail.svelte';
	import OrderSummary from './components/OrderSummary.svelte';
	import { mockCart } from './mockCart';
	
	let total_price = mockCart.items.reduce((a, c) => a + c.book.price_in_pound * c.quantity, 0);
</script>

<main class="flex-1 px-4 sm:px-6 lg:px-8 py-8 md:py-12">
	<div class="max-w-7xl mx-auto">
		<div class="flex flex-wrap justify-between gap-6 p-4">
			<div class="flex min-w-72 flex-col gap-3">
				<p
					class="text-[#0d171b] dark:text-slate-50 text-4xl font-black leading-tight tracking-[-0.033em]"
				>
					Your Shopping Cart
				</p>
				<p class="text-[#4c809a] dark:text-slate-400 text-base font-normal leading-normal">
					Review and manage the books you've selected.
				</p>
			</div>
			<a class="text-primary text-sm font-medium leading-normal self-end pb-1" href="/"
				>Continue Shopping</a
			>
		</div>
		<div class="mt-8 grid grid-cols-1 lg:grid-cols-3 gap-12">
			<div class="lg:col-span-2">
				<div class="flow-root">
					<ul class="-my-6 divide-y divide-gray-200 dark:divide-gray-700" role="list">
						{#each mockCart.items as { book, quantity }}
							<li class="flex py-6">
								<BookDetail {book} {quantity} />
							</li>
						{/each}
					</ul>
				</div>
			</div>
			<div class="lg:col-span-1">
				<div class="rounded-xl bg-white dark:bg-card-dark/50 shadow-md p-6 dark:shadow-md">
					<OrderSummary {total_price} shipping_price={mockCart.shipping_price} />
					<div class="mt-6">
						<button
							class="w-full flex items-center justify-center rounded-lg border border-transparent bg-primary px-6 py-3 text-base font-bold text-white shadow-sm hover:bg-primary/90"
						>
							Proceed to Checkout
						</button>
					</div>
				</div>
			</div>
		</div>
		<!-- Empty Cart State -->
		<!--
          <div class="text-center py-20">
            <span class="material-symbols-outlined text-6xl text-gray-400 dark:text-gray-500">shopping_cart</span>
            <h3 class="mt-4 text-xl font-semibold text-gray-900 dark:text-white">Your cart is currently empty</h3>
            <p class="mt-2 text-base text-gray-500 dark:text-gray-400">Looks like you haven't added any books yet.</p>
            <div class="mt-6">
              <button type="button" class="inline-flex items-center rounded-md border border-transparent bg-primary px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2">
                Start Shopping
              </button>
            </div>
          </div>
          -->
	</div>
</main>
