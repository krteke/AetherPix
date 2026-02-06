<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';
	import type { Image, ListViewResponse } from '$lib/types/type';
	import { fade, fly } from 'svelte/transition';

	let viewMode = $state<'grid' | 'list'>('grid');
	let isLoading = $state(true);
	let images: Image[] = $state([]);

	// 分页状态
	let currentPage = $state(1);
	let pageSize = $state(12);

	let totalItems = $state(0);
	let totalPages = $derived(0);

	async function fetchImages() {
		isLoading = true;
		images = [];
		try {
			const res = await fetch(`/api/view/list?page=${currentPage - 1}&limit=${pageSize}`);

			if (!res.ok) {
				isLoading = false;
				throw new Error('Failed to fetch images');
			}

			const data: ListViewResponse = await res.json();

			totalItems = data.total;
			totalPages = data.pages;
			images = data.images;
		} catch (error) {
			console.error('Error fetching images:', error);
		}

		isLoading = false;
	}

	$effect(() => {
		fetchImages();
	});

	function copyLink(url: string) {
		navigator.clipboard.writeText(url);
		msg.alert('链接已复制到剪切板: \n' + url, '复制成功', 'success');
	}

	// TODO: implement deleteImage
	function deleteImage(id: number) {
		if (confirm('确定要删除这张图片吗？')) {
			images = images.filter((img) => img.id !== id);
			if (images.length === 0 && currentPage > 1) {
				currentPage--;
			}
		}
	}
</script>

