<script lang="ts">
	import { goto } from '$app/navigation';
	import { uploadSession } from '$lib/state/upload.svelte';
	import { resolve } from '$app/paths';
	import { uploader, type UploadItem, type UploadLocation } from '$lib/state/uploader.svelte';
	import UploadProgress from '$lib/components/UploadProgress.svelte';
	import { auth } from '$lib/state/auth.svelte';

	let isDragging = $state(false);
	let quality = $state(uploader.quality);

	// 设置选项
	let storageLocation: UploadLocation = $state(uploader.uploadLocation);
	let isPublic = $state(uploader.isPublic);

	// 核心队列
	let queue = $derived(uploader.queue);
	// 标记是否已经点击过开始，用于控制 Banner 显示时机
	let hasStarted = $state(false);

	$effect(() => {
		queue.forEach((item) => {
			item.isPublic = isPublic;
			item.quality = quality;
			item.location = storageLocation;
		});
	});

	function handleFiles(files: FileList | null) {
		if (!files) return;
		uploader.addFiles(files);
		isDragging = false;
	}

	function startUpload() {
		hasStarted = true;
		// 触发并发调度器
		uploader.processQueue();
	}

	function retryFailedItems() {
		uploader.retryFailedItems();
	}

	function clearQueue() {
		hasStarted = false;
		uploader.clearQueue();
	}

	function retryUpload(item: UploadItem) {
		uploader.retry(item);
	}

	// 计算属性：是否显示完成 Banner
	// 逻辑：队列不为空 && 没有正在上传的任务 && 已经开始过流程
	let isBatchComplete = $derived(
		queue.length > 0 && hasStarted && !queue.some((i) => i.status === 'uploading')
	);

	// 计算属性：成功数量
	let successCount = $derived(queue.filter((i) => i.status === 'success').length);

	function goToDetails() {
		const completed = queue.filter((i) => i.status === 'success' && i.result).map((i) => i.result!);

		uploadSession.setBatch(completed);
		goto(resolve('/upload/result/'));
	}
</script>

<div class="mx-auto max-w-6xl px-4 py-10">
	<div class="mb-10 text-center">
		<h1 class="mb-2 text-4xl font-extrabold text-base-content transition-all duration-300">
			上传图片
		</h1>
		<p class="text-base-content/60 transition-all duration-300">极速上传，全球分发</p>
	</div>

	<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-3">
		<div class="space-y-6 lg:col-span-2">
			<div
				class="card cursor-pointer border-2 border-dashed transition-all duration-300
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
				<div class="card-body items-center py-12 text-center">
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
					<p class="text-sm opacity-60">支持 JPG, PNG, GIF, WebP</p>
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

			<!-- 文件列表 -->
			{#if queue.length > 0}
				<div class="mb-2 flex items-center justify-between">
					<span class="text-sm font-bold text-base-content/50 transition-all duration-300"
						>文件列表 ({queue.length})</span
					>
					{#if !queue.some((i) => i.status === 'uploading')}
						<button class="btn transition-all duration-300 btn-sm btn-primary" onclick={clearQueue}>
							清空
						</button>
					{/if}
					{#if !hasStarted || queue.some((i) => i.status === 'pending')}
						<button
							class="btn animate-pulse transition-all duration-300 btn-sm btn-primary"
							onclick={startUpload}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="mr-1 h-4 w-4"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
							</svg>
							开始上传
						</button>
					{:else if queue.some((i) => i.status === 'error')}
						<button
							class="btn transition-all duration-300 btn-sm btn-warning"
							onclick={retryFailedItems}
						>
							重试失败文件
						</button>
					{/if}
				</div>

				<div class="flex flex-col gap-2">
					{#each queue as item (item.name)}
						<UploadProgress {item} onRetry={() => retryUpload(item)} />
					{/each}
				</div>

				<!-- 完成提示框 -->
				{#if isBatchComplete}
					<div
						class="animate-bounce-in mt-6 alert transition-all duration-300 {successCount ===
						queue.length
							? 'alert-success'
							: 'alert-warning'} shadow-lg"
					>
						{#if successCount === queue.length}
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
						{:else}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6 shrink-0 stroke-current"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
								/></svg
							>
						{/if}

						<div class=" transition-all duration-300">
							<h3 class="font-bold">处理完成</h3>
							<div class="text-xs">成功: {successCount} / 总数: {queue.length}</div>
							{#if successCount < queue.length}
								<div class="text-xs opacity-70">部分文件上传失败，请点击右侧按钮重试。</div>
							{/if}
						</div>

						{#if successCount > 0}
							<button
								class="btn border-none bg-base-100 text-base-content transition-all duration-300 btn-sm hover:bg-base-200"
								onclick={goToDetails}
							>
								查看成功详情
							</button>
						{/if}
					</div>
				{/if}
			{/if}
		</div>

		<div class="lg:col-span-1">
			<div
				class="card sticky top-8 border border-base-200 bg-base-100 shadow-xl transition-all duration-300"
			>
				<div class="card-body">
					<h2 class="mb-4 card-title text-lg">上传设置</h2>

					<!-- 存储位置选择 -->
					<div class="w-full">
						<label class="label" for="location-select">
							<span class="font-medium">存储位置</span>
						</label>
						<select
							id="location-select"
							class="select-bordered select w-full transition-all duration-300"
							bind:value={storageLocation}
						>
							<option value="r2">Cloudflare R2</option>
							<!-- <option value="s3">Amazon S3</option> -->
							<!-- <option value="github">GitHub Repo</option> -->
							<option value="local">Local VPS</option>
						</select>
						<div class="label">
							<span class="opacity-60">当前剩余空间: 充足</span>
						</div>
					</div>

					{#if auth.exists}
						<div class="divider my-2"></div>

						<!-- 访问权限 -->
						<div>
							<label class="label cursor-pointer">
								<span class="font-medium">公开访问</span>
								<input
									type="checkbox"
									class="toggle transition-all duration-300 toggle-primary"
									bind:checked={isPublic}
								/>
							</label>
							<div class="px-1 text-xs text-base-content/60 transition-all duration-300">
								{isPublic
									? '任何持有链接的人都可以访问图片。'
									: '不生成访问链接，只有设置密码或时效后才生成临时链接。'}
							</div>
						</div>
					{/if}
					{#if storageLocation == 'local'}
						<div class="divider my-2"></div>
						<div>
							<label class="label cursor-pointer" for="quality">
								<span class="font-medium">图片质量</span>
							</label>
							<input
								id="quality"
								type="range"
								class="range range-primary transition-all duration-300 range-xs"
								min="1"
								max="100"
								bind:value={quality}
							/>
							<div class="px-1 text-xs text-base-content/60 transition-all duration-300">
								{quality}%
							</div>
						</div>
					{/if}
					<div class="divider my-2"></div>

					<!-- 信息汇总 -->
					<div class="space-y-2 rounded-lg bg-base-200 p-3 text-sm transition-all duration-300">
						<div class="flex justify-between">
							<span class="opacity-70">待传文件:</span>
							<span class="font-bold">{queue.filter((i) => i.status === 'pending').length}</span>
						</div>
						<div class="flex justify-between">
							<!-- <span class="opacity-70">预计耗时:</span>
							<span>~{queue.length * 2}s</span> -->
							<span class="opacity-70">失败文件:</span>
							<span class="font-bold">{queue.filter((i) => i.status === 'error').length}</span>
						</div>
						<div class="flex justify-between">
							<span class="opacity-70">成功文件:</span>
							<span class="font-bold">{queue.filter((i) => i.status === 'success').length}</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
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
