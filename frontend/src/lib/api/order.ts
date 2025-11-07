import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';

export const orderAPI = {
	async pay(order_id: string): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/orders/${order_id}/pay`, {
			method: 'PUT',
			credentials: 'include'
		});

		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
	}
};
