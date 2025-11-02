export interface FieldError {
	field: string;
	message: string;
	params?: any;
}

export interface ApiError {
	message: string;
	errors?: FieldError[];
}

export class AppError extends Error {
	public errors?: FieldError[];
	
	constructor(data: ApiError) {
		super(data.message);
		this.errors = data.errors;

		// Set the prototype explicitly.
		Object.setPrototypeOf(this, AppError.prototype);
	}
}
