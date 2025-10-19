<script>
   let links = [
      {
         title: "About Us",
         href: "/about",
      },
      {
         title: "Contact",
         href: "/contact",
      },
      {
         title: "Terms of Service",
         href: "/tos",
      },
      {
         title: "Privacy Policy",
         href: "/policy",
      },
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
               <a class="text-muted-light dark:text-muted-dark hover:text-primary" href={link.href}
                  >{link.title}</a
               >
            {/each}
         </div>
         <div class="flex space-x-6 mt-6">
            {#each icons as icon}
               <a
                  class="text-muted-light dark:text-muted-dark hover:text-primary"
                  href={icon.href}
                  aria-label="link to {icon.title}"
               >
                  <span class="size-6 {icon.icon_class}"></span>
               </a>
            {/each}
         </div>
         <p class="mt-8 text-center text-sm text-muted-light dark:text-muted-dark">
            © 2025 Bookstore. All rights reserved.
         </p>
      </div>
   </div>
</footer>
