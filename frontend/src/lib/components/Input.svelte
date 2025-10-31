<script lang="ts">
	import { cn } from '$lib/utils';
	import type { ClassValue, HTMLInputAttributes } from 'svelte/elements';
	interface Props extends HTMLInputAttributes {
		label: string;
		name: string;
		value: string;
		class?: ClassValue;
	}

	let { label, name, class: className, value = $bindable(), ...restProps }: Props = $props();
</script>

<label
	class="block text-sm sm:text-base font-medium text-muted-light dark:text-muted-dark"
	for={name}>{label}</label
>
<input
	class={cn(
		'focus:ring-primary focus:border-primary focus:outline-none bg-background-light dark:bg-background-dark dark:text-text-dark text-text-light flex w-full min-w-0 flex-1 overflow-hidden rounded-lg border border-gray-300 px-3 py-2 sm:text-base leading-normal font-normal placeholder:text-muted-light focus:ring-2 focus:outline-0 dark:border-gray-600 dark:placeholder:text-muted-dark mt-1 text-sm',
		className
	)}
	{name}
	bind:value
	{...restProps}
/>
