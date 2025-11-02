import { authAPI } from '$lib/api';
import { AppError } from '$lib/types';
import type { AuthResult, SignupCredentials, User } from '$lib/types/auth';

class AuthStore {
	user = $state<User | null>(null);
	isLoading = $state(false);
	isInitialized = $state<boolean>(false);

	// Initialize auth state on app load (check if user is logged in)
	async initialize(): Promise<void> {
		if (this.isInitialized) return;

		try {
			this.user = await authAPI.getCurrentUser();
		} catch (error: unknown) {
			if (error instanceof AppError && error.message.includes('ExpiredSignature')) {
				try {
					await authAPI.refreshToken();
					this.user = await authAPI.getCurrentUser();
				} catch {
					if (error instanceof AppError) {
						console.log(error.message);
					} else {
						console.log('Token refresh failed');
					}
					this.user = null;
				}
			} else {
				console.log(error);
				// User is not authenticated
				this.user = null;
			}
		} finally {
			this.isInitialized = true;
		}
	}

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
			this.user = await authAPI.getCurrentUser();

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

	async logout(): Promise<void> {
		this.isLoading = true;
		try {
			await authAPI.logout();
			this.user = null;
		} catch (error) {
			console.error('Logout error:', error);
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
