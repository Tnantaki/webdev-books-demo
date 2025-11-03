import { cartAPI } from '$lib/api';
import { AppError } from '$lib/types';
import type { Cart, CartResult } from '$lib/types/cart';

class CartStore {
	cart = $state<Cart | null>(null);
	isLoading = $state(false);
	error = $state<string | null>(null);

	async loadCart() {
		this.isLoading = true;
		this.error = null;

		try {
			this.cart = await cartAPI.getCart();
			return { success: true };
		} catch (error: unknown) {
			if (error instanceof AppError) {
				this.error = error.message;
			} else {
				this.error = 'An error occurred';
			}
			return { success: false };
		} finally {
			this.isLoading = false;
		}
	}

	async addToCart(bookId: string, quantity: number): Promise<CartResult> {
		this.isLoading = true;
		this.error = null;

		try {
			await cartAPI.addToCart(bookId, quantity);
			await this.loadCart(); // Refresh cart after adding
			return { success: true };
		} catch (error: unknown) {
			if (error instanceof AppError) {
				this.error = error.message;
			} else {
				this.error = 'An error occurred';
			}
			return { success: false };
		} finally {
			this.isLoading = false;
		}
	}

	async removeItemFromCart(id: string): Promise<CartResult> {
		this.isLoading = true;

		try {
			await cartAPI.removeItemFromCart(id);
			await this.loadCart(); // Refresh cart after adding
			return { success: true };
		} catch (error: unknown) {
			if (error instanceof AppError) {
				this.error = error.message;
			} else {
				this.error = 'An error occurred';
			}
			return { success: false };
		} finally {
			this.isLoading = false;
		}
	}

	async editCartItem(id: string, quantity: number): Promise<CartResult> {
		this.isLoading = true;

		try {
			await cartAPI.editCartItem(id, quantity);
			console.log("edit item success")
			await this.loadCart(); // Refresh cart after adding
			return { success: true };
		} catch (error: unknown) {
			if (error instanceof AppError) {
				this.error = error.message;
			} else {
				this.error = 'An error occurred';
			}
			return { success: false };
		} finally {
			this.isLoading = false;
		}
	}

	getBookQuantity(bookId: string) {
		if (!this.cart?.items) return 0;

		const item = this.cart.items.find((item) => item.book.id === bookId);
		return item?.quantity || 0;
	}
}

export const cartStore = new CartStore();
