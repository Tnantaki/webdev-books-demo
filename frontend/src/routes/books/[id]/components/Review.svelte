<script lang="ts">
	import Star from '$lib/components/icons/Star.svelte';
	let ratings = [
		{ star: 1, rating: 2 },
		{ star: 2, rating: 3 },
		{ star: 3, rating: 5 },
		{ star: 4, rating: 15 },
		{ star: 5, rating: 75 }
	];
	let total_rating = ratings.reduce((a, c) => a + c.rating, 0);
	let average_rating = ratings.reduce((a, c) => a + c.rating * c.star, 0) / total_rating;

	let reviews = [
		{
			code: 'JS',
			email: 'Jane Smith',
			rating: 5,
			review:
				"An absolute masterpiece! The story is captivating and the characters are so well-developed. I couldn't put it down.",
			date: 'July 15, 2024'
		},
		{
			code: 'MD',
			email: 'Michael Davis',
			rating: 4,
			review:
				'A very thought-provoking read. I enjoyed the unique premise, though the pacing felt a bit slow in the middle. Still, highly recommended.',
			date: 'July 12, 2024'
		},
		{
			code: 'EC',
			email: 'Emily Carter',
			rating: 3,
			review:
				"This book changed my perspective on life. It's beautifully written and emotionally resonant. A must-read for everyone.",
			date: 'July 10, 2024'
		}
	];
</script>

<h2 class="text-2xl font-bold text-text-light dark:text-text-dark mb-6">Customer Reviews</h2>
<div class="flex flex-col md:flex-row gap-8 mb-8">
	<div
		class="flex flex-col items-center justify-center bg-gray-200 dark:bg-gray-800 p-6 rounded-lg md:w-1/3"
	>
		<p class="text-5xl font-bold text-text-light dark:text-text-dark">{average_rating}</p>
		<Star rating={average_rating} />
		<p class="text-sm text-gray-500 dark:text-gray-400">Based on {total_rating} reviews</p>
	</div>
	<div class="flex-1">
		<div class="space-y-2">
			{#each ratings.reverse() as rate}
				{@const percent = (rate.rating / total_rating) * 100}
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium text-text-light dark:text-gray-300"
						>{rate.star} star</span
					>
					<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
						<div class="bg-accent h-2.5 rounded-full" style="width: {percent}%"></div>
					</div>
					<span class="text-sm font-medium text-text-light dark:text-gray-300">{percent}%</span>
				</div>
			{/each}
		</div>
	</div>
</div>
<div class="space-y-8">
	{#each reviews as review}
		<div class="border-b border-gray-200 dark:border-gray-700 pb-6">
			<div class="flex items-start gap-4">
				<div
					class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary font-bold"
				>
					<span>{review.code}</span>
				</div>
				<div class="flex-1">
					<div class="flex justify-between items-center mb-1">
						<p class="font-semibold text-text-light dark:text-text-dark">{review.email}</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">{review.date}</p>
					</div>
					<Star class="mb-2" rating={review.rating} />
					<p class="text-sm text-gray-600 dark:text-gray-400">{review.review}</p>
				</div>
			</div>
		</div>
	{/each}
</div>
