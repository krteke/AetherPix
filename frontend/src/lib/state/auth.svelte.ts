import type { UserResponse, UserRole } from '$lib/types/type';

export type User = {
	pid: string;
	username: string;
	role: UserRole;
	isVerified: boolean;
};

class AuthStore {
	user = $state<User | null>(null);
	isLoading = $state(true);

	get isLoggedIn() {
		return !!this.user;
	}

	get isAdmin() {
		return this.user?.role === 'admin';
	}

	get exists() {
		return this.user !== null;
	}

	login(userData: User) {
		this.user = userData;
	}

	async logout() {
		this.user = null;
		try {
			const res = await fetch('/api/auth/logout', {
				method: 'POST',
				credentials: 'include'
			});
			if (res.ok) {
				this.user = null;
			}
		} catch (error) {
			console.error('Logout failed', error);
		}
	}

	async init() {
		this.isLoading = true;
		try {
			const res = await fetch('/api/auth/current', {
				method: 'GET',
				credentials: 'include'
			});

			if (res.ok) {
				const data: UserResponse = await res.json();
				this.user = data;
			} else {
				this.user = null;
			}
		} catch (error) {
			console.error('Auth check failed', error);
			this.user = null;
		} finally {
			this.isLoading = false;
		}
	}
}

export const auth = new AuthStore();
