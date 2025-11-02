import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';
import type { AuthResponse, LoginCredentials, SignupCredentials, User } from '$lib/types/auth';

interface RequestSignup {
	email: string;
	password: string;
	confirm_password: string;
}

export const authAPI = {
	async signup(credentials: SignupCredentials): Promise<AuthResponse> {
		const res = await fetch(`${PUBLIC_API_BASE}/auth/signup`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email: credentials.email,
				password: credentials.password,
				confirm_password: credentials.confirmPassword
			} as RequestSignup)
		});

		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
		return data;
	},

	async login(email: string, password: string): Promise<AuthResponse> {
		const res = await fetch(`${PUBLIC_API_BASE}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include', // CRITICAL: This sends/receives cookies
			body: JSON.stringify({ email, password } as LoginCredentials)
		});

		// Browser will stores cookie automatically
		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
		return data;
	},

	async logout(): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/auth/logout`, {
			method: 'POST',
			credentials: 'include' // CRITICAL: This sends/receives cookies
		});

		if (!res.ok) {
			throw new AppError({ message: 'Logout failed' });
		}
	},

	async refreshToken(): Promise<void> {
		const res = await fetch(`${PUBLIC_API_BASE}/auth/refresh`, {
			method: 'POST',
			credentials: 'include' // CRITICAL: This sends/receives cookies
		});

		if (!res.ok) {
			const error = await res.json();
			throw new AppError(error);
		}
	},

	// Get current user info (validates JWT cookie)
	async getCurrentUser(): Promise<User> {
		const res = await fetch(`${PUBLIC_API_BASE}/auth/me`, {
			method: 'GET',
			credentials: 'include' // Sends JWT cookie for validation
		});

		const data = await res.json();
		if (!res.ok) {
			throw new AppError(data);
		}
		return data;
	}
};
