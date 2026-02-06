<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import ProfilePanel from '$lib/components/ProfilePanel.svelte';
	import SecurityPanel from '$lib/components/SecurityPanel.svelte';
	import type { UserProfileResponse } from '$lib/types/type';
	import { onMount } from 'svelte';
	// import StatsPanel from '$lib/components/StatsPanel.svelte';

	let activeTab = $state('profile');

	const panels: Record<string, any> = {
		profile: ProfilePanel,
		security: SecurityPanel
		// stats: StatsPanel
		// prefs: PreferencesPanel
	};
	const PanelComponent = $derived(panels[activeTab]);

	let username = $state('');
	let email = $state('');
	let apiToken = $state('************');
	const fetchUser = async () => {
		console.log('fetchUser called');
		try {
			console.log('Attempting to fetch...');
			const response = await fetch('/api/profile/user', {
				method: 'GET'
			});

			console.log(response);
			if (!response.ok) {
				console.error('Fetch failed:', response.statusText);
				return;
			}
			console.log('Data received');

			const data: UserProfileResponse = await response.json();
			username = data.name;
			email = data.email;
			apiToken = data.apiToken;
		} catch (error) {
			console.error(error);
		}
	};

	onMount(() => {
		fetchUser();
	});
</script>

<div class="container mx-auto max-w-6xl px-4 py-8">
	<div class="mb-6">
		<h1 class="text-3xl font-extrabold">账户设置</h1>
		<p class="text-base-content/60 transition-all duration-300">
			管理您的个人信息、安全选项及偏好。
		</p>
	</div>

	<div class="flex flex-col items-start gap-8 md:flex-row">
		<aside class="sticky top-20 w-full shrink-0 md:w-64">
			<Sidebar bind:activeTab />
		</aside>

		<main class="w-full min-w-0 flex-1">
			<div class="animate-fade-in">
				<PanelComponent name={username} {email} {apiToken} class="transition-all duration-300" />
			</div>
		</main>
	</div>
</div>

<style>
	.animate-fade-in {
		animation: fadeIn 0.3s ease-out;
	}
	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
