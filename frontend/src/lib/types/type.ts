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

export type UploadResponse = {
	url: string;
};

export type ListViewResponse = {
	images: Image[];
	total: number;
	pages: number;
};

export type Image = {
	id: number;
	previewUrl: string;
	originalUrl: string;
	name: string;
	size: string;
};
