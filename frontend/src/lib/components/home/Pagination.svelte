<script lang="ts">
   import type { Pagination } from "$lib/types";

   let { pagination }: { pagination: Pagination } = $props();


   function get_page_numbers(current_page: number, total_pages: number): (number | string)[] {
      if (total_pages <= 7) {
         return Array.from({ length: total_pages }, (_, i) => i + 1);
      }

      let page_numbers: (number | string)[] = [];

      const start = Math.max(2, current_page - 2);
      const end = Math.min(total_pages - 1, current_page + 2);

      page_numbers.push(1);

      if (start > 2) page_numbers.push("...");

      for (let i = start; i <= end; i++) {
         page_numbers.push(i);
      }

      if (end < total_pages - 1) page_numbers.push("...");

      page_numbers.push(total_pages);

      return page_numbers;
   }

   async function handlePageChange(newPage: number) {
      // await goto(`?page=${newPage}`);
      pagination.current_page = newPage;
      console.log(newPage);
   }

   // $effect(() => {
   //    let pages = get_page_numbers();
   // });
   let pages = get_page_numbers(pagination.current_page, pagination.total_pages);

   let page_button_style =
      "text-sm leading-normal flex size-10 items-center justify-center rounded-full";

   console.log(pages);
</script>

<div class="flex items-center justify-center p-4 mt-8">
   <!-- Previous Button -->
   <button
      class="flex size-10 items-center justify-center text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary"
      aria-label="link to previous page"
      disabled={!pagination.has_previous}
      onclick={() => pagination.has_previous && handlePageChange(pagination.current_page - 1)}
   >
      <span class="icon-[material-symbols--chevron-left] size-6"></span>
   </button>

   <!-- Page Numbers -->
   {#each pages as page}
      {#if page === "..."}
         <span class="text-sm font-normal leading-normal flex size-10 items-center justify-center"
            >...</span
         >
      {:else if page == pagination.current_page}
         <button
            class="{page_button_style} font-bold text-white bg-primary"
            onclick={() => handlePageChange(page as number)}>{page}</button
         >
      {:else}
         <button
            class="{page_button_style} font-normal hover:bg-primary/20 dark:hover:bg-primary/30"
            onclick={() => handlePageChange(page as number)}>{page}</button
         >
      {/if}
   {/each}

   <!-- Next Button -->
   <button
      class="flex size-10 items-center justify-center text-muted-light dark:text-muted-dark hover:text-primary dark:hover:text-primary"
      aria-label="link to previous page"
      disabled={!pagination.has_next}
      onclick={() => pagination.has_next && handlePageChange(pagination.current_page + 1)}
   >
      <span class="icon-[material-symbols--chevron-right] size-6"></span>
   </button>
</div>
