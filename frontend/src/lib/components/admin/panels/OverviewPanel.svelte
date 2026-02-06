<script lang="ts">
	import { adminStore } from '$lib/state/admin.svelte';
	import { onMount } from 'svelte';

	// 模拟服务器负载波动
	onMount(() => {
		const interval = setInterval(() => {
			// 随机波动 CPU 和 RAM
			const cpuChange = Math.floor(Math.random() * 10) - 5;
			const ramChange = Math.floor(Math.random() * 6) - 3;

			adminStore.server.cpu = Math.max(5, Math.min(99, adminStore.server.cpu + cpuChange));
			adminStore.server.ram = Math.max(10, Math.min(99, adminStore.server.ram + ramChange));
		}, 2000);
		return () => clearInterval(interval);
	});

	// 颜色计算
	let cpuColor = $derived(
		adminStore.server.cpu > 80
			? 'text-error'
			: adminStore.server.cpu > 50
				? 'text-warning'
				: 'text-primary'
	);
	let ramColor = $derived(adminStore.server.ram > 80 ? 'progress-error' : 'progress-info');
</script>

<div class="mb-8 grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
	<!-- 业务统计卡片 -->
	<div class="stats border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="stat">
			<div class="stat-figure text-secondary">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					class="inline-block h-8 w-8 stroke-current"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
					></path></svg
				>
			</div>
			<div class="stat-title">图片总数</div>
			<div class="stat-value">34.2K</div>
			<div class="stat-desc">今日新增 145 张</div>
		</div>
	</div>

	<div class="stats border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="stat">
			<div class="stat-figure text-primary">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					class="inline-block h-8 w-8 stroke-current"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0z"
					></path></svg
				>
			</div>
			<div class="stat-title">注册用户</div>
			<div class="stat-value">{adminStore.users.length}</div>
			<div class="stat-desc">活跃用户 85%</div>
		</div>
	</div>

	<div class="stats border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="stat">
			<div class="stat-title">存储占用</div>
			<div class="stat-value text-accent">42%</div>
			<div class="stat-desc">450GB / 1TB</div>
		</div>
	</div>

	<div class="stats border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="stat">
			<div class="stat-title">系统运行</div>
			<div class="stat-value font-mono text-lg">{adminStore.server.uptime}</div>
			<div class="stat-desc text-xs">{adminStore.server.version}</div>
		</div>
	</div>
</div>

<!-- 服务器负载面板 -->
<div class="card border border-base-200 bg-base-100 shadow-xl transition-all duration-300">
	<div class="card-body">
		<h3 class="mb-4 card-title">服务器负载 (Live)</h3>
		<div class="flex flex-col items-center justify-around gap-8 md:flex-row">
			<!-- CPU 环形进度条 -->
			<div class="flex flex-col items-center gap-2">
				<div
					class="radial-progress {cpuColor} border-4 border-base-200 bg-base-200 transition-all duration-300"
					style="--value:{adminStore.server.cpu}; --size:8rem; --thickness: 0.8rem;"
					role="progressbar"
				>
					<span class="text-3xl font-bold">{adminStore.server.cpu}%</span>
				</div>
				<span class="font-bold opacity-70">CPU 使用率</span>
			</div>

			<div class="divider md:divider-horizontal"></div>

			<!-- 内存 进度条 -->
			<div class="flex w-full max-w-xs flex-col gap-2">
				<div class="flex justify-between">
					<span class="font-bold opacity-70">RAM 使用率</span>
					<span class="font-mono">{adminStore.server.ram}%</span>
				</div>
				<progress class="progress {ramColor} h-4 w-full" value={adminStore.server.ram} max="100"
				></progress>
				<div class="text-right text-xs opacity-50">8GB / 16GB</div>
			</div>

			<div class="divider md:divider-horizontal"></div>

			<!-- 磁盘 进度条 -->
			<div class="flex w-full max-w-xs flex-col gap-2">
				<div class="flex justify-between">
					<span class="font-bold opacity-70">NVMe 磁盘</span>
					<span class="font-mono">{adminStore.server.disk}%</span>
				</div>
				<progress
					class="progress h-4 w-full progress-success"
					value={adminStore.server.disk}
					max="100"
				></progress>
				<div class="text-right text-xs opacity-50">Data Partition</div>
			</div>
		</div>
	</div>
</div>
