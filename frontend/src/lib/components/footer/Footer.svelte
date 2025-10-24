<script>
   import FooterIcon from "./FooterIcon.svelte";
   import FooterLink from "./FooterLink.svelte";

   let links = [
      { title: "About Us", href: "/about" },
      { title: "Contact", href: "/contact" },
      { title: "Terms of Service", href: "/tos" },
      { title: "Privacy Policy", href: "/policy" },
   ];

   let icons = [
      {
         title: "x",
         href: "https://x.com/",
         icon_class: "icon-[ri--twitter-x-fill]",
      },
      {
         title: "facebook",
         href: "https://www.facebook.com/",
         icon_class: "icon-[ic--baseline-facebook]",
      },
      {
         title: "instagram",
         href: "https://www.instagram.com/",
         icon_class: "icon-[line-md--instagram]",
      },
   ];
</script>

<footer class="bg-card-light dark:bg-card-dark">
   <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
      <div class="flex flex-col items-center">
         <div class="flex space-x-6">
            {#each links as link}
               <FooterLink href={link.href} title={link.title} />
            {/each}
         </div>
         <div class="flex space-x-6 mt-6">
            {#each icons as icon}
               <FooterIcon {...icon} />
            {/each}
         </div>
         <p class="mt-8 text-center text-sm text-muted-light dark:text-muted-dark">
            © 2025 Bookstore. All rights reserved.
         </p>
      </div>
   </div>
</footer>
