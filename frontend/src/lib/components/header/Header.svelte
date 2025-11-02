<script>
	import { authStore } from '$lib/store/auth.svelte';
	import BookSearch from './BookSearch.svelte';
	import Logo from './Logo.svelte';
	import Profile from './Profile.svelte';
	import ThemeModeSwitch from './ThemeModeSwitch.svelte';
</script>

<header class="w-full bg-card-light dark:bg-card-dark shadow-sm">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex items-center justify-between h-16">
			<Logo />
			<div class="flex">
				<BookSearch />
				<div class="hidden lg:ml-4 lg:flex lg:items-center gap-4">
					<ThemeModeSwitch />
					<!-- <a class="text-sm font-medium hover:text-primary" href="#">My Account</a> -->
					<a class="flex items-center text-sm font-medium hover:text-primary" href="/cart">
						<span class="icon-[mdi--cart-outline] mr-1 size-6"></span>
						Cart
					</a>
					<button
						class="flex-shrink-0 bg-white dark:bg-background-dark p-1 rounded-full text-muted-light dark:text-muted-dark hover:text-primary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary"
						type="button"
					>
						<span class="sr-only">View notifications</span>
						<span class="material-symbols-outlined">notifications</span>
					</button>
					{#if authStore.user}
						<Profile />
					{:else}
						<div class="flex items-center text-nowrap">
							<a class="px-2 text-base font-medium hover:text-primary" href="/signup">Sign Up</a>
							<div class="w-px bg-text-light dark:bg-text-dark h-[20px]"></div>
							<a class="px-2 text-base font-medium hover:text-primary" href="/login">Login</a>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
</header>
