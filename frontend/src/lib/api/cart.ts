import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';
import type { AddCartItem, Cart, EditCartItem } from '$lib/types/cart';

export const cartAPI = {
	async getCart(): Promise<Cart> {
		const res = await fetch(`${PUBLIC_API_BASE}/cart`, {
			credentials: 'include'
		});
		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
		return data;
	},

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
	},

	async removeItemFromCart(id: string): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/cart/item/${id}`, {
			method: 'DELETE',
			credentials: 'include'
		});
		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
	},

	async editCartItem(id: string, quantity: number): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/cart/item/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include',
			body: JSON.stringify({
				quantity
			} as EditCartItem)
		});
		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
	}
};
