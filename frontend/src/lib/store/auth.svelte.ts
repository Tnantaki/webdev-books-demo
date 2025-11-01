import { authAPI } from '$lib/api/auth';
import { AppError } from '$lib/types';
import type { AuthResult, SignupCredentials } from '$lib/types/auth';

class AuthStore {
	isLoading = $state(false);

	async signup(credencials: SignupCredentials): Promise<AuthResult> {
		this.isLoading = true;

		try {
			// For test UI
			// console.log(credencials);
			// await new Promise((resolve) => setTimeout(resolve, 2000));

			await authAPI.signup(credencials);

			return { success: true };
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
	
	async login(email: string, password: string): Promise<AuthResult> {
		this.isLoading = true;

		try {
			await authAPI.login(email, password);

			return { success: true };
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
	
	async refreshToken(): Promise<AuthResult> {
		this.isLoading = true;

		try {
			await authAPI.refreshToken();

			return { success: true };
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

export const authStore = new AuthStore();
