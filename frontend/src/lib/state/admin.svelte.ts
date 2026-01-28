// src/lib/state/admin.svelte.ts

type User = {
	id: number;
	name: string;
	email: string;
	role: 'admin' | 'user';
	status: 'active' | 'banned';
	joinedAt: string;
	storageUsed: string;
};

class AdminStore {
	// 1. 系统设置
	config = $state({
		siteName: 'FlashImg Pro',
		allowRegistration: true,
		allowPublicUpload: true, // 游客上传
		maintenanceMode: false,
		announcement: '系统将于今晚 02:00 进行例行维护。'
	});

	// 2. 服务器实时状态 (模拟)
	server = $state({
		cpu: 12,
		ram: 45, // %
		disk: 68, // %
		uptime: '14天 2小时 15分',
		version: 'Loco.rs v0.5.2'
	});

	// 3. 用户列表
	users = $state<User[]>([
		{
			id: 1,
			name: 'Admin',
			email: 'admin@host.com',
			role: 'admin',
			status: 'active',
			joinedAt: '2023-01-01',
			storageUsed: '540 MB'
		},
		{
			id: 2,
			name: 'Alice',
			email: 'alice@test.com',
			role: 'user',
			status: 'active',
			joinedAt: '2023-05-12',
			storageUsed: '1.2 GB'
		},
		{
			id: 3,
			name: 'Bob',
			email: 'bob@bad.com',
			role: 'user',
			status: 'banned',
			joinedAt: '2023-06-20',
			storageUsed: '0 MB'
		},
		{
			id: 4,
			name: 'Charlie',
			email: 'charlie@web.com',
			role: 'user',
			status: 'active',
			joinedAt: '2023-08-15',
			storageUsed: '45 MB'
		}
	]);

	// Action: 切换用户状态
	toggleUserStatus(id: number) {
		const user = this.users.find((u) => u.id === id);
		if (user) {
			user.status = user.status === 'active' ? 'banned' : 'active';
		}
	}

	// Action: 删除用户
	deleteUser(id: number) {
		this.users = this.users.filter((u) => u.id !== id);
	}

	// Action: 添加用户
	addUser(name: string, email: string) {
		const newUser: User = {
			id: Date.now(),
			name,
			email,
			role: 'user',
			status: 'active',
			joinedAt: new Date().toISOString().split('T')[0],
			storageUsed: '0 KB'
		};
		this.users = [...this.users, newUser];
	}
}

export const adminStore = new AdminStore();
