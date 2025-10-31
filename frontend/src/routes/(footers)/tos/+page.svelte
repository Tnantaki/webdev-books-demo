<script lang="ts">
	import { ArrowBigUpDash, Download, Printer } from '@lucide/svelte';
	import { tos_sessions } from './data';

	let printContent: HTMLElement;
	let showButton = $state(false);

	function printSection() {
		const originalContent = document.body.innerHTML;

		document.body.innerHTML = printContent.innerHTML;
		window.print();
		document.body.innerHTML = originalContent;

		// Reload to restore event listeners
		window.location.reload();
	}

	function handleScroll() {
		showButton = window.scrollY > 60;
	}

	function scrollToTop() {
		window.scrollTo({
			top: 0,
			behavior: 'smooth'
		});
	}
</script>

<svelte:window onscroll={handleScroll} />

<div class="w-full flex flex-col lg:flex-row gap-8">
	<aside class="w-full lg:w-1/4 lg:sticky top-8 self-start">
		<div class="p-6 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
			<h3 class="text-lg font-bold text-text-light dark:text-slate-50 mb-4">Table of Contents</h3>
			<ul class="space-y-3">
				{#each tos_sessions as session, idx}
					<li>
						<a
							class={idx === 0
								? 'font-semibold hover:underline text-primary text-sm'
								: 'text-gray-600 dark:text-gray-300 hover:text-primary dark:hover:text-primary font-medium text-sm'}
							href={`#${session.id}`}>{session.header}</a
						>
					</li>
				{/each}
			</ul>

			<div
				class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700 flex flex-col sm:flex-row gap-3"
			>
				<button
					class="flex-1 flex items-center justify-center gap-1 min-w-[84px] cursor-pointer overflow-hidden rounded-lg h-10 px-2 bg-primary/20 text-primary text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/30"
					onclick={printSection}
				>
					<Printer size={20} />
					<span class="truncate">Print</span>
				</button>
				<a
					class="flex-1 flex items-center justify-center gap-1 min-w-[84px] cursor-pointer overflow-hidden rounded-lg h-10 px-2 bg-primary/20 text-primary text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/30"
					href="./Book_Store_TOR.pdf"
					type="application/octet-stream"
					download="Book Store Demo TOR"
				>
					<Download size={20} />
					<span class="truncate">Download</span>
				</a>
			</div>
		</div>
	</aside>
	<div class="w-full lg:w-3/4">
		<div
			class="bg-white dark:bg-gray-800 p-8 sm:p-12 rounded-lg shadow-sm"
			bind:this={printContent}
		>
			<div class="flex flex-wrap justify-between items-start gap-4 mb-8">
				<div class="flex flex-col gap-2">
					<h1
						class="text-text-light dark:text-slate-50 text-4xl font-black leading-tight tracking-[-0.033em]"
					>
						Terms of Service
					</h1>
					<p class="text-gray-500 dark:text-gray-400 text-base font-normal leading-normal">
						Last updated: October 26, 2023
					</p>
				</div>
			</div>
			<article
				class="prose prose-lg dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 tos-article"
			>
				{#each tos_sessions as { id, title, description, lists, link }}
					<section class="mb-8" {id}>
						{#if title}
							<h2
								class="text-text-light dark:text-text-dark mb-4 border-b border-gray-200 pb-2 text-[22px] leading-tight font-bold tracking-[-0.015em] dark:border-gray-700"
							>
								{title}
							</h2>
						{/if}
						<p class="text-base font-normal leading-relaxed mb-4">{description}</p>
						{#if link}
							<a class="text-primary hover:underline" href={link.href}>{link.title}</a>
						{/if}
						{#if lists}
							<ul class="list-disc pl-6 space-y-2 text-base">
								{#each lists as list}
									<li>{list}</li>
								{/each}
							</ul>
						{/if}
					</section>
				{/each}
			</article>
		</div>
	</div>
</div>

{#if showButton}
	<button
		class="fixed bottom-8 right-8 bg-primary text-white p-3 rounded-full shadow-lg hover:bg-primary/90 transition-opacity"
		id="back-to-top"
		onclick={scrollToTop}
	>
		<ArrowBigUpDash />
	</button>
{/if}
