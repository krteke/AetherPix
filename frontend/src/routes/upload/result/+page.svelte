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

	// 如果没有图片（比如用户直接刷新了这个页面），重定向回首页
	onMount(() => {
		if (images.length === 0) {
			goto(resolve('/'));
		}
	});
</script>

{#if images.length > 0}
	<div class="container mx-auto px-4 py-8">
		<!-- 顶部导航面包屑 -->
		<div class="breadcrumbs mb-6 text-sm">
			<ul>
				<li><a href="/">首页</a></li>
				<li>上传结果</li>
			</ul>
		</div>

		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold">上传成功</h1>
			<p class="text-base-content/60 transition-all duration-300">共 {images.length} 张图片</p>
		</div>

		<!-- 1. 走马灯 Component -->
		<!-- 绑定 activeIndex，这样父组件知道当前在看哪张图 -->
		<ResultCarousel {images} bind:activeIndex />

		<div class="divider my-8"></div>

		<!-- 2. 信息详情 Component -->
		<!-- 传入当前激活的图片对象 -->
		<ResultInfo image={images[activeIndex]} />

		<div class="mt-12 flex justify-center">
			<a href="/" class="btn btn-ghost">继续上传</a>
		</div>
	</div>
{/if}
