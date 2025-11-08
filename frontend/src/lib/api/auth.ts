import type { AuthResponse, LoginCredentials, SignupCredentials, User } from '$lib/types/auth';
import { apiClient } from './apiClient';

interface RequestSignup {
	email: string;
	password: string;
	confirm_password: string;
}

export const authAPI = {
	async signup(credentials: SignupCredentials): Promise<AuthResponse> {
		const body = {
			email: credentials.email,
			password: credentials.password,
			confirm_password: credentials.confirmPassword
		} as RequestSignup;

		return apiClient.post('/auth/signup', body);
	},

	async login(email: string, password: string): Promise<AuthResponse> {
		const body = { email, password } as LoginCredentials;
		return apiClient.post('/auth/login', body, { credentials: 'include' });
	},

	async logout(): Promise<void> {
		return apiClient.post('/auth/logout', undefined, { credentials: 'include' });
	},

	async refreshToken(): Promise<void> {
		return apiClient.post('/auth/refresh', undefined, { credentials: 'include' });
	},

	// Get current user info (validates JWT cookie)
	async getCurrentUser(fetch: typeof window.fetch = window.fetch): Promise<User> {
		// credentials for sends JWT cookie for validation
		return apiClient.get('/auth/me', { credentials: 'include' }, fetch);
	}
};
