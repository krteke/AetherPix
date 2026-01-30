type ModalType = 'info' | 'success' | 'warning' | 'error';

class MessageStore {
	// 模态框的 UI 状态
	isOpen = $state(false);
	title = $state('');
	content = $state('');
	type = $state<ModalType>('info');
	showCancel = $state(false);
	confirmText = $state('确定');
	cancelText = $state('取消');

	// 存储 Promise 的 resolve 函数
	private resolvePromise: ((value: boolean) => void) | null = null;

	/**
	 * 替代 window.alert
	 */
	alert(content: string, title = '提示', type: ModalType = 'info'): Promise<void> {
		this.reset();
		this.title = title;
		this.content = content;
		this.type = type;
		this.showCancel = false;
		this.isOpen = true;

		return new Promise((resolve) => {
			this.resolvePromise = () => resolve();
		});
	}

	/**
	 * 替代 window.confirm
	 * @returns Promise<boolean> - 点击确定返回 true，点击取消返回 false
	 */
	confirm(content: string, title = '确认操作', type: ModalType = 'warning'): Promise<boolean> {
		this.reset();
		this.title = title;
		this.content = content;
		this.type = type;
		this.showCancel = true;
		this.isOpen = true;

		return new Promise((resolve) => {
			this.resolvePromise = resolve;
		});
	}

	// 用户点击按钮时调用
	handleAction(result: boolean) {
		this.isOpen = false;
		if (this.resolvePromise) {
			this.resolvePromise(result);
			this.resolvePromise = null;
		}
	}

	// 重置默认值
	private reset() {
		this.confirmText = '确定';
		this.cancelText = '取消';
		// 防止上一次的 Promise 悬挂
		this.resolvePromise = null;
	}
}

export const msg = new MessageStore();
