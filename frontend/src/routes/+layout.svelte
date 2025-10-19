<script lang="ts">
   import "../app.css";
   import favicon from "$lib/assets/favicon.svg";
   import Nav from "../components/Nav.svelte";
   import Footer from "../components/Footer.svelte";

   let { children } = $props();
</script>

<svelte:head>
   <link rel="icon" href={favicon} />
   <title>Bookstore - Online Bookstore</title>
</svelte:head>

<div
   class="bg-background-light dark:bg-background-dark text-text-light dark:text-text-dark font-display"
>
   <div
      class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden"
   >
      <div class="layout-container flex h-full grow flex-col">
         <Nav />

         {@render children?.()}

         <Footer />
      </div>
   </div>
</div>
