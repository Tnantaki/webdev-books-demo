import { PUBLIC_API_BASE } from '$env/static/public';
import type { BookPage } from '$lib/types';
import type { PageLoad } from './$types';

// Fetch books from API
export const load: PageLoad = async ({ url, fetch }) => {
	const response = await fetch(`${PUBLIC_API_BASE}/books/page?${url.searchParams.toString()}`);

	if (!response.ok) {
		throw new Error(`Failed to fetch books: ${response.statusText}`);
	}

	const bookPage: BookPage = await response.json();
	if (!bookPage) {
		throw new Error('Failed to deserialize json');
	}
	const { pagination, books } = bookPage;
	return { pagination, books };
};
