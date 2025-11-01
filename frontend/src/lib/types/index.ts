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
	price_in_pound: number;
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

export interface BookFilterParams {
	page: number;
	per_page: number;
	sort_by: 'title' | 'genre' | 'price' | 'rating';
	order: 'asc' | 'desc';
	genre?: string;
}

export interface BookRating {
	id: string;
	user_id: string;
	rating: number;
	review?: string;
	created_at: Date;
}

export interface ApiError {
	error: string;
	field?: any; // Got on validation error
}
