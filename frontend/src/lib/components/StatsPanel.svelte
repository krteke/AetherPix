<script lang="ts">
	import { userState } from '$lib/state/user.svelte';

	// 计算存储百分比
	let usagePercent = $derived((userState.stats.storageUsed / userState.stats.storageLimit) * 100);

	// 进度条颜色逻辑
	let progressColor = $derived(
		usagePercent > 90
			? 'progress-error'
			: usagePercent > 70
				? 'progress-warning'
				: 'progress-primary'
	);
</script>

<div class="flex flex-col gap-6">
	<!-- 顶部概览卡片 -->
	<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
		<div class="stats border border-base-200 shadow transition-all duration-300">
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
							d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
						></path></svg
					>
				</div>
				<div class="stat-title">总上传数</div>
				<div class="stat-value text-primary">{userState.stats.totalUploads}</div>
				<div class="stat-desc">本月新增 12 张</div>
			</div>
		</div>

		<div class="stats border border-base-200 shadow transition-all duration-300">
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
							d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
						></path><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
						></path></svg
					>
				</div>
				<div class="stat-title">总浏览量</div>
				<div class="stat-value text-secondary">
					{(userState.stats.totalViews / 1000).toFixed(1)}k
				</div>
				<div class="stat-desc">↗︎ 14% (环比)</div>
			</div>
		</div>

		<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
			<div class="card-body justify-center p-5">
				<div class="mb-2 flex items-end justify-between">
					<span class="text-sm font-bold opacity-70">存储空间</span>
					<span class="text-xs opacity-50"
						>{userState.stats.storageUsed}MB / {userState.stats.storageLimit}MB</span
					>
				</div>
				<progress class="progress {progressColor} w-full" value={usagePercent} max="100"></progress>
				<div class="mt-1 text-right font-mono text-xs">{usagePercent.toFixed(1)}% 已用</div>
			</div>
		</div>
	</div>

	<!-- 最近上传列表 -->
	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body transition-all duration-300">
			<h3 class="mb-2 card-title text-base">最近上传记录</h3>
			<div class="overflow-x-auto">
				<table class="table table-zebra transition-all duration-300">
					<thead>
						<tr>
							<th>文件名</th>
							<th>大小</th>
							<th>浏览</th>
							<th>日期</th>
							<th>操作</th>
						</tr>
					</thead>
					<tbody>
						{#each userState.stats.recentUploads as file, i (i)}
							<tr class=" transition-all duration-300">
								<td>
									<div class="flex items-center gap-3">
										<div class="placeholder avatar">
											<div class="h-8 w-8 rounded bg-neutral text-neutral-content">
												<span class="text-xs">IMG</span>
											</div>
										</div>
										<div class="max-w-[120px] truncate text-sm font-bold">{file.name}</div>
									</div>
								</td>
								<td class="font-mono text-xs">{file.size}</td>
								<td>{file.views}</td>
								<td class="text-xs opacity-70">{file.date}</td>
								<td><button class="btn btn-ghost btn-xs">详情</button></td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="mt-2 card-actions justify-center">
				<button class="btn btn-ghost btn-xs">查看全部历史</button>
			</div>
		</div>
	</div>
</div>
