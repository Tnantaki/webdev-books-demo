<script lang="ts">
	import type { ClassValue } from 'svelte/elements';

	interface StarProps {
		class?: ClassValue;
		rating: number;
	}

	let props: StarProps = $props();

	let star_styles = [];

	for (let i = 0; i < 5; i++) {
		let result = props.rating - i;
		if (result > 1) {
			star_styles.push('icon-[icomoon-free--star-full] text-accent');
		} else if (result <= 0) {
			star_styles.push('icon-[icomoon-free--star-empty] text-gray-300 dark:text-gray-600');
		} else {
			if (result < 0.25) {
				star_styles.push('icon-[icomoon-free--star-empty] text-gray-300 dark:text-gray-600');
			} else if (result < 0.75) {
				star_styles.push('icon-[icomoon-free--star-half] text-accent');
			} else {
				star_styles.push('icon-[icomoon-free--star-full] text-accent');
			}
		}
	}
</script>

<div class={['flex items-center', props.class]}>
	{#each star_styles as style}
		<span class="size-5 {style}"></span>
	{/each}
</div>
