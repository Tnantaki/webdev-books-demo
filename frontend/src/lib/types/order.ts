export interface BookAtPurchase {
	book_id: string;
	title: string;
	genre: string;
	img_path: string;
	quantity: number;
	price_at_purchase: number;
}

export interface OrderDetail {
	id: string;
	total_price: number;
	order_status: string;
	created_at: string;
	updated_at: Date;
	items: BookAtPurchase[];
}
