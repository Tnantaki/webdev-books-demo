<script lang="ts">
	interface Props {
		quantity: number | '';
		maxAvailable: number;
		reachMaxQuantity?: boolean;
	}

	let {
		quantity = $bindable(),
		maxAvailable,
		reachMaxQuantity = $bindable(false)
	}: Props = $props();

	const decrease = () => {
		if (typeof quantity === 'number' && quantity > 1) {
			quantity--;
			reachMaxQuantity = false;
		}
	};

	const increase = () => {
		if (typeof quantity === 'number' && quantity < maxAvailable) {
			quantity++;
		} else {
			reachMaxQuantity = true;
		}
	};

	function handleInputChange(event: Event & { currentTarget: HTMLInputElement }) {
		const value = event.currentTarget.value;
		reachMaxQuantity = false;

		// Allow empty input for better UX while typing
		if (value === '') {
			quantity = '';
			return;
		}

		// Convert to number
		const num = parseInt(value, 10);

		// Validate input
		if (isNaN(num)) {
			quantity = '';
			return;
		}

		if (num > maxAvailable) {
			quantity = maxAvailable;
			reachMaxQuantity = true;
		} else {
			quantity = num;
		}
	}

	// trigger when user click outside of input field
	function handleOnBlur() {
		// Reset to minimum if empty on blur
		if (quantity === '') {
			quantity = 1;
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
		disabled={(quantity as number) <= 1}>-</button
	>
	<input
		class="w-12 text-lg py-1 text-center border-0 bg-transparent focus:ring-0 text-text-light dark:text-text-dark"
		type="text"
		inputmode="numeric"
		bind:value={quantity}
		oninput={handleInputChange}
		onblur={handleOnBlur}
		onkeydown={handleKeyDown}
	/>
	<button
		class="px-3 py-2 font-medium dark:bg-gray-800 text-text-light dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-r-lg cursor-pointer"
		onclick={increase}>+</button
	>
</div>
