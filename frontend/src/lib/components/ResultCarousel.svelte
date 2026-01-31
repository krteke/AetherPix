<script lang="ts">
	import type { UploadedFile } from '$lib/state/upload.svelte';

	let { images, activeIndex = $bindable(0) } = $props<{
		images: UploadedFile[];
		activeIndex: number;
	}>();

	function next() {
		activeIndex = (activeIndex + 1) % images.length;
	}

	function prev() {
		activeIndex = (activeIndex - 1 + images.length) % images.length;
	}
</script>

<div class="group relative mx-auto mb-8 w-full max-w-4xl">
	<!-- 图片展示区 -->
	<div
		class="relative carousel h-[500px] w-full overflow-hidden rounded-box border border-base-200 bg-base-300 transition-all duration-300"
	>
		{#each images as img, index (index)}
			<div
				class="absolute carousel-item flex h-full w-full items-center justify-center transition-opacity duration-300
                {index === activeIndex ? 'z-10 opacity-100' : 'z-0 opacity-0'}"
			>
				<img src={img.url} alt={img.name} class="max-h-full max-w-full object-contain shadow-lg" />
			</div>
		{/each}
	</div>

	<!-- 导航按钮 (仅当多于1张时显示) -->
	{#if images.length > 1}
		<div
			class="absolute top-1/2 right-5 left-5 z-20 flex -translate-y-1/2 transform justify-between opacity-0 transition-opacity group-hover:opacity-100"
		>
			<button class="bg-opacity-70 btn btn-circle border-none text-white btn-neutral" onclick={prev}
				>❮</button
			>
			<button class="bg-opacity-70 btn btn-circle border-none text-white btn-neutral" onclick={next}
				>❯</button
			>
		</div>

		<!-- 底部指示器 -->
		<div class="mt-2 flex w-full justify-center gap-2 py-2">
			{#each images as _, i (i)}
				<button
					class="btn btn-circle transition-all duration-300 btn-xs {i === activeIndex
						? 'btn-primary'
						: 'bg-base-200 btn-ghost'}"
					onclick={() => (activeIndex = i)}
				></button>
			{/each}
		</div>
	{/if}
</div>
