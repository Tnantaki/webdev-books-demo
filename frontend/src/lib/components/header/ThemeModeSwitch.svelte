<script lang="ts">
	import { Moon, Sun } from '@lucide/svelte';
	import { toggleMode } from 'mode-watcher';
</script>

<button
	aria-label="Toggle dark mode"
	class="flex items-center justify-center p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary transition-colors"
	onclick={toggleMode}
	type="button"
>
	<Sun class="hidden dark:block" />
	<Moon class="block dark:hidden" />
</button>
