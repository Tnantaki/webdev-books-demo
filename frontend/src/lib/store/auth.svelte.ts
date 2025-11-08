import { authAPI } from '$lib/api';
import { AppError } from '$lib/types';
import type { AuthResult, SignupCredentials, User } from '$lib/types/auth';
import { AppStore } from './app.svelte';
import { cartStore } from './cart.svelte';

class AuthStore extends AppStore {
	user = $state<User | null>(null);
	isInitialized = $state<boolean>(false);

	// Initialize auth state on app load (check if user is logged in)
	async initialize(fetch: typeof window.fetch): Promise<void> {
		if (this.isInitialized) return;

		try {
			this.user = await authAPI.getCurrentUser(fetch);
		} catch (error: unknown) {
			if (error instanceof AppError) {
				try {
					// Refresh Token
					await authAPI.refreshToken();
					this.user = await authAPI.getCurrentUser(fetch);
				} catch (error: unknown) {
					const message = error instanceof AppError ? error.message : 'Token refresh failed';
					console.log(message);
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
		return this.execute(async () => {
			await authAPI.signup(credencials);

			return { success: true };
		});
	}

	async login(email: string, password: string): Promise<AuthResult> {
		return this.execute(async () => {
			await authAPI.login(email, password);
			this.user = await authAPI.getCurrentUser();
			await cartStore.loadCart();

			return { success: true };
		});
	}

	async logout(): Promise<void> {
		this.execute(async () => {
			await authAPI.logout();
			this.user = null;
		});
	}

	async refreshToken(): Promise<AuthResult> {
		return this.execute(async () => {
			await authAPI.refreshToken();

			return { success: true };
		});
	}
}

export const authStore = new AuthStore();
