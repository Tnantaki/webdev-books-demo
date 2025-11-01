import { authAPI } from '$lib/api/auth';
import type { AuthResult, SignupCredentials } from '$lib/types/auth';

class AuthStore {
	isLoading = $state(false);

	async signup(credencials: SignupCredentials): Promise<AuthResult> {
		this.isLoading = true;
		
		try {
			// For test UI
			console.log(credencials);
			await new Promise((resolve) => setTimeout(resolve, 2000));
			
			// const data = await authAPI.signup(credencials);
			// console.log(data);
			
			return { success: true };
		} catch (error: any) {
			console.log(error);
			const message = error instanceof Error ? error.message : 'An error occurred';
			return { success: false, error: message };
		} finally {
			this.isLoading = false;
		}
	}
}

export const authStore = new AuthStore();
