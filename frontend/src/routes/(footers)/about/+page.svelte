<script lang="ts">
	import { images_gallery, image_banner } from './data';
</script>

<div class="relative w-full overflow-x-hidden">
	<!-- Hero Section -->
	<div class="layout-container flex h-full grow flex-col">
		<div class="flex flex-1 justify-center">
			<div class="layout-content-container flex flex-col w-full">
				<div class="@container">
					<div class="p-0 sm:p-4">
						<div
							class="flex min-h-[480px] flex-col gap-6 bg-cover bg-center bg-no-repeat @[480px]:gap-8 items-center justify-center p-4"
							data-alt={image_banner.alt}
							style="background-image: linear-gradient(rgba(26, 35, 126, 0.5) 0%, rgba(26, 35, 126, 0.7) 100%), url(&quot;{image_banner.src}&quot;)"
						>
							<div class="flex flex-col gap-2 text-center">
								<h1
									class="text-white text-4xl font-black leading-tight tracking-[-0.033em] @[480px]:text-5xl @[480px]:font-black @[480px]:leading-tight @[480px]:tracking-[-0.033em]"
								>
									For the Love of Books
								</h1>
								<h2
									class="text-white/90 text-sm font-normal leading-normal @[480px]:text-base @[480px]:font-normal @[480px]:leading-normal"
								>
									Connecting readers with the stories that shape their lives.
								</h2>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
		<!-- Mission & Vision Section -->
		<div class="flex flex-col gap-10 px-4 py-10 @container">
			<div class="flex flex-col gap-4 text-center">
				<h1
					class="text-primary dark:text-white tracking-light text-[32px] font-bold leading-tight @[480px]:text-4xl @[480px]:font-black @[480px]:leading-tight @[480px]:tracking-[-0.033em] max-w-3xl mx-auto"
				>
					Our Mission &amp; Vision
				</h1>
				<p
					class="text-text-light/80 dark:text-text-dark/80 text-base font-normal leading-normal max-w-3xl mx-auto"
				>
					We are driven by a singular passion: to make the world's stories accessible to everyone.
					We believe in the power of books to inspire, educate, and transform lives.
				</p>
			</div>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 p-0">
				<div
					class="flex flex-1 gap-4 rounded-xl border border-primary/20 bg-background-light dark:bg-background-dark p-6 flex-col text-center items-center"
				>
					<div class="text-primary dark:text-secondary">
						<span class="material-symbols-outlined" style="font-size: 36px;">auto_stories</span>
					</div>
					<div class="flex flex-col gap-2">
						<h2 class="text-primary dark:text-white text-xl font-bold leading-tight">
							Our Mission
						</h2>
						<p
							class="text-text-light/80 dark:text-text-dark/80 text-base font-normal leading-normal"
						>
							To foster a vibrant community of readers by offering a diverse and curated selection
							of books at accessible prices.
						</p>
					</div>
				</div>
				<div
					class="flex flex-1 gap-4 rounded-xl border border-primary/20 bg-background-light dark:bg-background-dark p-6 flex-col text-center items-center"
				>
					<div class="text-primary dark:text-secondary">
						<span class="material-symbols-outlined" style="font-size: 36px;">trending_up</span>
					</div>
					<div class="flex flex-col gap-2">
						<h2 class="text-primary dark:text-white text-xl font-bold leading-tight">Our Vision</h2>
						<p
							class="text-text-light/80 dark:text-text-dark/80 text-base font-normal leading-normal"
						>
							To be the most beloved online destination for book lovers, innovating the way people
							discover and engage with literature.
						</p>
					</div>
				</div>
			</div>
		</div>
		<!-- Our Story Section -->
		<div class="px-4 py-10">
			<div class="flex flex-col gap-4 text-center mb-10">
				<h1
					class="text-primary dark:text-white tracking-light text-[32px] font-bold leading-tight @[480px]:text-4xl @[480px]:font-black @[480px]:leading-tight @[480px]:tracking-[-0.033em] max-w-3xl mx-auto"
				>
					Our Story
				</h1>
			</div>
			<div class="grid grid-cols-[auto_1fr] gap-x-4 md:gap-x-8">
				<div class="flex flex-col items-center gap-2 pt-3">
					<div
						class="text-primary dark:text-secondary bg-primary/10 dark:bg-secondary/10 p-2 rounded-full"
					>
						<span class="material-symbols-outlined">lightbulb</span>
					</div>
					<div class="w-[2px] bg-primary/20 h-full"></div>
				</div>
				<div class="flex flex-1 flex-col pb-12 pt-3">
					<p class="text-text-light/80 dark:text-text-dark/80 text-sm font-medium leading-normal">
						2018
					</p>
					<p class="text-primary dark:text-white text-lg font-bold leading-normal">
						The Idea Was Born
					</p>
				</div>
				<div class="flex flex-col items-center gap-2">
					<div class="w-[2px] bg-primary/20 h-full"></div>
					<div
						class="text-primary dark:text-secondary bg-primary/10 dark:bg-secondary/10 p-2 rounded-full"
					>
						<span class="material-symbols-outlined">groups</span>
					</div>
					<div class="w-[2px] bg-primary/20 h-full"></div>
				</div>
				<div class="flex flex-1 flex-col pb-12">
					<p class="text-text-light/80 dark:text-text-dark/80 text-sm font-medium leading-normal">
						2019
					</p>
					<p class="text-primary dark:text-white text-lg font-bold leading-normal">
						Our First 1,000 Customers
					</p>
				</div>
				<div class="flex flex-col items-center gap-2">
					<div class="w-[2px] bg-primary/20 h-full"></div>
					<div
						class="text-primary dark:text-secondary bg-primary/10 dark:bg-secondary/10 p-2 rounded-full"
					>
						<span class="material-symbols-outlined">import_contacts</span>
					</div>
					<div class="w-[2px] bg-primary/20 h-full"></div>
				</div>
				<div class="flex flex-1 flex-col pb-12">
					<p class="text-text-light/80 dark:text-text-dark/80 text-sm font-medium leading-normal">
						2021
					</p>
					<p class="text-primary dark:text-white text-lg font-bold leading-normal">
						Expanding Our Collection
					</p>
				</div>
				<div class="flex flex-col items-center gap-2">
					<div class="w-[2px] bg-primary/20 h-full"></div>
					<div
						class="text-primary dark:text-secondary bg-primary/10 dark:bg-secondary/10 p-2 rounded-full"
					>
						<span class="material-symbols-outlined">public</span>
					</div>
				</div>
				<div class="flex flex-1 flex-col">
					<p class="text-text-light/80 dark:text-text-dark/80 text-sm font-medium leading-normal">
						2023
					</p>
					<p class="text-primary dark:text-white text-lg font-bold leading-normal">Going Global</p>
				</div>
			</div>
		</div>
		<!-- Image Gallery -->
		<div class="px-4 py-10">
			<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
				{#each images_gallery as { alt, src }}
					<div>
						<img {alt} {src} class="h-auto max-w-full rounded-lg" />
					</div>
				{/each}
			</div>
		</div>
		<!-- CTA Section -->
		<div class="px-4 py-16 text-center bg-primary/5 dark:bg-primary/10 rounded-xl">
			<div class="flex flex-col gap-4">
				<h1
					class="text-primary dark:text-white tracking-light text-[32px] font-bold leading-tight @[480px]:text-4xl @[480px]:font-black @[480px]:leading-tight @[480px]:tracking-[-0.033em] max-w-3xl mx-auto"
				>
					Join Our Journey
				</h1>
				<p
					class="text-text-light/80 dark:text-text-dark/80 text-base font-normal leading-normal max-w-3xl mx-auto"
				>
					Ready to find your next favorite book? Explore our vast collection and become part of our
					story.
				</p>
				<div class="mt-4">
					<a
						class="flex w-fit min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-8 bg-accent text-white text-base font-bold leading-normal tracking-[0.015em] mx-auto hover:bg-accent/90 transition-colors"
						href="/"
					>
						<span class="truncate">Browse Our Books</span>
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
