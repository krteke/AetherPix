// src/lib/state/upload.svelte.ts
export type UploadedFile = {
	id: string;
	url: string;
	name: string;
	size: string;
	width: number;
	height: number;
	type: string; // e.g., 'image/png'
};

// 使用 Svelte 5 $state 创建全局状态
class UploadSession {
	currentBatch = $state<UploadedFile[]>([]);

	setBatch(files: UploadedFile[]) {
		this.currentBatch = files;
	}
}

export const uploadSession = new UploadSession();
