<script lang="ts">
   import Star from "./Star.svelte";

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
   class="group relative flex flex-col bg-card-light dark:bg-card-dark rounded-xl shadow-md hover:shadow-xl transition-shadow duration-300"
>
   <div
      class="w-full bg-center bg-no-repeat aspect-[3/4] bg-cover rounded-t-xl"
      data-alt="{book.title} book cover"
      style="
         background-image: url(&quot;{book.img_path}&quot;);
      "
   ></div>
   <div class="flex flex-col flex-grow p-4">
      <h3 class="text-base font-bold leading-tight flex-grow">{book.title}</h3>
      <Star rating={book.average_rating} />
      <p class="text-lg font-bold text-primary mt-2">${book.price_in_pound}</p>
   </div>
   <a aria-label="View details for {book.title}" class="absolute inset-0" href="/books/{book.id}"
   ></a>
</div>
