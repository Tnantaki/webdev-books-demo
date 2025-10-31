<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Header } from '$lib/components/header';
	import { Footer } from '$lib/components/footer';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Bookstore - Online Bookstore Demo</title>
	<link
		href="https://fonts.googleapis.com/css2?family=Work+Sans:wght@400;500;700&amp;display=swap"
		rel="stylesheet"
	/>
	<link
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined"
		rel="stylesheet"
	/>
</svelte:head>

<ModeWatcher />

<div
	class="bg-background-light dark:bg-background-dark text-text-light dark:text-text-dark font-display flex flex-1 flex-col"
>
	<div class="relative flex min-h-screen w-full flex-col group/design-root overflow-x-hidden">
		<div class="layout-container flex h-full grow flex-col justify-between">
			<Header />

			<main class="flex-1">
				<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
					{@render children?.()}
				</div>
			</main>

			<Footer />
		</div>
	</div>
</div>
