<script lang="ts">
	let { value = $bindable(), available } = $props();

	const decrease = () => {
		if (value > 1) {
			value--;
		}
	};
	const increase = () => {
		if (value <= available) {
			value++;
		}
	};
</script>

<div class="flex items-center border border-gray-300 dark:border-gray-600 rounded-lg h-fit">
	<button
		class="px-3 py-2 font-medium dark:bg-gray-800 text-text-light dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-800 rounded-l-lg disabled:cursor-default disabled:hover:bg-transparent disabled:text-gray-300 dark:disabled:text-gray-500 disabled:dark:bg-transparent cursor-pointer"
		onclick={decrease}
		disabled={value <= 1}>-</button
	>
	<input
		class="w-12 text-lg py-1 text-center border-0 bg-transparent focus:ring-0 text-text-light dark:text-text-dark"
		type="text"
		bind:value={
			() => value,
			(v) => {
				value = v > available ? available : v;
			}
		}
	/>
	<button
		class="px-3 py-2 font-medium dark:bg-gray-800 text-text-light dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-r-lg cursor-pointer"
		onclick={increase}>+</button
	>
</div>

<!-- <p>
	You have reached the maximum quantity available for this item
</p> -->
