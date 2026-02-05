<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { uploadSession, type UploadedFile } from '$lib/state/upload.svelte';

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

		processImagesBatch(images);

		return () => {
			images.forEach((i) => {
				URL.revokeObjectURL(i.previewUrl!);
			});
		};
	});

	async function processImage(image: UploadedFile) {
		image.name = image.rawFile.name;
		image.size = image.rawFile.size;
		image.type = image.rawFile.type;
		image.previewUrl = URL.createObjectURL(image.rawFile);

		try {
			const img = await loadImage(image.rawFile);
			image.height = img.height;
			image.width = img.width;
		} catch (error) {
			console.error(error);
			image.height = 0;
			image.width = 0;
		}
	}

	async function processImagesBatch(images: UploadedFile[], batchSize = 3) {
		for (let i = 0; i < images.length; i += batchSize) {
			const batch = images.slice(i, i + batchSize);
			await Promise.all(batch.map(processImage));
		}
	}

	const loadImage = (file: File): Promise<HTMLImageElement> => {
		return new Promise((resolve, reject) => {
			const img = new Image();
			const url = URL.createObjectURL(file);

			img.onload = () => {
				resolve(img);
				URL.revokeObjectURL(url);
			};

			img.onerror = () => {
				URL.revokeObjectURL(url);
				reject(new Error(`Failed to load image: ${file.name}`));
			};

			img.src = url;
		});
	};
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
			<a href={resolve('/')} class="btn btn-ghost transition-all duration-300">继续上传</a>
		</div>
	</div>
{/if}
