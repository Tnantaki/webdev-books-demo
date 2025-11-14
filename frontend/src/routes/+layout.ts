import { PUBLIC_API_BASE } from '$env/static/public';
import { authStore } from '$lib/store/auth.svelte';
import { cartStore } from '$lib/store/cart.svelte';
import { error } from '@sveltejs/kit';
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

	// request book genre
	const response_genre = await fetch(`${PUBLIC_API_BASE}/books/genre`);

	if (!response_genre.ok) {
		throw error(response_genre.status, `Failed to fetch books: ${response_genre.statusText}`);
	}

	const genres: string[] = await response_genre.json();
	
	return { genres };
};
