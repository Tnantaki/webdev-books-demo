import { cartAPI } from '$lib/api';
import type { Cart, CartResult } from '$lib/types/cart';
import { AppStore } from './app.svelte';

class CartStore extends AppStore {
	cart = $state<Cart | null>(null);

	async loadCart(fetch: typeof window.fetch = window.fetch) {
		return this.execute(async () => {
			this.cart = await cartAPI.getCart(fetch);

			return { success: true };
		});
	}

	async addToCart(bookId: string, quantity: number): Promise<CartResult> {
		return this.execute(async () => {
			await cartAPI.addToCart(bookId, quantity);
			await this.loadCart(); // Refresh cart after adding

			return { success: true };
		});
	}

	async removeItemFromCart(id: string): Promise<CartResult> {
		return this.execute(async () => {
			await cartAPI.removeItemFromCart(id);
			await this.loadCart(); // Refresh cart after adding

			return { success: true };
		});
	}

	async editCartItem(id: string, quantity: number): Promise<CartResult> {
		return this.execute(async () => {
			await cartAPI.editCartItem(id, quantity);
			await this.loadCart(); // Refresh cart after adding

			return { success: true };
		});
	}

	async checkout(): Promise<CartResult> {
		return this.execute(async () => {
			await cartAPI.checkout();
			await this.loadCart();

			return { success: true };
		});
	}

	getBookQuantity(bookId: string) {
		if (!this.cart?.items) return 0;

		const item = this.cart.items.find((item) => item.book.id === bookId);
		return item?.quantity || 0;
	}
}

export const cartStore = new CartStore();
