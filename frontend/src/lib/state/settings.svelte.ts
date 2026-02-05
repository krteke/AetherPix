import type { AppSettings } from '$lib/types/type';

class SettingsStore {
	settings: AppSettings | null = $state(null);

	get allowRegister() {
		return this.settings?.allow_registration ?? false;
	}

	get allowEveryoneUpload() {
		return this.settings?.allow_everyone_upload ?? false;
	}

	get siteName() {
		return this.settings?.site_name ?? '';
	}

	get uploadMaxSize() {
		return this.settings?.upload_max_size ?? 0;
	}

	get localBaseUrl() {
		return this.settings?.local_base_url ?? '';
	}

	get r2BaseUrl() {
		return this.settings?.r2_base_url ?? '';
	}

	get exists() {
		return this.settings !== null;
	}

	async init() {
		try {
			const res = await fetch('/api/settings');
			if (res.ok) {
				const settings: AppSettings = await res.json();
				this.settings = settings;
			} else {
				this.settings = null;
			}
		} catch (error) {
			console.error('Error initializing settings:', error);
			this.settings = null;
		}
	}

	async update(settings: AppSettings) {
		try {
			const res = await fetch('/api/settings/update', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(settings)
			});
			if (res.ok) {
				this.settings = settings;
			} else {
				console.error('Error updating settings:', await res.text());
			}
		} catch (error) {
			console.error('Error updating settings:', error);
		}
	}
}

export const settingsStore = new SettingsStore();
