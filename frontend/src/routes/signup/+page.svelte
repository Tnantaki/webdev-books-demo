<script lang="ts">
	import { goto } from '$app/navigation';
	import { Input } from '$lib/components';
	import { authStore } from '$lib/store/auth.svelte';
	import type { SignupCredentials } from '$lib/types/auth';

	let formInput = $state({ email: '', name: '', password: '', confirmPassword: '' });
	let error = $state('');

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';
		const credencials = {
			email: formInput.email,
			password: formInput.password,
			confirmPassword: formInput.confirmPassword
		} satisfies SignupCredentials;

		const result = await authStore.signup(credencials);
		if (result.success) {
			goto('/login');
		} else {
			error = result.error || 'An error occured';
		}
	}
</script>

<div class="w-full max-w-md space-y-8 mx-auto py-8">
	<div>
		<div class="flex items-center justify-center gap-2">
			<span class="icon-[material-symbols--menu-book] text-primary text-4xl"></span>
			<h2 class="text-center text-3xl font-bold tracking-tight">Create your account</h2>
		</div>
	</div>
	<form onsubmit={handleSubmit} class="mt-8 space-y-6">
		<div class="rounded-xl bg-card-light dark:bg-card-dark p-8 shadow-lg">
			<div class="space-y-6">
				<div>
					<Input
						label="Name"
						id="name"
						name="name"
						bind:value={formInput.name}
						autocomplete="name"
						placeholder="e.g. Jane Doe"
						type="text"
						required
					/>
				</div>
				<div>
					<Input
						label="Email address"
						id="email"
						name="email"
						bind:value={formInput.email}
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
						bind:value={formInput.password}
						autocomplete="new-password"
						placeholder="Min. 8 characters"
						type="password"
						required
					/>
				</div>
				<div>
					<Input
						label="Confirm Password"
						id="confirm-password"
						name="confirm-password"
						bind:value={formInput.confirmPassword}
						autocomplete="new-password"
						placeholder="Re-enter your password"
						type="password"
						required
					/>
				</div>
			</div>
			<div class="mt-8">
				<button
					class="group relative flex w-full justify-center rounded-lg border border-transparent bg-primary py-2 px-4 text-sm font-medium text-white hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 dark:focus:ring-offset-background-dark"
					type="submit"
				>
					{#if authStore.isLoading}
						Loading...
					{:else}
						Create Account
					{/if}
				</button>
			</div>
		</div>
		<div class="text-center text-sm sm:text-base">
			<a class="font-medium text-primary hover:text-primary/90" href="/login">
				Already have an account? Login
			</a>
		</div>
	</form>
</div>