<div class="container mx-auto max-w-7xl px-4 py-8">
	<!-- 顶部工具栏 -->
	<div class="mb-8 flex flex-col items-center justify-between gap-4 md:flex-row">
		<div>
			<h1 class="text-3xl font-bold">我的图库</h1>
			<p class="mt-1 text-sm text-base-content/60 transition-all duration-300">
				管理您的所有上传资源 ({totalItems} items)
			</p>
		</div>

		<div class="flex w-full gap-3 md:w-auto">
			<!-- 视图切换按钮 -->
			<div class="join rounded-lg bg-base-200 p-1 transition-all duration-300">
				<button
					class="btn join-item border-none bg-base-content shadow-none transition-all duration-300 btn-sm {viewMode ===
					'grid'
						? 'text-primary shadow-sm'
						: 'bg-transparent text-base-content/50'}"
					onclick={() => (viewMode = 'grid')}
					title="网格视图"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
						/></svg
					>
				</button>
				<button
					class="btn join-item border-none bg-base-content shadow-none transition-all duration-300 btn-sm {viewMode ===
					'list'
						? 'text-primary shadow-sm'
						: 'bg-transparent text-base-content/50'}"
					onclick={() => (viewMode = 'list')}
					title="列表视图"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16M4 18h16"
						/></svg
					>
				</button>
			</div>
		</div>
	</div>

	<!-- 内容区域 -->
	{#if isLoading}
		<!-- 骨架屏 Loading -->
		<div class="grid grid-cols-2 gap-6 transition-all duration-300 md:grid-cols-4">
			{#each Array(8) as _, i (i)}
				<div class="flex flex-col gap-4">
					<div class="h-48 w-full skeleton rounded-xl"></div>
					<div class="h-4 w-28 skeleton"></div>
					<div class="h-4 w-full skeleton"></div>
				</div>
			{/each}
		</div>
	{:else if images.length === 0}
		<!-- 空状态 -->
		<div
			class="rounded-box border border-dashed border-base-300 bg-base-100 py-20 text-center transition-all duration-300"
		>
			<div class="mb-4 flex justify-center text-base-content/30 transition-all duration-300">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-24 w-24"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
					/></svg
				>
			</div>
			<h3 class="text-lg font-bold transition-all duration-300">没有找到图片</h3>
			<p class="opacity-60 transition-all duration-300">尝试更改搜索词或上传新图片。</p>
		</div>
	{:else}
		<!-- 模式 A: 网格视图 -->
		{#if viewMode === 'grid'}
			<div class="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-5">
				{#each images as img (img.id)}
					<div
						class="group card border border-base-200 bg-base-100 shadow-md transition-all duration-300 hover:-translate-y-1 hover:shadow-xl"
						in:fly={{ y: 20, duration: 300 }}
					>
						<figure class="relative aspect-square overflow-hidden bg-base-200">
							<img
								src={img.previewUrl}
								alt={img.name}
								class="h-full w-full object-cover"
								loading="lazy"
							/>

							<!-- 悬浮操作层 -->
							<div
								class="absolute inset-0 flex items-center justify-center gap-2 bg-black/50 opacity-0 backdrop-blur-[2px] transition-opacity group-hover:opacity-100"
							>
								<button
									class="btn-white btn btn-circle transition-all duration-300 btn-sm"
									onclick={() => copyLink(img.originalUrl)}
									title="复制链接"
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="h-4 w-4"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
										><path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
										/></svg
									>
								</button>
								<button
									class="btn btn-circle text-white transition-all duration-300 btn-sm btn-error"
									onclick={() => deleteImage(img.id)}
									title="删除"
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="h-4 w-4"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
										><path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
										/></svg
									>
								</button>
							</div>
						</figure>
						<div class="card-body gap-1 p-3">
							<h3 class="truncate text-sm font-bold" title={img.name}>{img.name}</h3>
							<div
								class="flex justify-between text-xs text-base-content/60 transition-all duration-300"
							>
								<!-- <span>{img.size}</span> -->
							</div>
						</div>
					</div>
				{/each}
			</div>

			<!-- 模式 B: 列表视图 -->
		{:else}
			<div
				class="overflow-x-auto rounded-lg border border-base-200 bg-base-100 shadow transition-all duration-300"
			>
				<table class="table table-zebra transition-all duration-300">
					<thead>
						<tr class="transition-all duration-300">
							<th>预览</th>
							<th>文件名</th>
							<!-- <th>大小</th> -->
							<th class="text-right">操作</th>
						</tr>
					</thead>
					<tbody class="transition-all duration-300">
						{#each images as img (img.id)}
							<tr in:fade class="transition-all duration-300">
								<td>
									<div class="avatar">
										<div class="mask h-12 w-12 bg-base-200 mask-squircle">
											<img src={img.previewUrl} alt={img.name} loading="lazy" />
										</div>
									</div>
								</td>
								<td>
									<div class="max-w-50 truncate font-bold" title={img.name}>{img.name}</div>
								</td>
								<!-- <td class="font-mono text-sm">{img.size}</td> -->
								<td class="text-right">
									<button class="btn btn-ghost btn-xs" onclick={() => copyLink(img.originalUrl)}
										>复制链接</button
									>
									<button
										class="btn text-error btn-ghost btn-xs"
										onclick={() => deleteImage(img.id)}>删除</button
									>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		<!-- 底部翻页控制器 -->
		<div class="mt-10 flex justify-center">
			<div class="join border border-base-200 bg-base-100 shadow-sm transition-all duration-300">
				<!-- 上一页 -->
				<button
					class="btn join-item transition-all duration-300 btn-sm"
					disabled={currentPage === 1}
					onclick={() => currentPage--}
				>
					«
				</button>

				{#each Array(totalPages) as _, i (i)}
					{@const pageNum = i + 1}
					{#if pageNum === 1 || pageNum === totalPages || (pageNum >= currentPage - 1 && pageNum <= currentPage + 1)}
						<button
							class="btn join-item transition-all duration-300 btn-sm {currentPage === pageNum
								? 'btn-active btn-primary'
								: ''}"
							onclick={() => (currentPage = pageNum)}
						>
							{pageNum}
						</button>
					{:else if pageNum === currentPage - 2 || pageNum === currentPage + 2}
						<button class="btn btn-disabled join-item transition-all duration-300 btn-sm"
							>...</button
						>
					{/if}
				{/each}

				<!-- 下一页 -->
				<button
					class="btn join-item btn-sm"
					disabled={currentPage === totalPages}
					onclick={() => currentPage++}
				>
					»
				</button>
			</div>
		</div>

		<!-- 分页信息 -->
		<div class="mt-2 text-center text-xs text-base-content/40 transition-all duration-300">
			第 {currentPage} 页 / 共 {totalPages} 页
		</div>
	{/if}
</div>
