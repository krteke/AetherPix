<script lang="ts">
	// 使用 Svelte 5 Runes
	let isRegister = $state(false);
	let isLoading = $state(false);

	// 模拟提交
	function handleSubmit(e: Event) {
		e.preventDefault();
		isLoading = true;
		setTimeout(() => {
			isLoading = false;
			alert(isRegister ? '注册成功！' : '登录成功！');
			// 这里可以添加 goto('/') 跳转
		}, 1500);
	}
</script>

<div class="flex min-h-[80vh] flex-col items-center justify-center">
	<div class="card w-full max-w-sm border border-base-200 bg-base-100 shadow-2xl">
		<div class="card-body">
			<!-- 标题 & 切换 -->
			<h2 class="mb-4 card-title justify-center text-3xl font-bold">
				{isRegister ? '创建账户' : '欢迎回来'}
			</h2>

			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<!-- 邮箱输入 -->
				<div class="form-control">
					<label class="label" for="email">
						<span class="label-text">电子邮箱</span>
					</label>
					<input
						type="email"
						id="email"
						placeholder="email@example.com"
						class="input-bordered input w-full"
						required
					/>
				</div>

				<!-- 密码输入 -->
				<div class="form-control">
					<label class="label" for="password">
						<span class="label-text">密码</span>
					</label>
					<input
						type="password"
						id="password"
						placeholder="••••••••"
						class="input-bordered input w-full"
						required
					/>
					{#if !isRegister}
						<label class="label" for="password">
							<a href="#" class="label-text-alt link link-hover">忘记密码?</a>
						</label>
					{/if}
				</div>

				<!-- 注册时的确认密码 -->
				{#if isRegister}
					<div class="form-control animate-fade-in-down">
						<label class="label" for="confirm-pass">
							<span class="label-text">确认密码</span>
						</label>
						<input
							type="password"
							id="confirm-pass"
							placeholder="••••••••"
							class="input-bordered input w-full"
							required
						/>
					</div>
				{/if}

				<!-- 提交按钮 -->
				<div class="form-control mt-6">
					<button class="btn w-full btn-primary" disabled={isLoading}>
						{#if isLoading}
							<span class="loading loading-spinner"></span>
						{/if}
						{isRegister ? '立即注册' : '登录'}
					</button>
				</div>
			</form>

			<!-- 底部切换引导 -->
			<div class="divider text-sm text-base-content/60">或</div>
			<div class="text-center text-sm">
				{isRegister ? '已有账号?' : '还没有账号?'}
				<button class="ml-1 link font-bold link-primary" onclick={() => (isRegister = !isRegister)}>
					{isRegister ? '去登录' : '去注册'}
				</button>
			</div>
		</div>
	</div>
</div>

<style>
	/* 简单的淡入动画 */
	.animate-fade-in-down {
		animation: fadeInDown 0.3s ease-out;
	}
	@keyframes fadeInDown {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
