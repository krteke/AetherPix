export type UserResponse = {
	pid: string;
	username: string;
	role: UserRole;
	isVerified: boolean;
};

export type UserRole = 'admin' | 'user';

export type AppSettings = {
	uploadMaxSize: number;
	allowRegistration: boolean;
	siteName: string;
	allowEveryoneUpload: boolean;
};
