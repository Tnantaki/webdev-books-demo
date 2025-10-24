<script lang="ts">
   import { page } from "$app/state";

   let errorMassge;
   if (page.error) {
      errorMassge = page.error.message;
   } else {
      errorMassge = "Internal Server Error";
   }
</script>

<div class="flex flex-col w-full justify-center">
   <h1>{page.status}</h1>
   <h2>{errorMassge}</h2>
</div>
