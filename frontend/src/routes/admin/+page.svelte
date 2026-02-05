<script lang="ts">
	import AdminSidebar from '$lib/components/admin/AdminSidebar.svelte';
	import StoragePanel from '$lib/components/admin/panels/StoragePanel.svelte';
	// import OverviewPanel from '$lib/components/admin/panels/OverviewPanel.svelte';
	// import UsersPanel from '$lib/components/admin/panels/UsersPanel.svelte';
	import SystemPanel from '$lib/components/admin/panels/SystemPanel.svelte';

	// 默认激活概览
	let activeTab = $state('system');

	// Tab 映射
	const panels: Record<string, any> = {
		// overview: OverviewPanel,
		// users: UsersPanel,
		system: SystemPanel,
		storage: StoragePanel
	};

	const TabComponent = $derived(panels[activeTab]);
</script>

<div class="flex min-h-screen flex-col bg-base-100 transition-all duration-300 md:flex-row">
	<AdminSidebar bind:activeTab />

	<main class="h-screen flex-1 overflow-y-auto p-4 md:p-8">
		<div class="breadcrumbs mb-4 text-sm opacity-50">
			<ul>
				<li>Admin</li>
				<li class="capitalize">{activeTab}</li>
			</ul>
		</div>

		<div class="animate-fade-in mx-auto max-w-7xl">
			<TabComponent />
		</div>
	</main>
</div>

<style>
	.animate-fade-in {
		animation: fadeIn 0.2s ease-out;
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
