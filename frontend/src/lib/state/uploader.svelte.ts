import type { UploadedFile } from './upload.svelte';

export type UploadStatus = 'pending' | 'uploading' | 'success' | 'error';

export interface UploadItem {
	name: string;
	file: File;
	status: UploadStatus;
	progress: number; // 0-100
	speed: string; // e.g., "1.2 MB/s"
	loaded: number; // 已上传字节数
	total: number; // 总字节数
	xhr?: XMLHttpRequest; // 用于取消上传
	result?: UploadedFile;
	response?: any;

	// 用于计算速度的临时变量
	_lastLoaded: number;
	_lastTime: number;
}

class UploadManager {
	// 任务队列
	queue: UploadItem[] = $state([]);
	// 最大并发数
	concurrency = 3;
	// API 地址
	uploadUrl = '/api/upload';

	// 添加文件到队列
	addFiles(files: FileList) {
		const newItems: UploadItem[] = Array.from(files).map((file) => ({
			name: file.name,
			file,
			status: 'pending',
			progress: 0,
			speed: '0 KB/s',
			loaded: 0,
			total: file.size,
			_lastLoaded: 0,
			_lastTime: Date.now()
		}));

		this.queue.push(...newItems);
	}

	// 核心：并发调度器
	processQueue() {
		// 计算正在上传的数量
		const uploadingCount = this.queue.filter((i) => i.status === 'uploading').length;

		// 如果达到并发限制，什么都不做
		if (uploadingCount >= this.concurrency) return;

		// 计算还能启动几个
		const slotsAvailable = this.concurrency - uploadingCount;

		// 从 pending 状态的任务中取出 N 个
		const nextItems = this.queue.filter((i) => i.status === 'pending').slice(0, slotsAvailable);

		// 启动这些任务
		nextItems.forEach((item) => {
			this.uploadFile(item);
		});
	}

	retryFailedItems() {
		const failedItems = this.queue.filter((i) => i.status === 'error');
		failedItems.forEach((item) => {
			item.status = 'pending';
			this.uploadFile(item);
		});
	}

	// 单个文件上传逻辑 (使用 XHR)
	private uploadFile(item: UploadItem) {
		item.status = 'uploading';
		item._lastTime = Date.now();
		item._lastLoaded = 0;

		const xhr = new XMLHttpRequest();
		item.xhr = xhr; // 保存引用以便取消

		// 监听上传进度
		xhr.upload.onprogress = (e) => {
			if (e.lengthComputable) {
				const now = Date.now();
				const timeDiff = now - item._lastTime;

				// 每 500ms 更新一次速度，防止 UI 闪烁过于频繁
				if (timeDiff > 500) {
					const loadedDiff = e.loaded - item._lastLoaded;
					const speedBytesPerSec = (loadedDiff / timeDiff) * 1000;

					item.speed = this.formatSpeed(speedBytesPerSec);
					item._lastTime = now;
					item._lastLoaded = e.loaded;
				}

				item.loaded = e.loaded;
				item.progress = (e.loaded / e.total) * 100;
			}
		};

		// 处理完成/失败
		xhr.onload = () => {
			if (xhr.status >= 200 && xhr.status < 300) {
				item.status = 'success';
				item.progress = 100;
				item.speed = '';
				try {
					item.response = JSON.parse(xhr.responseText);
				} catch {
					item.response = xhr.responseText;
				}
			} else {
				item.status = 'error';
				item.speed = '';
			}
			this.triggerNext(); // 任务结束，触发下一个
		};

		xhr.onerror = () => {
			item.status = 'error';
			item.speed = '';
			this.triggerNext();
		};

		xhr.onabort = () => {
			item.status = 'pending';
			item.progress = 0;
			item.speed = '';
			this.triggerNext();
		};

		// 发送请求
		xhr.open('POST', this.uploadUrl, true);

		const formData = new FormData();
		formData.append('file', item.file);
		// 可以在这里 append 其他参数，比如 storageLocation
		// formData.append('strategy', 'r2');

		xhr.send(formData);
	}

	// 任务完成后的回调
	private triggerNext() {
		setTimeout(() => {
			this.processQueue();
		}, 100);
	}

	// 手动重试
	retry(item: UploadItem) {
		if (item.status === 'uploading') return;
		item.status = 'pending';
		item.progress = 0;
		this.processQueue();
	}

	// 辅助：格式化速度
	private formatSpeed(bytesPerSec: number): string {
		if (bytesPerSec === 0) return '0 KB/s';
		const k = 1024;
		const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s'];
		const i = Math.floor(Math.log(bytesPerSec) / Math.log(k));
		return parseFloat((bytesPerSec / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}
}

// 导出单例
export const uploader = new UploadManager();
