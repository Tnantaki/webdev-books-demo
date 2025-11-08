import { AppError } from '$lib/types';

export class AppStore {
	isLoading = $state(false);
	error = $state<string | null>(null);

	protected async execute<T>(fn: () => Promise<T>) {
		this.isLoading = true;
		this.error = null;

		try {
			return await fn();
		} catch (error: unknown) {
			if (error instanceof AppError) {
				const { message, errors } = error;
				return { success: false, message, errors };
			}
			return { success: false, message: 'An error occurred' };
		} finally {
			this.isLoading = false;
		}
	}
}
