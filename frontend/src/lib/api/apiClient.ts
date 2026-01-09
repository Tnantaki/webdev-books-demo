import { PUBLIC_API_BASE } from '$env/static/public';
import { AppError } from '$lib/types';

type MethodWithData = 'POST' | 'PUT' | 'PATCH';

async function deserializeData<T>(res: Response): Promise<T> {
	const contentType = res.headers.get('content-type');

	if (!res.ok) {
		if (contentType && contentType.includes('application/json')) {
			const err_json = await res.json();
			throw new AppError(err_json);
		}
		const err_text = await res.json();
		throw new Error(err_text);
	}
	if (contentType && contentType.includes('application/json')) {
		return (await res.json()) as T;
	}
	return undefined as T;
}

async function methodWithData<T = any, B = any>(
	method: MethodWithData,
	url: string,
	body?: B,
	options?: RequestInit
): Promise<T> {
	const res = await fetch(PUBLIC_API_BASE + url, {
		method,
		headers: {
			...(body && { 'Content-Type': 'application/json' }),
			...options?.headers
		},
		body: body ? JSON.stringify(body) : undefined,
		...options
	});
	return deserializeData<T>(res);
}

export const apiClient = {
	async get<T = any>(
		url: string,
		options?: RequestInit,
		fetch: typeof window.fetch = window.fetch
	): Promise<T> {
		const res = await fetch(PUBLIC_API_BASE + url, { method: 'GET', ...options });
		return deserializeData<T>(res);
	},

	async delete<T = any>(url: string, options?: RequestInit): Promise<T> {
		const res = await fetch(PUBLIC_API_BASE + url, { ...options });
		return deserializeData<T>(res);
	},

	async post<T = any, B = any>(url: string, body?: B, options?: RequestInit): Promise<T> {
		return methodWithData('POST', url, body, options);
	},

	async put<T = any, B = any>(url: string, body?: B, options?: RequestInit): Promise<T> {
		return methodWithData('PUT', url, body, options);
	}
};
