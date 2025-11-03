// import { PUBLIC_API_BASE } from '$env/static/public';
// import { error } from '@sveltejs/kit';
// import type { PageLoad } from './$types';
// import type { Cart } from '$lib/types/cart';

// export const load: PageLoad = async ({ fetch }) => {
// 	const res = await fetch(`${PUBLIC_API_BASE}/cart`, {
// 		credentials: 'include'
// 	});

// 	if (!res.ok) {
// 		if (res.status == 404) {
// 			return {};
// 		}
// 		throw error(res.status, `Failed to fetch data: ${res.statusText}`);
// 	}

// 	const cart: Cart = await res.json();
// 	if (!cart) {
// 		throw error(500, 'Failed to deserialize json');
// 	}

// 	return { cart };
// };
