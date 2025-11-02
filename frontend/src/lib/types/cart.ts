import type { FieldError } from '.';
import type { Book } from './book';

export interface CartItem {
	id: string;
	book_item: Book;
	quantity: number;
	updated_at: Date;
}

export interface Cart {
	items: CartItem[];
	total_price: number;
	shipping_price: number;
}

export interface AddCartItem {
	book_id: string;
	quantity: number;
}

export interface CartResult {
	success: boolean;
	message?: string
	errors?: FieldError[];
}