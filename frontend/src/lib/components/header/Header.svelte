<script>
   import BookSearch from "./BookSearch.svelte";
   import Logo from "./Logo.svelte";
   import Profile from "./Profile.svelte";
</script>

<header class="w-full bg-card-light dark:bg-card-dark shadow-sm">
   <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
         <Logo />
         <div class="flex">
            <BookSearch />
            <div class="hidden lg:ml-4 lg:flex lg:items-center gap-4">
               <!-- <a class="text-sm font-medium hover:text-primary" href="#">My Account</a> -->
               <a class="flex items-center text-sm font-medium hover:text-primary" href="/cart">
                  <span class="icon-[mdi--cart-outline] mr-1 size-6"></span>
                  <!-- <span class="material-symbols-outlined mr-1">shopping_cart</span> -->
                  Cart
               </a>
               <button
                  class="flex-shrink-0 bg-white dark:bg-background-dark p-1 rounded-full text-muted-light dark:text-muted-dark hover:text-primary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                  type="button"
               >
                  <span class="sr-only">View notifications</span>
                  <span class="material-symbols-outlined">notifications</span>
               </button>
               <Profile />
            </div>
         </div>
      </div>
   </div>
</header>
