<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';
	import { settingsStore } from '$lib/state/settings.svelte';

	let isSaving = $state(false);
	let allowRegister = $state(settingsStore.allowRegister);
	let allowEveryoneUpload = $state(settingsStore.allowEveryoneUpload);
	let siteName = $state(settingsStore.siteName);
	let uploadMaxSize = $state(settingsStore.uploadMaxSize);
	let localBaseUrl = $state(settingsStore.localBaseUrl);
	let r2BaseUrl = $state(settingsStore.r2BaseUrl);

	async function saveSettings() {
		isSaving = true;
		try {
			await settingsStore.update({
				allow_registration: allowRegister,
				allow_everyone_upload: allowEveryoneUpload,
				site_name: siteName,
				upload_max_size: uploadMaxSize,
				local_base_url: localBaseUrl,
				r2_base_url: r2BaseUrl
			});

			msg.alert('保存成功', '保存成功', 'success');
		} catch (error) {
			console.error('Failed to update settings:', error);
		} finally {
			isSaving = false;
		}
	}
</script>

<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">访问控制</h2>

			<div class="w-full">
				<label class="label cursor-pointer">
					<span class="font-bold">开放用户注册</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-success"
						bind:checked={allowRegister}
					/>
				</label>
				<div class="px-1 opacity-60">允许新用户创建账号。关闭后仅管理员可添加用户。</div>
			</div>

			<div class="divider"></div>

			<div class="w-full">
				<label class="label cursor-pointer">
					<span class="font-bold">允许游客上传</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-primary"
						bind:checked={allowEveryoneUpload}
					/>
				</label>
				<div class="px-1 opacity-60">未登录用户可以上传图片。</div>
			</div>

			<!-- <div class="divider"></div> -->

			<!-- <div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text font-bold text-error">维护模式</span>
					<input
						type="checkbox"
						class="toggle transition-all duration-300 toggle-error"
						bind:checked={adminStore.config.maintenanceMode}
					/>
				</label>
				<div class="label-text-alt px-1 opacity-60">开启后，除管理员外所有用户无法访问前台。</div>
			</div> -->
		</div>
	</div>

	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">站点配置</h2>

			<!-- <div class="form-control">
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
			</div> -->
			<div>
				<label class="label" for="site_name">站点名称</label>
				<input
					id="site_name"
					type="text"
					class="input-bordered input transition-all duration-300"
					bind:value={siteName}
				/>
			</div>
			<div>
				<label class="label" for="max_size">最大上传尺寸(MB)</label>
				<input
					id="max_size"
					type="text"
					class="input-bordered input transition-all duration-300"
					bind:value={uploadMaxSize}
				/>
			</div>
			<div>
				<label class="label" for="local_base_url">Local Base Url</label>
				<input
					id="local_base_url"
					type="text"
					class="input-bordered input w-full transition-all duration-300"
					placeholder="自定义图片url，默认为:前端url/api/view"
					bind:value={localBaseUrl}
				/>
			</div>
			<div>
				<label class="label" for="r2_base_url">R2 Base Url</label>
				<input
					id="r2_base_url"
					type="text"
					class="input-bordered input w-full transition-all duration-300"
					bind:value={r2BaseUrl}
				/>
			</div>

			<div class="mt-6 card-actions justify-end">
				<button
					class="btn w-full transition-all duration-300 btn-primary md:w-auto"
					disabled={isSaving}
					onclick={saveSettings}
				>
					{#if isSaving}
						<span class="loading loading-spinner"></span>
					{/if}
					保存更改
				</button>
			</div>
		</div>
	</div>
</div>
