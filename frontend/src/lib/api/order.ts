import { apiClient } from './apiClient';

export const orderAPI = {
	async pay(order_id: string): Promise<void> {
		return apiClient.put(`/orders/${order_id}/pay`, undefined, { credentials: 'include' });
	}
};
