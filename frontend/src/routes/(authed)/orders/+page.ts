import { PUBLIC_API_BASE } from '$env/static/public';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { OrderDetail } from '$lib/types/order';

export const load: PageLoad = async ({ fetch }) => {
	const res = await fetch(`${PUBLIC_API_BASE}/orders`, {
		credentials: 'include'
	});

	if (!res.ok) {
		if (res.status == 404) {
			return {};
		}
		throw error(res.status, `Failed to fetch data: ${res.statusText}`);
	}

	const orders: OrderDetail[] = await res.json();
	if (!orders) {
		throw error(500, 'Failed to deserialize json');
	}

	return { orders };
};
