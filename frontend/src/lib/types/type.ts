export type UserResponse = {
	pid: string;
	username: string;
	role: UserRole;
	isVerified: boolean;
};

export type UserRole = 'admin' | 'user';

export type ErrorResponse = {
	error: string;
	description: string;
};
