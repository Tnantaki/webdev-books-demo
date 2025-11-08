import { authStore } from '$lib/store/auth.svelte';
import { cartStore } from '$lib/store/cart.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch }) => {
	// Initialize auth state when app loads
	// Use SvelteKit fetch for SSR
	await authStore.initialize(fetch);

	// Only load cart if user is logged in
	if (authStore.user) {
		console.log(authStore.user); // DEBUG

		// Use SvelteKit fetch for SSR
		await cartStore.loadCart(fetch);
	}
};
