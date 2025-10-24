<script lang="ts">
   const genres = $props();
</script>

<div class="flex gap-2 p-1 overflow-x-auto">
   <button
      class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-primary text-white pl-4 pr-4 text-sm font-medium"
   >
      All
   </button>
   {#each genres as genre}
      <button
         class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-card-light dark:bg-card-dark pl-4 pr-4 text-sm font-medium hover:bg-primary/20 dark:hover:bg-primary/30"
      >
         {genre}
      </button>
   {/each}
</div>
