import { getBooksPage } from "$lib/api";
import type { PageLoad } from "./$types";

// Fetch books from API
export const load: PageLoad = async ({ url }) => {
  const bookPage = await getBooksPage();
  console.log(bookPage)

  return { bookPage };
};

// async function fetch_books_page(page: number = 1): Promise<void> {
//    loading = true;
//    error = null;

//    try {
//       const response = await fetch(`http://localhost:3000/api/books?page=${page}`);

//       const result: BookPage = await response.json();
//       if (!result) {
//          throw new Error("Failed to deserialize json");
//       }
//       const { pagination, data } = result;

//       current_page = pagination.current_page;
//    } catch (err) {
//       error = err instanceof Error ? err.message : "An error occurred";
//       books = [];
//    } finally {
//       loading = false;
//    }
// }