<script lang="ts">
   import BookCard from "../components/BookCard.svelte";
   import { mockup_books, mockup_genres } from "../lib/data/mockup_books";
</script>

<main class="flex-1">
   <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="mb-8">
         <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold">All Books</h2>
            <div class="flex gap-2 p-1 overflow-x-auto">
               <button
                  class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-primary text-white pl-4 pr-4 text-sm font-medium"
               >
                  All
               </button>
               {#each mockup_genres as genre}
                  <button
                     class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-card-light dark:bg-card-dark pl-4 pr-4 text-sm font-medium hover:bg-primary/20 dark:hover:bg-primary/30"
                  >
                     {genre}
                  </button>
               {/each}
            </div>
         </div>
      </div>
      <div
         class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6"
      >
         {#each mockup_books as book}
            <BookCard {...book} />
         {/each}
      </div>
      <div class="flex items-center justify-center p-4 mt-8">
         <a
            class="flex size-10 items-center justify-center text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary"
            href="#"
         >
            <span class="material-symbols-outlined">chevron_left</span>
         </a>
         <a
            class="text-sm font-bold leading-normal flex size-10 items-center justify-center text-white rounded-full bg-primary"
            href="#">1</a
         >
         <a
            class="text-sm font-normal leading-normal flex size-10 items-center justify-center rounded-full hover:bg-primary/20 dark:hover:bg-primary/30"
            href="#">2</a
         >
         <a
            class="text-sm font-normal leading-normal flex size-10 items-center justify-center rounded-full hover:bg-primary/20 dark:hover:bg-primary/30"
            href="#">3</a
         >
         <span class="text-sm font-normal leading-normal flex size-10 items-center justify-center"
            >...</span
         >
         <a
            class="text-sm font-normal leading-normal flex size-10 items-center justify-center rounded-full hover:bg-primary/20 dark:hover:bg-primary/30"
            href="#">10</a
         >
         <a
            class="flex size-10 items-center justify-center text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary"
            href="#"
         >
            <span class="material-symbols-outlined">chevron_right</span>
         </a>
      </div>
   </div>
</main>
