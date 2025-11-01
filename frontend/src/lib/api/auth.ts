import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';
import type { AuthResponse, SignupCredentials } from '$lib/types/auth';

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
			// credentials: 'include', // CRITICAL: This sends/receives cookies
			body: JSON.stringify({
				email: credentials.email,
				password: credentials.password,
				confirm_password: credentials.confirmPassword
			} as RequestSignup)
		});

		const data = await res.json();
		if (!res.ok) {
			console.log("throw app error")
			throw new AppError(data);
		}

		return data;
	}
};
