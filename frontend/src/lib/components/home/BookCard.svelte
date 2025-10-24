<script lang="ts">
	import { Star } from '$lib/components/icons';

	interface Props {
		id: string;
		genre: string;
		description: string;
		img_path: string;
		title: string;
		available: number;
		average_rating: number;
		price_in_pound: number;
		total_ratings: number;
	}

	let book: Props = $props();
</script>

<div
	class="group bg-card-light dark:bg-card-dark relative flex flex-col rounded-xl shadow-md transition-shadow duration-300 hover:shadow-xl"
>
	<div
		class="aspect-[3/4] w-full rounded-t-xl bg-cover bg-center bg-no-repeat"
		data-alt="{book.title} book cover"
		style="
         background-image: url(&quot;{book.img_path}&quot;);
      "
	></div>
	<div class="flex flex-grow flex-col p-4">
		<h3 class="flex-grow text-base leading-tight font-bold">{book.title}</h3>
		<Star rating={book.average_rating} />
		<p class="text-primary mt-2 text-lg font-bold">${book.price_in_pound}</p>
	</div>
	<a aria-label="View details for {book.title}" class="absolute inset-0" href="/books/{book.id}"
	></a>
</div>
