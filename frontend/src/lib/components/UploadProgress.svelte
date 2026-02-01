<script lang="ts">
	import type { UploadItem } from '$lib/state/uploader.svelte';

	let { item, onRetry } = $props<{
		item: UploadItem;
		onRetry: () => void;
	}>();
</script>

<div class="card mb-3 border border-base-200 bg-base-100 shadow-sm transition-all duration-300">
	<div class="card-body flex-row items-center gap-4 p-4">
		<div class="placeholder avatar">
			<div class="h-12 w-12 rounded bg-neutral text-neutral-content">
				<span class="text-xs uppercase">{item.name.split('.').pop()?.slice(0, 4)}</span>
			</div>
		</div>

		<div class="min-w-0 flex-1">
			<div class="mb-1 flex justify-between">
				<div class="flex flex-col">
					<span class="truncate pr-2 text-sm font-bold" title={item.name}>{item.name}</span>
					<span class="text-xs opacity-50">{(item.file.size / 1024 / 1024).toFixed(2)} MB</span>
				</div>

				<div class="flex flex-col items-end">
					<span class="text-xs font-bold opacity-70">
						{#if item.status === 'pending'}等待中
						{:else if item.status === 'error'}失败
						{:else if item.status === 'success'}完成
						{:else}
							{Math.round(item.progress)}%
							<span class="ml-1 text-primary">{item.speed}</span>
						{/if}
					</span>
				</div>
			</div>

			<progress
				class="progress w-full transition-all duration-300
                {item.status === 'success'
					? 'progress-success'
					: item.status === 'error'
						? 'progress-error'
						: item.status === 'pending'
							? 'progress-secondary'
							: 'progress-primary'}"
				value={item.status === 'pending' ? 0 : item.progress}
				max="100"
			>
			</progress>
		</div>

		<div class="flex w-20 justify-end">
			{#if item.status === 'success'}
				<div class="text-success">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						/></svg
					>
				</div>
			{:else if item.status === 'error'}
				<button
					class="btn transition-all duration-300 btn-outline btn-xs btn-error"
					onclick={onRetry}>重试</button
				>
			{:else if item.status === 'uploading'}
				<button class="btn btn-circle btn-ghost btn-xs" onclick={() => item.xhr?.abort()}>✕</button>
			{:else}
				<span class="badge badge-ghost badge-sm">...</span>
			{/if}
		</div>
	</div>
</div>
