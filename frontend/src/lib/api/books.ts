import { env } from '$env/dynamic/public';
import type { BookPage } from '$lib/types/book';

export const getBooksPage = async (): Promise<BookPage> => {
	const response = await fetch(`${env.PUBLIC_API_BASE}/books/page`);
	if (!response.ok) {
		throw new Error(`Failed to fetch books: ${response.statusText}`);
	}
	return response.json();
};
