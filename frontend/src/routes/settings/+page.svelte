<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import ProfilePanel from '$lib/components/ProfilePanel.svelte';
	import SecurityPanel from '$lib/components/SecurityPanel.svelte';
	import StatsPanel from '$lib/components/StatsPanel.svelte';
	import PreferencesPanel from '$lib/components/PreferencesPanel.svelte';

	// 当前激活的 Tab 状态
	let activeTab = $state('profile');

	// 动态组件映射
	const panels: Record<string, any> = {
		profile: ProfilePanel,
		security: SecurityPanel,
		stats: StatsPanel,
		prefs: PreferencesPanel
	};
</script>

<div class="container mx-auto max-w-6xl px-4 py-8">
	<div class="mb-6">
		<h1 class="text-3xl font-extrabold">账户设置</h1>
		<p class="text-base-content/60 transition-all duration-300">
			管理您的个人信息、安全选项及偏好。
		</p>
	</div>

	<div class="flex flex-col items-start gap-8 md:flex-row">
		<!-- 左侧：Sidebar -->
		<aside class="sticky top-20 w-full flex-shrink-0 md:w-64">
			<!-- 使用 bind:activeTab 实现双向绑定 -->
			<Sidebar bind:activeTab />
		</aside>

		<!-- 右侧：内容区 -->
		<main class="w-full min-w-0 flex-1">
			<!-- 简单的淡入动画容器 -->
			<div class="animate-fade-in">
				<!-- Svelte 动态组件 -->
				<svelte:component this={panels[activeTab]} class=" transition-all duration-300" />
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
