<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';
	import type { ApiKeyResponse } from '$lib/types/type';

	const { name, email, apiToken }: { name: string; email: string; apiToken: string } = $props();
	let type = $state('password');
	let apiKey = $state(() => apiToken);
	let isResetting = $state(false);

	const resetToken = async () => {
		if (isResetting) return;
		isResetting = true;
		try {
			const response = await fetch('/api/auth/reset/api-key', {
				method: 'POST'
			});

			if (!response.ok) {
				msg.alert(await response.text(), '重置失败', 'error');
			}

			const data: ApiKeyResponse = await response.json();
			apiKey = () => data.key;
			msg.alert('API Token 已重置', '重置成功', 'success');
		} catch (error) {
			console.error(error);
			msg.alert('API Token 重置失败', '重置失败', 'error');
		} finally {
			isResetting = false;
		}
	};

	const copyApiToken = () => {
		navigator.clipboard.writeText(apiToken);
		msg.alert('API Token 已复制到剪贴板', '复制成功', 'success');
	};
</script>

<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
	<div class="card-body">
		<h2 class="mb-4 card-title text-xl">个人资料</h2>

		<!-- <div class="mb-6 flex items-center gap-6">
			<div class="avatar">
				<div class="w-24 rounded-full ring ring-primary ring-offset-2 ring-offset-base-100">
					<img src={formData.avatar} alt="Avatar" />
				</div>
			</div>
			<div class=" transition-all duration-300">
				<button class="btn mb-2 btn-outline btn-sm">更换头像</button>
				<p class="text-xs text-base-content/60">支持 JPG, PNG. 最大 2MB.</p>
			</div>
		</div> -->

		<form
			class="flex flex-col gap-4"
			onsubmit={(e) => {
				e.preventDefault();
				// handleSave();
			}}
		>
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label class="label"><span class="label-text">用户名</span></label>
					<input
						type="text"
						value={name}
						disabled
						class="input-bordered input cursor-not-allowed bg-base-200 text-base-content/60 transition-all duration-300"
					/>
				</div>
				<div>
					<label class="label"><span class="label-text">电子邮箱</span></label>
					<input
						type="email"
						value={email}
						disabled
						class="input-bordered input cursor-not-allowed bg-base-200 text-base-content/60 transition-all duration-300"
					/>
				</div>
			</div>
			<div>
				<label class="label"><span class="label-text">API Token</span></label>
				<input
					{type}
					value={apiKey()}
					disabled
					class="input-bordered input cursor-not-allowed bg-base-200 text-base-content/60 transition-all duration-300"
				/>
				<button
					class="btn transition-all duration-300 btn-sm btn-primary"
					onclick={() => (type = type === 'text' ? 'password' : 'text')}
					>{type === 'text' ? '隐藏' : '显示'}</button
				>
				<button class="btn transition-all duration-300 btn-sm btn-primary" onclick={copyApiToken}
					>复制</button
				>
				<button
					class="btn transition-all duration-300 btn-outline btn-sm btn-primary"
					disabled={isResetting}
					onclick={resetToken}
				>
					{#if isResetting}<span
							class="loading loading-xs loading-spinner transition-all duration-300"
						></span>{/if}
					重置
				</button>
			</div>

			<!-- <div class="mt-4 card-actions justify-end">
				<button class="btn min-w-[120px] btn-primary" disabled={isSaving}>
					{#if isSaving}<span class="loading loading-xs loading-spinner"></span>{/if}
					保存修改
				</button>
			</div> -->
		</form>
	</div>
</div>
