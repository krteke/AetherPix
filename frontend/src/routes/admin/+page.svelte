<script lang="ts">
	import AdminSidebar from '$lib/components/admin/AdminSidebar.svelte';
	import OverviewPanel from '$lib/components/admin/panels/OverviewPanel.svelte';
	import UsersPanel from '$lib/components/admin/panels/UsersPanel.svelte';
	import SystemPanel from '$lib/components/admin/panels/SystemPanel.svelte';

	// 默认激活概览
	let activeTab = $state('overview');

	// Tab 映射
	const panels: Record<string, any> = {
		overview: OverviewPanel,
		users: UsersPanel,
		system: SystemPanel
	};
</script>

<div class="flex min-h-screen flex-col bg-base-100 md:flex-row">
	<!-- 侧边栏 -->
	<AdminSidebar bind:activeTab />

	<!-- 主内容区 -->
	<main class="h-screen flex-1 overflow-y-auto p-4 md:p-8">
		<!-- 顶部 Header (仅移动端显示 Sidebar 切换时有用，这里简化处理) -->
		<div class="breadcrumbs mb-4 text-sm opacity-50">
			<ul>
				<li>Admin</li>
				<li class="capitalize">{activeTab}</li>
			</ul>
		</div>

		<!-- 动态面板渲染 -->
		<div class="animate-fade-in mx-auto max-w-7xl">
			<svelte:component this={panels[activeTab]} />
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
