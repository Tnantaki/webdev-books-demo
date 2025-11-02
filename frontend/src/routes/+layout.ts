import { authStore } from '$lib/store/auth.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
	console.log('load layout');
	// Initialize auth state when app loads
	await authStore.initialize();
	console.log(authStore.user);
};
