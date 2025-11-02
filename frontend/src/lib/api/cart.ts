import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';
import type { AddCartItem } from '$lib/types/cart';

export const cartAPI = {
	async addToCart(bookId: string, quantity: number): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/cart/item`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include',
			body: JSON.stringify({
				book_id: bookId,
				quantity
			} as AddCartItem)
		});
		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
	}
};
