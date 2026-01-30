<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let status = $state<'loading' | 'success' | 'error'>('loading');
	let message = $state('正在验证您的邮箱...');

	onMount(async () => {
		const token = page.url.searchParams.get('token');

		if (!token) {
			status = 'error';
			message = '无效的验证链接 (Token 缺失)。';
			return;
		}

		try {
			const res = await fetch(`/api/auth/verify/${token}`);

			if (res.ok) {
				status = 'success';
				message = '邮箱验证成功！您现在可以登录了。3秒钟后将自动跳转...';
				setTimeout(() => goto(resolve('/login')), 3000);
			} else {
				status = 'error';
				message = '验证链接已失效或过期，请尝试重新登录获取新邮件。';
			}
		} catch (e) {
			status = 'error';
			message = '网络请求失败，请稍后重试。';
			console.error(e);
		}
	});
</script>

<div class="flex min-h-screen items-center justify-center bg-base-200 p-4">
	<div class="card w-full max-w-md bg-base-100 text-center shadow-xl">
		<div class="card-body items-center">
			{#if status === 'loading'}
				<span class="loading mb-4 loading-lg loading-spinner text-primary"></span>
				<h2 class="card-title">验证中...</h2>
				<p>{message}</p>
			{:else if status === 'success'}
				<div class="mb-4 text-success">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-16 w-16"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
				</div>
				<h2 class="card-title text-success">验证成功</h2>
				<p class="mb-4">{message}</p>
				<a href={resolve('/login')} class="btn w-full btn-primary">立即登录</a>
			{:else if status === 'error'}
				<div class="mb-4 text-error">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-16 w-16"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
				</div>
				<h2 class="card-title text-error">验证失败</h2>
				<p class="mb-4">{message}</p>
				<a href={resolve('/login')} class="btn w-full btn-outline">返回登录页</a>
			{/if}
		</div>
	</div>
</div>
