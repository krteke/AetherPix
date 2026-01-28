<script lang="ts">
	import type { UploadedFile } from '$lib/state/upload.svelte';

	let { image } = $props<{ image: UploadedFile }>();

	// 链接类型定义
	const linkTypes = ['Raw Link', 'Markdown', 'HTML', 'BBCode'];
	let selectedType = $state('Markdown');

	// 根据选择的类型生成链接
	let displayLink = $derived.by(() => {
		switch (selectedType) {
			case 'Raw Link':
				return image.url;
			case 'Markdown':
				return `![${image.name}](${image.url})`;
			case 'HTML':
				return `<img src="${image.url}" alt="${image.name}" />`;
			case 'BBCode':
				return `[img]${image.url}[/img]`;
			default:
				return image.url;
		}
	});

	function copyLink() {
		navigator.clipboard.writeText(displayLink);
		alert('链接已复制！');
	}
</script>

<div class="mx-auto grid max-w-5xl grid-cols-1 gap-6 md:grid-cols-3">
	<!-- 左侧：链接工具 -->
	<div class="card border border-base-200 bg-base-100 shadow-xl md:col-span-2">
		<div class="card-body">
			<h3 class="mb-4 card-title text-base">获取引用链接</h3>

			<div class="flex flex-col gap-4">
				<div class="join w-full">
					<!-- 1. 下拉菜单选择格式 -->
					<select
						class="select-bordered select join-item w-32 bg-base-200"
						bind:value={selectedType}
					>
						{#each linkTypes as type, i (i)}
							<option value={type}>{type}</option>
						{/each}
					</select>

					<!-- 2. 链接显示框 -->
					<input
						type="text"
						class="input-bordered input join-item w-full font-mono text-sm"
						value={displayLink}
						readonly
					/>

					<!-- 3. 复制按钮 -->
					<button class="btn join-item btn-primary" onclick={copyLink}> 复制 </button>
				</div>

				<p class="mt-2 text-xs text-base-content/60">
					当前选择: <span class="font-bold text-primary">{selectedType}</span>。
					支持直接粘贴到您的文档中。
				</p>
			</div>
		</div>
	</div>

	<!-- 右侧：文件详情 -->
	<div class="card h-fit border border-base-200 bg-base-100 shadow-xl">
		<div class="card-body p-5">
			<h3 class="mb-2 card-title text-sm tracking-wide uppercase opacity-70">文件详情</h3>

			<div class="flex flex-col gap-3 text-sm">
				<div class="flex justify-between border-b border-base-200 pb-2">
					<span class="opacity-60">文件名</span>
					<span class="max-w-[150px] truncate font-medium" title={image.name}>{image.name}</span>
				</div>
				<div class="flex justify-between border-b border-base-200 pb-2">
					<span class="opacity-60">文件大小</span>
					<span class="font-mono">{image.size}</span>
				</div>
				<div class="flex justify-between border-b border-base-200 pb-2">
					<span class="opacity-60">分辨率</span>
					<span class="font-mono">{image.width} x {image.height}</span>
				</div>
				<div class="flex justify-between">
					<span class="opacity-60">格式</span>
					<span class="badge badge-ghost badge-sm">{image.type}</span>
				</div>
			</div>

			<div class="mt-4 card-actions">
				<a href={image.url} download={image.name} class="btn w-full btn-outline btn-sm">
					下载文件
				</a>
			</div>
		</div>
	</div>
</div>
