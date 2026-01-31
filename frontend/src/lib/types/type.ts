export type UserResponse = {
	pid: string;
	username: string;
	role: UserRole;
	isVerified: boolean;
};

export type UserRole = 'admin' | 'user';
