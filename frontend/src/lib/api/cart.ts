import type { AddCartItem, Cart, EditCartItem } from '$lib/types/cart';
import { apiClient } from './apiClient';

export const cartAPI = {
	async getCart(fetch: typeof window.fetch): Promise<Cart> {
		return apiClient.get('/cart', { credentials: 'include' }, fetch);
	},

	async addToCart(bookId: string, quantity: number): Promise<void> {
		const body = { book_id: bookId, quantity } as AddCartItem;
		return apiClient.post('/cart/item', body, { credentials: 'include' });
	},

	async removeItemFromCart(id: string): Promise<void> {
		return apiClient.delete(`/cart/item/${id} `, { credentials: 'include' });
	},

	async editCartItem(id: string, quantity: number): Promise<void> {
		const body = { quantity } as EditCartItem;
		return apiClient.put(`/cart/item/${id} `, body, { credentials: 'include' });
	},

	async checkout(): Promise<void> {
		return apiClient.post('/cart/checkout', undefined, { credentials: 'include' });
	}
};
