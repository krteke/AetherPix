export type UserResponse = {
	pid: string;
	username: string;
	role: UserRole;
	isVerified: boolean;
};

export type UserRole = 'admin' | 'user';

export type AppSettings = {
	upload_max_size: number;
	allow_registration: boolean;
	site_name: string;
	allow_everyone_upload: boolean;
	local_base_url: string;
	r2_base_url: string;
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

export type UserProfileResponse = {
	name: string;
	email: string;
	apiToken: string;
};

export type ApiKeyResponse = {
	key: string;
};

export type PresignResponse = {
	uploadUrl: string;
};
