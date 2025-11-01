<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button, Input } from '$lib/components';
	import Popup from '$lib/components/Popup.svelte';
	import { authStore } from '$lib/store/auth.svelte';
	import type { SignupCredentials } from '$lib/types/auth';
	import { Check } from '@lucide/svelte';

	let form = $state({ email: '', name: '', password: '', confirmPassword: '' });
	let isOpenPopup = $state(false);
	let error = $state();
	let fieldErrors = $state<Record<string, string>>({});

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';
		fieldErrors = {};

		// Client-side validation
		if (form.password !== form.confirmPassword) {
			fieldErrors['confirm_password'] = 'Passwords do not match';
			return;
		}

		if (form.password.length < 8) {
			fieldErrors['password'] = 'Password must be at least 8 characters';
			return;
		}

		const credencials = {
			email: form.email,
			password: form.password,
			confirmPassword: form.confirmPassword
		} satisfies SignupCredentials;

		const result = await authStore.signup(credencials);
		if (result.success) {
			isOpenPopup = true;
		} else {
			if (result.errors) {
				result.errors.map((error) => (fieldErrors[error.field] = error.message));
			} else {
				error = result.message || 'An error occured';
			}
		}
	}

	function handlePopupClose() {
		isOpenPopup = false;
		goto('/login');
	}

	function getFieldError(field: string): string {
		return fieldErrors[field] || '';
	}
</script>

<div class="w-full max-w-md space-y-8 mx-auto pt-8 pb-4">
	<div>
		<div class="flex items-center justify-center gap-2">
			<span class="icon-[material-symbols--menu-book] text-primary text-4xl"></span>
			<h2 class="text-center text-3xl font-bold tracking-tight">Create your account</h2>
		</div>
	</div>
	<form onsubmit={handleSubmit} class="mt-8 space-y-6">
		<div class="rounded-xl bg-card-light dark:bg-card-dark p-8 shadow-lg">
			<div class="space-y-5">
				<div>
					<Input
						label="Name"
						id="name"
						name="name"
						bind:value={form.name}
						autocomplete="name"
						placeholder="e.g. Jane Doe"
						type="text"
						error={getFieldError('name')}
						required
					/>
				</div>
				<div>
					<Input
						label="Email address"
						id="email"
						name="email"
						bind:value={form.email}
						autocomplete="email"
						placeholder="you@example.com"
						type="email"
						error={getFieldError('email')}
						required
					/>
				</div>
				<div>
					<Input
						label="Password"
						id="password"
						name="password"
						bind:value={form.password}
						autocomplete="new-password"
						placeholder="Min. 8 characters"
						type="password"
						error={getFieldError('password')}
						required
					/>
				</div>
				<div>
					<Input
						label="Confirm Password"
						id="confirm-password"
						name="confirm-password"
						bind:value={form.confirmPassword}
						autocomplete="new-password"
						placeholder="Re-enter your password"
						type="password"
						error={getFieldError('confirm_password')}
						required
					/>
				</div>
			</div>
			<div class="mt-8">
				{#if error}
					<div class="text-sm font-medium text-error pl-1 mb-2">
						{error}
					</div>
				{/if}
				<Button type="submit" disabled={authStore.isLoading}>
					{#if authStore.isLoading}
						Loading...
					{:else}
						Create Account
					{/if}
				</Button>
			</div>
		</div>
		<div class="text-center text-sm sm:text-base">
			<a class="font-medium text-primary hover:text-primary/90" href="/login">
				Already have an account? Login
			</a>
		</div>
	</form>
</div>

<Popup bind:open={isOpenPopup}>
	{#snippet title()}
		<div
			class="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/50"
		>
			<Check class="text-green-500 dark:text-green-400 size-7" />
		</div>
		<h3 class="mt-4 text-xl font-bold">Account Created Successfully!</h3>
	{/snippet}

	{#snippet description()}
		<p class="mt-4 text-sm text-muted-light dark:text-muted-dark">
			Welcome to Bookworm. You can now log in to start your journey.
		</p>
		<div class="mt-8">
			<button
				class="w-full justify-center rounded-lg border border-transparent bg-primary py-2 px-4 text-sm font-medium text-white hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 dark:focus:ring-offset-card-dark cursor-pointer"
				type="button"
				onclick={handlePopupClose}
			>
				Continue to Login
			</button>
		</div>
	{/snippet}
</Popup>
