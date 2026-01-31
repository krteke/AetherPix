<script lang="ts">
	import { resolve } from '$app/paths';

	let email = $state('');
	let isSent = $state(false);
	let isLoading = $state(false);

	async function handleForgot(e: Event) {
		e.preventDefault();
		isLoading = true;

		try {
			await fetch('/api/auth/forgot', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});
			isSent = true;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-base-200 transition-all duration-300">
	<div class="card w-full max-w-sm bg-base-100 shadow-xl transition-all duration-300">
		<div class="card-body">
			{#if !isSent}
				<h2 class="mb-2 card-title text-2xl font-bold">重置密码</h2>
				<p class="mb-4 text-sm text-base-content/60 transition-all duration-300">
					输入您的注册邮箱，我们将发送重置链接给您。
				</p>

				<form onsubmit={handleForgot}>
					<div class="form-control mb-4">
						<input
							type="email"
							placeholder="name@example.com"
							class="input-bordered input transition-all duration-300"
							bind:value={email}
							required
						/>
					</div>
					<button class="btn w-full btn-primary" disabled={isLoading}>
						{#if isLoading}<span class="loading loading-spinner"></span>{/if}
						发送重置邮件
					</button>
				</form>
				<div class="mt-4 text-center">
					<a href={resolve('/login')} class="link text-sm link-hover">想起密码了？返回登录</a>
				</div>
			{:else}
				<div class="text-center">
					<div class="mb-4 flex justify-center text-info">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-12 w-12"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="text-lg font-bold">邮件已发送</h3>
					<p class="mt-2 text-sm opacity-70">如果该邮箱存在，您将很快收到重置指南。</p>
					<a href={resolve('/login')} class="btn mt-6 btn-ghost">返回登录</a>
				</div>
			{/if}
		</div>
	</div>
</div>
