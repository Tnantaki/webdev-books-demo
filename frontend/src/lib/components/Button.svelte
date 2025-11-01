<script lang="ts">
	import { cn } from '$lib/utils';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let { class: className, children, ...restProps }: HTMLButtonAttributes = $props();
</script>

<button
	class={cn(
		'flex w-full justify-center rounded-lg border border-transparent bg-primary py-2.5 px-4 text-sm font-medium text-white hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 dark:focus:ring-offset-background-dark sm:text-base cursor-pointer',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</button>
