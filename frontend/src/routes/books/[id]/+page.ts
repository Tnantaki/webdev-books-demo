import { PUBLIC_API_BASE } from '$env/static/public';
import type { Book, BookPage } from '$lib/types';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// Fetch books from API
export const load: PageLoad = async ({ params, fetch }) => {
	const response = await fetch(`${PUBLIC_API_BASE}/books/${params.id}`);

	if (!response.ok) {
		throw error(response.status, `Failed to fetch books: ${response.statusText}`);
	}
	
	const book: Book = await response.json();
	return {
		book
	};
};
