import { orderAPI } from '$lib/api';
import type { OrderResult } from '$lib/types/order';
import { AppStore } from './app.svelte';

class OrderStore extends AppStore {
	async pay(order_id: string): Promise<OrderResult> {
		return this.execute(async () => {
			await orderAPI.pay(order_id);
			
			return { success: true };
		});
	}
}

export const orderStore = new OrderStore();
