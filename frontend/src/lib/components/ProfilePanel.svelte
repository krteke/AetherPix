<script lang="ts">
	import { userState } from '$lib/state/user.svelte';

	// 创建本地副本以供编辑
	let formData = $state({ ...userState.info });
	let isSaving = $state(false);

	function handleSave() {
		isSaving = true;
		// 模拟 API 请求
		setTimeout(() => {
			userState.updateInfo(formData);
			isSaving = false;
			alert('个人资料已更新！');
		}, 800);
	}
</script>

<div class="card border border-base-200 bg-base-100 shadow">
	<div class="card-body">
		<h2 class="mb-4 card-title text-xl">编辑个人资料</h2>

		<!-- 头像上传 -->
		<div class="mb-6 flex items-center gap-6">
			<div class="avatar">
				<div class="w-24 rounded-full ring ring-primary ring-offset-2 ring-offset-base-100">
					<img src={formData.avatar} alt="Avatar" />
				</div>
			</div>
			<div>
				<button class="btn mb-2 btn-outline btn-sm">更换头像</button>
				<p class="text-xs text-base-content/60">支持 JPG, PNG. 最大 2MB.</p>
			</div>
		</div>

		<!-- 表单 -->
		<form
			class="flex flex-col gap-4"
			onsubmit={(e) => {
				e.preventDefault();
				handleSave();
			}}
		>
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div class="form-control">
					<label class="label"><span class="label-text">用户名</span></label>
					<input type="text" bind:value={formData.username} class="input-bordered input" />
				</div>
				<div class="form-control">
					<label class="label"><span class="label-text">电子邮箱</span></label>
					<input
						type="email"
						bind:value={formData.email}
						disabled
						class="input-bordered input cursor-not-allowed bg-base-200 text-base-content/60"
					/>
				</div>
			</div>

			<div class="form-control">
				<label class="label"><span class="label-text">个人简介</span></label>
				<textarea
					bind:value={formData.bio}
					class="textarea-bordered textarea h-24"
					placeholder="写点什么..."
				></textarea>
			</div>

			<div class="mt-4 card-actions justify-end">
				<button class="btn min-w-[120px] btn-primary" disabled={isSaving}>
					{#if isSaving}<span class="loading loading-xs loading-spinner"></span>{/if}
					保存修改
				</button>
			</div>
		</form>
	</div>
</div>
