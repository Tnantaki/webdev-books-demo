import { orderAPI } from '$lib/api';
import { AppError } from '$lib/types';
import type { OrderResult } from '$lib/types/order';

class OrderStore {
	isLoading = $state(false);
	error = $state<string | null>(null);

	async pay(order_id: string): Promise<OrderResult> {
		this.isLoading = true;

		try {
			await orderAPI.pay(order_id);
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
}

export const orderStore = new OrderStore();
