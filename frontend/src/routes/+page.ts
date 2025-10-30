import { PUBLIC_API_BASE } from '$env/static/public';
import type { BookPage } from '$lib/types';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// Fetch books from API
export const load: PageLoad = async ({ url, fetch }) => {
	const response = await fetch(`${PUBLIC_API_BASE}/books/page?${url.searchParams.toString()}`);

	if (!response.ok) {
		throw error(response.status, `Failed to fetch books: ${response.statusText}`);
	}

	const bookPage: BookPage = await response.json();
	if (!bookPage) {
		throw error(500, 'Failed to deserialize json');
	}
	const { pagination, books } = bookPage;
	return { pagination, books };
};
