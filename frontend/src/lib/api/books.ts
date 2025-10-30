import { PUBLIC_API_BASE } from '$env/static/public';
import type { BookFilterParams, BookPage } from '$lib/types';

export const getBooksPage = async (): Promise<BookPage> => {
	const response = await fetch(`${PUBLIC_API_BASE}/books/page`);
	if (!response.ok) {
		throw new Error(`Failed to fetch books: ${response.statusText}`);
	}
	return response.json();
};
