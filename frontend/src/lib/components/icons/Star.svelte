<script lang="ts">
   let { rating }: { rating: number } = $props();

   let star_styles = [];
   for (let i = 0; i < 5; i++) {
      let result = rating - i;
      if (result > 1) {
         star_styles.push("icon-[icomoon-free--star-full] text-yellow-400");
      } else if (result <= 0) {
         star_styles.push("icon-[icomoon-free--star-empty] text-gray-300 dark:text-gray-600");
      } else {
         if (result < 0.25) {
            star_styles.push("icon-[icomoon-free--star-empty] text-gray-300 dark:text-gray-600");
         } else if (result < 0.75) {
            star_styles.push("icon-[icomoon-free--star-half] text-yellow-400");
         } else {
            star_styles.push("icon-[icomoon-free--star-full] text-yellow-400");
         }
      }
   }
</script>

<div class="flex items-center mt-2">
   {#each star_styles as style}
      <span class="size-5 {style}"></span>
   {/each}
</div>
