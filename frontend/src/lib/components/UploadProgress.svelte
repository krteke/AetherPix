<script lang="ts">
	import { onMount } from 'svelte';

	let { file, onComplete } = $props<{
		file: File;
		onComplete: (data: any) => void;
	}>();

	let progress = $state(0);
	let status = $state<'uploading' | 'done'>('uploading');

	onMount(() => {
		// 模拟上传过程
		const interval = setInterval(() => {
			progress += Math.random() * 10;
			if (progress >= 100) {
				progress = 100;
				status = 'done';
				clearInterval(interval);
				// 模拟返回的数据
				onComplete({
					id: crypto.randomUUID(),
					url: URL.createObjectURL(file),
					name: file.name,
					size: (file.size / 1024 / 1024).toFixed(2) + ' MB',
					width: 1920, // 模拟数据
					height: 1080,
					type: file.type
				});
			}
		}, 200); // 速度快一点
		return () => clearInterval(interval);
	});
</script>

<div class="card mb-3 border border-base-200 bg-base-100 shadow-sm">
	<div class="card-body flex-row items-center gap-4 p-4">
		<!-- 图标 -->
		<div class="placeholder avatar">
			<div class="h-12 w-12 rounded bg-neutral text-neutral-content">
				<span class="text-xs uppercase">{file.name.split('.').pop()}</span>
			</div>
		</div>

		<!-- 进度条区域 -->
		<div class="flex-1">
			<div class="mb-1 flex justify-between">
				<span class="w-48 truncate text-sm font-bold">{file.name}</span>
				<span class="text-xs opacity-60">{Math.round(progress)}%</span>
			</div>
			<progress
				class="progress w-full {status === 'done' ? 'progress-success' : 'progress-primary'}"
				value={progress}
				max="100"
			>
			</progress>
		</div>

		<!-- 状态图标 -->
		<div class="flex w-8 justify-center">
			{#if status === 'done'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6 text-success"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M5 13l4 4L19 7"
					/>
				</svg>
			{:else}
				<span class="loading loading-sm loading-spinner text-primary"></span>
			{/if}
		</div>
	</div>
</div>
