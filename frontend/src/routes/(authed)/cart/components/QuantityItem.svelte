<script lang="ts">
	import { Alert } from '$lib/components';
	import { X } from '@lucide/svelte';

	interface Props {
		quantity: number | '';
		maxAvailable: number;
		handleQuantity: (value: number) => void;
	}

	let { quantity, maxAvailable, handleQuantity }: Props = $props();

	let reachMaxQuantity = $state(false);
	let currentQuantity = $derived(quantity);

	function displayReachMaxQuantity() {
		reachMaxQuantity = true;
		setTimeout(() => (reachMaxQuantity = false), 3000);
	}

	const decrease = () => {
		if (typeof currentQuantity === 'number' && currentQuantity > 1) {
			handleQuantity(currentQuantity - 1);
		}
	};
	const increase = () => {
		if (typeof currentQuantity === 'number' && currentQuantity < maxAvailable) {
			handleQuantity(currentQuantity + 1);
		} else {
			displayReachMaxQuantity();
		}
	};

	// real-time updates when typing input
	function handleInputChange(event: Event & { currentTarget: HTMLInputElement }) {
		const value = event.currentTarget.value;

		// Allow empty input for better UX while typing
		if (value === '') {
			currentQuantity = '';
			return;
		}

		// Convert to number
		const num = parseInt(value, 10);

		// Validate input
		if (isNaN(num)) {
			currentQuantity = '';
			return;
		}

		currentQuantity = num;
	}

	// trigger when value of an input element changes and the element loses focus
	function handleOnChange() {
		if (typeof currentQuantity === 'number' && currentQuantity > maxAvailable) {
			currentQuantity = maxAvailable;
			displayReachMaxQuantity();
		}
		handleQuantity(currentQuantity as number);
	}

	// trigger when user click outside of input field
	function handleOnBlur() {
		// Reset to original quantity
		if (currentQuantity === '') {
			currentQuantity = quantity;
		}
	}

	function handleKeyDown(event: KeyboardEvent) {
		// Blur input when Enter is pressed
		if (event.key === 'Enter') {
			(event.target as HTMLInputElement).blur();
		}
	}
</script>

<div class="flex items-center border border-gray-300 dark:border-gray-600 rounded-lg h-fit">
	<button
		class="px-3 py-2 font-medium dark:bg-gray-800 text-text-light dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-l-lg disabled:cursor-default disabled:hover:bg-transparent disabled:text-gray-300 dark:disabled:text-gray-500 disabled:dark:bg-transparent cursor-pointer"
		onclick={decrease}
		disabled={(currentQuantity as number) <= 1}>-</button
	>
	<input
		class="w-12 text-lg py-1 text-center border-0 bg-transparent focus:ring-0 text-text-light dark:text-text-dark"
		type="text"
		inputmode="numeric"
		bind:value={quantity}
		oninput={handleInputChange}
		onblur={handleOnBlur}
		onkeydown={handleKeyDown}
		onchange={handleOnChange}
	/>
	<button
		class="px-3 py-2 font-medium dark:bg-gray-800 text-text-light dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-r-lg cursor-pointer"
		onclick={increase}>+</button
	>
</div>

<Alert bind:open={reachMaxQuantity}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-red-100 dark:bg-red-900/50"
		>
			<X class="text-red-500 dark:text-red-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-error">
			You have reached the maximum quantity available for this item
		</p>
	{/snippet}
</Alert>
