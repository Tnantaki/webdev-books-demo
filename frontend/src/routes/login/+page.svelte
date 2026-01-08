<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Alert, Button, Input } from '$lib/components';
	import { authStore } from '$lib/store/auth.svelte';
	import { CircleAlert } from '@lucide/svelte';

	let form = $state({ email: '', password: '' });

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();

		const result = await authStore.login(form.email, form.password);
		if (result.success) {
			const redirectTo = page.url.searchParams.get('redirectTo') ?? '/';
			await goto(redirectTo, { replaceState: true });
		}
	}
	
	let googleButton = $state(false);
</script>

<div class="max-w-md w-full mx-auto py-12">
	<div class="bg-card-light dark:bg-card-dark rounded-xl shadow-lg p-8">
		<div class="text-center mb-8">
			<h2 class="text-3xl font-bold text-primary">Welcome Back!</h2>
			<p class="text-muted-light dark:text-muted-dark mt-2">Sign in to continue to Bookstore.</p>
		</div>
		<form onsubmit={handleSubmit} class="space-y-6">
			<div>
				<Input
					label="Email address"
					id="email"
					name="email"
					bind:value={form.email}
					autocomplete="email"
					placeholder="you@example.com"
					type="email"
					required
				/>
			</div>
			<div>
				<Input
					label="Password"
					id="password"
					name="password"
					bind:value={form.password}
					autocomplete="current-password"
					placeholder="••••••••"
					type="password"
					required
				/>
			</div>
			{#if authStore.error}
				<div class="text-sm font-medium text-error pl-1 mb-2">
					{authStore.error}
				</div>
			{/if}
			<div>
				<Button type="submit">
					{#if authStore.isLoading}
						Loading...
					{:else}
						Login
					{/if}
				</Button>
			</div>
		</form>
		<div class="mt-6 relative">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-gray-300 dark:border-gray-600"></div>
			</div>
			<div class="relative flex justify-center text-sm">
				<span class="px-2 bg-card-light dark:bg-card-dark text-muted-light dark:text-muted-dark"
					>Or continue with</span
				>
			</div>
		</div>
		<div class="mt-6">
			<button
				class="w-full flex items-center justify-center py-2.5 px-4 border border-gray-300 dark:border-gray-700 rounded-lg shadow-sm text-sm sm:text-base font-medium text-text-light dark:text-text-dark bg-card-light dark:bg-card-dark hover:bg-background-light dark:hover:bg-background-dark focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary cursor-pointer"
				onclick={() => googleButton = true}
			>
				<svg
					class="size-7 mr-3"
					fill="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					><path
						d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
						fill="#4285F4"
					></path><path
						d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
						fill="#34A853"
					></path><path
						d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
						fill="#FBBC05"
					></path><path
						d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
						fill="#EA4335"
					></path><path d="M1 1h22v22H1z" fill="none"></path></svg
				>
				<span>Login with Google</span>
			</button>
		</div>
		<div class="text-center mt-6">
			<p class="text-sm sm:text-base text-muted-light dark:text-muted-dark">
				Don’t have an account?
				<a class="font-medium text-primary hover:text-primary/90 cursor-pointer" href="/signup">
					Register
				</a>
			</p>
		</div>
	</div>
</div>

<Alert bind:open={googleButton}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-amber-100 dark:bg-amber-800/50"
		>
			<CircleAlert class="text-amber-500 dark:text-amber-400 size-7" />
		</div>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-lg text-accent">
			Sorry, This feature are not support yet
		</p>
	{/snippet}
</Alert>