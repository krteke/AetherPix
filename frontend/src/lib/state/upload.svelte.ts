export type UploadedFile = {
	url: string | null;
	name?: string;
	size?: number;
	width?: number;
	height?: number;
	type?: string;
	previewUrl?: string;
	rawFile: File;
};

class UploadSession {
	currentBatch: UploadedFile[] = $state([]);

	setBatch(files: UploadedFile[]) {
		this.currentBatch = files;
	}
}

export const uploadSession = new UploadSession();
