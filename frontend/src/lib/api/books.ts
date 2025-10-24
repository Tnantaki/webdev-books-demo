import type { BookFilterParams, BookPage } from "$lib/types";

const API_BASE = "http://localhost:3000/api"; // TODO: move it to .env file

export const getBooksPage = async (): Promise<BookPage> => {
  // const params = new URLSearchParams();

  // params.append("page", filter.page?.toString() || "1");
  // params.append("per_page", filter.per_page?.toString() || "12");
  // params.append("sort_by", filter.per_page?.toString() || "12");
  // params.append("order", filter.per_page?.toString() || "12");
  // params.append("genre", filter.per_page?.toString() || "12");

  // const queryString = params.toString();

  // const response = await fetch(`${API_BASE}/books?${queryString}`);
  const response = await fetch(`${API_BASE}/books/page`);
  if (!response.ok) {
    throw new Error(`Failed to fetch books: ${response.statusText}`);
  }
  return response.json();
};
