<script>
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/store/auth.svelte';
	import { CircleUserRound, LogOut, Logs, Settings } from '@lucide/svelte';

	async function handleLogout() {
		await authStore.logout();
		goto('/login');
	}
</script>

<div class="relative group">
	<button
		class="flex-shrink-0 bg-white dark:bg-background-dark rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary flex items-center cursor-pointer"
		type="button"
		aria-label="Open user menu"
	>
		<div
			class="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-9"
			data-alt="User avatar"
			style="
      background-image: url(&quot;https://lh3.googleusercontent.com/aida-public/AB6AXuCx0X2we99eynSSqiqDG8NpxEWU0qOA0kpdhSDRftR9U8s2X9t-6WDFDdzVg3msmG2VhSxzqA0-FmKW-xHnwq-IpgDHey39KKFLzbFeTKLIrWYGL5gL3PAu7-TvP5pTxSadyfv9uPYXCtgyAJkFnUgCtTpkfQmBRPeeXkbE002k7dtTjPTSVsvLAG7HA_XeWKUEuae8GuUkyvayjmLf2nLsZq9BB9eoorsyLWm2_MWprl00ubt__cfsrcquFqCKATDQm9HIshSLukKt&quot;);
   "
		></div>
	</button>
	<div
		class="opacity-0 invisible group-hover:opacity-100 group-hover:visible z-10 origin-top-right absolute right-0 transition-all duration-200 scale-y-50 scale-x-50 group-hover:scale-y-100 group-hover:scale-x-100 -translate-y-2 group-hover:translate-y-0 delay-300 group-hover:delay-0"
	>
		<div
			class=" mt-2 w-48 rounded-lg shadow-xl inset-shadow-sm bg-card-light dark:bg-card-dark focus:outline-none overflow-hidden"
		>
			<a
				class="flex items-center gap-1 w-full font-medium px-4 py-2 text-sm text-text-light dark:text-text-dark hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
				href="/profile"
			>
				<CircleUserRound size={16} />
				<span>My Profile</span>
			</a>
			<a
				class="flex items-center gap-1 w-full font-medium px-4 py-2 text-sm text-text-light dark:text-text-dark hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
				href="/payment"
			>
				<span class="icon-[fluent--payment-32-regular] size-4"></span>
				<span>My Payment</span>
			</a>
			<a
				class="flex items-center gap-1 w-full font-medium px-4 py-2 text-sm text-text-light dark:text-text-dark hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
				href="/orders"
			>
				<Logs size={16} />
				<span>My Orders</span>
			</a>
			<button
				onclick={handleLogout}
				class="flex items-center gap-1 w-full font-medium px-4 py-2 text-sm text-text-light dark:text-text-dark hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
			>
				<LogOut size={16} /><span>Logout</span>
			</button>
		</div>
	</div>
</div>
