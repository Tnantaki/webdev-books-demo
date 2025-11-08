import type { FieldError } from ".";

export interface SignupCredentials {
	email: string;
	password: string;
	confirmPassword: string;
	name?: string; // TODO: will implement backend to take name
}

export interface LoginCredentials {
	email: string;
	password: string;
}

export interface User {
	user_id: string;
	role: string;
	exp: number;
}

export interface AuthResponse {
	message: string;
}

export interface AuthResult {
	success: boolean;
	errors?: FieldError[];
}
