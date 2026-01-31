<script lang="ts">
	import { adminStore } from '$lib/state/admin.svelte';

	let isSaved = $state(false);

	function saveSettings() {
		// 模拟保存
		isSaved = true;
		setTimeout(() => (isSaved = false), 2000);
	}
</script>

<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
	<!-- 访问控制 -->
	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">访问控制</h2>

			<div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text font-bold">开放用户注册</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-success"
						bind:checked={adminStore.config.allowRegistration}
					/>
				</label>
				<div class="label-text-alt px-1 opacity-60">
					允许新用户创建账号。关闭后仅管理员可添加用户。
				</div>
			</div>

			<div class="divider"></div>

			<div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text font-bold">允许游客上传</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-primary"
						bind:checked={adminStore.config.allowPublicUpload}
					/>
				</label>
				<div class="label-text-alt px-1 opacity-60">
					未登录用户可以上传图片（不推荐在资源有限时开启）。
				</div>
			</div>

			<div class="divider"></div>

			<div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text font-bold text-error">维护模式</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-error"
						bind:checked={adminStore.config.maintenanceMode}
					/>
				</label>
				<div class="label-text-alt px-1 opacity-60">开启后，除管理员外所有用户无法访问前台。</div>
			</div>
		</div>
	</div>

	<!-- 站点信息 -->
	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">站点配置</h2>

			<div class="form-control">
				<label class="label"><span class="label-text">站点名称</span></label>
				<input
					type="text"
					class="input-bordered input transition-all duration-300"
					bind:value={adminStore.config.siteName}
				/>
			</div>

			<div class="form-control mt-4">
				<label class="label"><span class="label-text">全局公告消息</span></label>
				<textarea
					class="textarea-bordered textarea h-24 transition-all duration-300"
					bind:value={adminStore.config.announcement}
				></textarea>
				<label class="label"
					><span class="label-text-alt">显示在首页顶部的 Banner。留空则不显示。</span></label
				>
			</div>

			<div class="mt-6 card-actions justify-end">
				<button class="btn w-full btn-primary md:w-auto" onclick={saveSettings}>
					{#if isSaved}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M5 13l4 4L19 7"
							/></svg
						>
						已保存
					{:else}
						保存更改
					{/if}
				</button>
			</div>
		</div>
	</div>
</div>
