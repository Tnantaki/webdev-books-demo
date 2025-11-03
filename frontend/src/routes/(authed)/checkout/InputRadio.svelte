<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		id: string;
		name: string;
		children: Snippet;
	}
	let { id, name, children }: Props = $props();
</script>

<label
	class="font-medium text-text-light dark:text-text-dark flex items-center p-3 border border-gray-300 dark:border-gray-700 rounded-lg mx-2 has-checked:bg-slate-200 has-checked:ring-slate-400 has-checked:dark:bg-slate-900 has-checked:dark:ring-slate-700 ring-1 ring-transparent"
	for={id}
>
	<input
		class="form-radio h-4 w-4 text-primary border-gray-300 dark:border-gray-600 focus:ring-primary mr-3"
		{id}
		{name}
		type="radio"
	/>
	{@render children()}
</label>
