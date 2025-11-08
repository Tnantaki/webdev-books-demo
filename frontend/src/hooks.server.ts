import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const response = await resolve(event, {
		filterSerializedResponseHeaders: (name) => {
			// Allow content-type header to be read
			return name === 'content-type';
		}
	});
	return response;
};
