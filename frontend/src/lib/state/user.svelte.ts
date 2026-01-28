// src/lib/state/user.svelte.ts
class UserState {
	// 基础信息
	info = $state({
		username: 'AdminUser',
		email: 'admin@flashimg.com',
		avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Felix',
		bio: '热爱摄影，热爱开源。'
	});

	// 偏好设置
	preferences = $state({
		defaultLinkType: 'Markdown', // Markdown, HTML, URL
		autoCompress: true,
		privateUpload: false
	});

	// 统计数据 (模拟只读)
	stats = $state({
		totalUploads: 142,
		totalViews: 89200,
		storageUsed: 450.5, // MB
		storageLimit: 1024, // MB
		recentUploads: [
			{ id: 1, name: 'screenshot_01.png', date: '2023-10-25', size: '1.2MB', views: 120 },
			{ id: 2, name: 'avatar_new.jpg', date: '2023-10-24', size: '0.5MB', views: 45 },
			{ id: 3, name: 'project_demo.gif', date: '2023-10-22', size: '4.1MB', views: 890 }
		]
	});

	updateInfo(newInfo: Partial<typeof this.info>) {
		this.info = { ...this.info, ...newInfo };
	}
}

export const userState = new UserState();
