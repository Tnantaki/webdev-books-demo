export interface BookPagination {
	current_page: number;
	per_page: number;
	total_items: number;
	total_pages: number;
	has_next: boolean;
	has_previous: boolean;
}

export interface Book {
	id: string;
	title: string;
	genre: string;
	description: string;
	price: number;
	available: number;
	img_path: string;
	average_rating: number;
	total_ratings: number;
	created_at: Date;
	updated_at: Date;
}

export interface BookPage {
	pagination: BookPagination;
	books: Book[];
}

export type BookSort = 'title' | 'genre' | 'price' | 'average_rating';
export type BookOrder = 'asc' | 'desc';

export interface BookFilterParams {
	page: number;
	per_page: number;
	sort_by: BookSort;
	order: BookOrder;
	genre?: string;
}

export interface BookRating {
	id: string;
	user_id: string;
	rating: number;
	review?: string;
	created_at: Date;
}
