<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let token = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let status = $state<'idle' | 'submitting' | 'success' | 'error'>('idle');
	let errorMsg = $state('');

	let isValid = $derived(password.length >= 6 && password === confirmPassword);

	onMount(() => {
		token = page.url.searchParams.get('token') || '';
		if (!token) {
			status = 'error';
			errorMsg = '链接无效或缺少令牌 (Token)，请重新申请重置邮件。';
		}
	});

	async function handleReset(e: Event) {
		e.preventDefault();

		if (!isValid) return;

		status = 'submitting';
		errorMsg = '';

		try {
			const res = await fetch('/api/auth/reset', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					token: token,
					password: password
				})
			});

			if (res.ok) {
				status = 'success';
				// 3秒后自动跳转
				setTimeout(() => goto(resolve('/login')), 3000);
			} else {
				const data = await res.json().catch(() => ({}));
				status = 'error';
				errorMsg = data.error || '重置失败，链接可能已过期。请重新尝试。';
			}
		} catch (err) {
			status = 'error';
			errorMsg = '网络请求失败，请检查您的连接。';
			console.error(err);
		} finally {
			if (status !== 'success') {
				status = status === 'error' ? 'error' : 'idle';
			}
		}
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-base-200 p-4">
	<div class="card w-full max-w-md border border-base-200 bg-base-100 shadow-xl">
		<div class="card-body">
			{#if status === 'success'}
				<div class="animate-fade-in py-6 text-center">
					<div class="mb-4 flex justify-center text-success">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-20 w-20"
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
					<h2 class="mb-2 text-2xl font-bold text-base-content">密码重置成功</h2>
					<p class="mb-6 text-base-content/60">您的密码已更新，请使用新密码登录。</p>
					<a href={resolve('/login')} class="btn w-full btn-primary">立即登录</a>
				</div>
			{:else if status === 'error' && !token}
				<div class="py-6 text-center">
					<div class="mb-4 flex justify-center text-error">
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
								d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					</div>
					<h2 class="mb-2 text-xl font-bold">链接无效</h2>
					<p class="mb-6 text-base-content/60">{errorMsg}</p>
					<a href={resolve('/auth/forgot')} class="btn w-full btn-outline">重新发送邮件</a>
				</div>
			{:else}
				<div class="mb-6 text-center">
					<h1 class="text-2xl font-bold">设置新密码</h1>
					<p class="mt-1 text-sm text-base-content/60">请输入您的新密码以完成重置。</p>
				</div>

				{#if status === 'error' && errorMsg}
					<div class="mb-4 alert py-2 text-sm alert-error">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6 shrink-0 stroke-current"
							fill="none"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>{errorMsg}</span>
					</div>
				{/if}

				<form onsubmit={handleReset} class="flex flex-col gap-4">
					<div>
						<label class="label" for="new-pass"> 新密码 </label>
						<input
							type="password"
							id="new-pass"
							placeholder="至少 6 位字符"
							class="input w-full"
							bind:value={password}
							minlength="6"
							required
						/>
					</div>

					<div>
						<label class="label" for="confirm-pass"> 确认密码 </label>
						<input
							type="password"
							id="confirm-pass"
							placeholder="再次输入密码"
							class="input w-full"
							class:input-error={confirmPassword && password !== confirmPassword}
							bind:value={confirmPassword}
							required
						/>
						{#if confirmPassword && password !== confirmPassword}
							<span class="text-error">两次输入的密码不一致</span>
						{/if}
					</div>

					<div class="mt-6">
						<button class="btn btn-primary" disabled={status === 'submitting' || !isValid}>
							{#if status === 'submitting'}
								<span class="loading loading-spinner"></span>
							{/if}
							重置密码
						</button>
					</div>
				</form>

				<div class="divider mt-6 text-sm text-base-content/60">或者</div>
				<div class="text-center">
					<a href={resolve('/login')} class="link text-sm link-hover">取消并返回登录</a>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.animate-fade-in {
		animation: fadeIn 0.5s ease-out;
	}
	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
