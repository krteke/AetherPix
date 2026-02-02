<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { uploadSession } from '$lib/state/upload.svelte';

	import ResultCarousel from '$lib/components/ResultCarousel.svelte';
	import ResultInfo from '$lib/components/ResultInfo.svelte';
	import { resolve } from '$app/paths';

	// 从全局状态获取图片
	let images = $state(uploadSession.currentBatch);
	let activeIndex = $state(0);

	onMount(() => {
		if (images.length === 0) {
			goto(resolve('/'));
		}

		images.forEach((i) => {
			i.name = i.rawFile.name;
			i.size = i.rawFile.size;
			i.type = i.rawFile.type;
			i.previewUrl = URL.createObjectURL(i.rawFile);
			const { height, width } = getImageHW(i.rawFile);
			i.height = height;
			i.width = width;
		});

		return () => {
			images.forEach((i) => {
				URL.revokeObjectURL(i.previewUrl!);
			});
		};
	});

	function getImageHW(file: File): { height: number; width: number } {
		const img = new Image();
		const url = URL.createObjectURL(file);

		img.onload = () => {
			const height = img.height;
			const width = img.width;
			URL.revokeObjectURL(url);
			return { height, width };
		};

		img.onerror = () => {
			URL.revokeObjectURL(url);
			return { height: 0, width: 0 };
		};

		img.src = url;
		return { height: 0, width: 0 };
	}
</script>

{#if images.length > 0}
	<div class="container mx-auto px-4 py-8">
		<!-- 顶部导航面包屑 -->
		<div class="breadcrumbs mb-6 text-sm">
			<ul>
				<li><a href={resolve('/')}>首页</a></li>
				<li>上传结果</li>
			</ul>
		</div>

		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold">上传成功</h1>
			<p class="text-base-content/60 transition-all duration-300">共 {images.length} 张图片</p>
		</div>

		<ResultCarousel {images} bind:activeIndex />

		<div class="divider my-8"></div>

		<ResultInfo image={images[activeIndex]} />

		<div class="mt-12 flex justify-center">
			<a href={resolve('/')} class="btn btn-ghost">继续上传</a>
		</div>
	</div>
{/if}
