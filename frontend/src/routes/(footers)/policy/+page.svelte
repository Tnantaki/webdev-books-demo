<script lang="ts">
	import { policies } from './data';
</script>

<div class="flex flex-col gap-8 max-w-5xl mx-auto">
	<div class="flex flex-wrap justify-between gap-3 p-4">
		<div class="flex min-w-72 flex-col gap-3">
			<p
				class="text-text-light dark:text-slate-50 text-4xl font-black leading-tight tracking-[-0.033em]"
			>
				Privacy Policy
			</p>
			<p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal">
				Last updated: October 26, 2023
			</p>
		</div>
	</div>
	<div class="space-y-8 p-4">
		<div>
			<p class="text-slate-700 dark:text-slate-300 text-base font-normal leading-relaxed pb-3 pt-1">
				Your privacy is important to us. It is Bookstore's policy to respect your privacy regarding
				any information we may collect from you across our website, and other sites we own and
				operate. We are committed to protecting your personal data and ensuring that you have a
				secure and transparent experience.
			</p>
		</div>
		{#each policies as policy}
			<div>
				<h2
					class="text-text-light dark:text-slate-50 text-[22px] font-bold leading-tight tracking-[-0.015em] pb-3 pt-5"
				>
					{policy.title}
				</h2>
				<p
					class="text-slate-700 dark:text-slate-300 text-base font-normal leading-relaxed pb-3 pt-1"
				>
					{policy.description}
				</p>
			</div>
		{/each}
	</div>
</div>
