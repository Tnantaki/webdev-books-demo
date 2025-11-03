import { authStore } from '$lib/store/auth.svelte';
import { cartStore } from '$lib/store/cart.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
	console.log('load layout');
	// Initialize auth state when app loads
	await authStore.initialize();

	// Only load cart if user is logged in
	if (authStore.user) {
		console.log(authStore.user); // DEBUG

		await cartStore.loadCart();
	}
};
