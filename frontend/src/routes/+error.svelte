<script lang="ts">
	import { page } from '$app/state';

	let errorMassge;
	if (page.error) {
		errorMassge = page.error.message;
	} else {
		errorMassge = 'Internal Server Error';
	}
</script>

<div class="flex flex-col w-full h-full justify-center items-center">
	<h1 class="text-2xl lg:text-4xl mt-6">{page.status}</h1>
	<h2 class="text-lg lg:text-2xl mb-4">{errorMassge}</h2>
</div>
