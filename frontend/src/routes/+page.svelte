<script lang="ts">
	import { goto } from '$app/navigation';
	import { uploadSession, type UploadedFile } from '$lib/state/upload.svelte';
	import UploadProgress from '$lib/components/UploadProgress.svelte';
	import { resolve } from '$app/paths';

	let isDragging = $state(false);

	// 待上传队列
	let uploadQueue: File[] = $state([]);
	// 已完成的图片结果
	let completedUploads: UploadedFile[] = $state([]);

	function handleFiles(files: FileList | null) {
		if (!files || files.length === 0) return;
		// 将新文件加入队列，触发 UploadProgress 组件渲染
		uploadQueue = [...uploadQueue, ...Array.from(files)];
		isDragging = false;
	}

	// 当单个文件上传完成后的回调
	function onUploadComplete(data: UploadedFile) {
		completedUploads = [...completedUploads, data];
	}

	// 检查是否全部完成
	let isAllDone = $derived(
		uploadQueue.length > 0 && uploadQueue.length === completedUploads.length
	);

	function goToDetails() {
		// 保存数据到全局状态
		uploadSession.setBatch(completedUploads);
		// 跳转到新的详情页
		goto(resolve('/upload/result/'));
	}
</script>

<div class="mx-auto max-w-3xl py-10">
	<div class="mb-8 text-center">
		<h1 class="mb-2 text-4xl font-extrabold text-base-content transition-all duration-300">
			上传图片
		</h1>
		<p class="text-base-content/60 transition-all duration-300">极速上传，全球分发</p>
	</div>

	<!-- 拖拽上传区域 -->
	<div
		class="card mb-8 cursor-pointer border-2 border-dashed transition-all duration-300
        {isDragging
			? 'scale-[1.01] border-primary bg-primary/10'
			: 'border-base-300 bg-base-100 hover:border-primary/50'}"
		tabindex="0"
		role="button"
		ondragover={(e) => {
			e.preventDefault();
			isDragging = true;
		}}
		ondragleave={() => (isDragging = false)}
		ondrop={(e) => {
			e.preventDefault();
			handleFiles(e.dataTransfer?.files || null);
		}}
	>
		<div class="card-body items-center py-10 text-center">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="mb-4 h-12 w-12 text-primary"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"
				/>
			</svg>
			<h3 class="text-lg font-bold">点击或拖拽文件</h3>
			<input
				type="file"
				id="fileInput"
				class="hidden"
				multiple
				accept="image/*"
				onchange={(e) => handleFiles(e.currentTarget.files)}
			/>
			<label for="fileInput" class="btn mt-4 transition-all duration-300 btn-sm btn-primary"
				>选择文件</label
			>
		</div>
	</div>

	<!-- 上传进度列表 -->
	{#if uploadQueue.length > 0}
		<div class="divider text-sm font-bold text-base-content/50">上传队列</div>

		{#each uploadQueue as file (file.name + file.lastModified)}
			<UploadProgress {file} onComplete={onUploadComplete} />
		{/each}

		<!-- 全部完成后显示的 Banner -->
		{#if isAllDone}
			<div class="animate-bounce-in mt-6 alert alert-success shadow-lg transition-all duration-300">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6 shrink-0 stroke-current"
					fill="none"
					viewBox="0 0 24 24"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
					/></svg
				>
				<div>
					<h3 class="font-bold">上传完成!</h3>
					<div class="text-xs">成功上传 {completedUploads.length} 张图片</div>
				</div>
				<button
					class="btn text-success-content transition-all duration-300 btn-sm btn-success"
					onclick={goToDetails}
				>
					查看详情与链接
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.animate-bounce-in {
		animation: bounceIn 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275) both;
	}
	@keyframes bounceIn {
		0% {
			transform: scale(0.9);
			opacity: 0;
		}
		100% {
			transform: scale(1);
			opacity: 1;
		}
	}
</style>
