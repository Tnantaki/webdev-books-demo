import { cartAPI } from '$lib/api';
import { AppError } from '$lib/types';
import type { CartResult } from '$lib/types/cart';

class CartStore {
	isLoading = $state(false);

	async addToCart(bookId: string, quantity: number): Promise<CartResult> {
		this.isLoading = true;

		try {
			await cartAPI.addToCart(bookId, quantity);
			return { success: true };
		} catch (error: unknown) {
			if (error instanceof AppError) {
				const { message, errors } = error;
				return { success: false, message, errors };
			}
			return { success: false, message: 'An error occurred' };
		} finally {
			this.isLoading = false;
		}
	}
}

export const cartStore = new CartStore();
