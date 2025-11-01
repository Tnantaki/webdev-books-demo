<script lang="ts">
	import { cn } from '$lib/utils';
	import type { ClassValue, HTMLInputAttributes } from 'svelte/elements';
	interface Props extends HTMLInputAttributes {
		label: string;
		name: string;
		value: string;
		error?: string;
		class?: ClassValue;
	}

	let { label, name, class: className, value = $bindable(), error, ...restProps }: Props = $props();
</script>

<label
	class="block text-sm sm:text-base font-medium text-text-light dark:text-text-dark pl-0.5"
	for={name}>{label}</label
>
<input
	class={cn(
		'focus:ring-primary focus:border-primary focus:outline-none bg-background-light dark:bg-background-dark dark:text-text-dark text-text-light flex w-full min-w-0 flex-1 overflow-hidden rounded-lg border border-gray-300 px-3 py-2 sm:text-base leading-normal font-normal placeholder:text-muted-light focus:ring-1 focus:outline-0 dark:border-gray-600 dark:placeholder:text-muted-dark mt-0.5 text-sm',
		error && 'border-error dark:border-error',
		className
	)}
	{name}
	bind:value
	{...restProps}
/>
{#if error}
	<span class="block text-error text-sm mt-1 px-0.5">{error}</span>
{/if}
