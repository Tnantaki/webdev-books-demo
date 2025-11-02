import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({ url, cookies }) => {
	const act = cookies.get('act');

	if (!act) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	return;
};
