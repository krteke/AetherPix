export type UploadedFile = {
	id: string;
	url: string;
	name: string;
	size: string;
	width: number;
	height: number;
	type: string;
};

class UploadSession {
	currentBatch = $state<UploadedFile[]>([]);

	setBatch(files: UploadedFile[]) {
		this.currentBatch = files;
	}
}

export const uploadSession = new UploadSession();
