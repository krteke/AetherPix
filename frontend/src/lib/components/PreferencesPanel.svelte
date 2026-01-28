<script lang="ts">
	import { userState } from '$lib/state/user.svelte';
</script>

<div class="card border border-base-200 bg-base-100 shadow">
	<div class="card-body">
		<h2 class="mb-6 card-title text-xl">通用偏好</h2>

		<div class="flex flex-col gap-6">
			<!-- 默认链接格式 -->
			<div class="form-control">
				<label class="label cursor-pointer justify-start gap-4">
					<span class="label-text w-32 font-bold">默认链接格式</span>
					<select
						class="select-bordered select w-full max-w-xs select-sm"
						bind:value={userState.preferences.defaultLinkType}
					>
						<option value="Markdown">Markdown</option>
						<option value="HTML">HTML</option>
						<option value="URL">Raw URL</option>
						<option value="BBCode">BBCode</option>
					</select>
				</label>
				<p class="mt-1 ml-36 text-xs opacity-60">上传完成后默认复制的链接类型。</p>
			</div>

			<div class="divider my-0"></div>

			<!-- 开关选项 -->
			<div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text">
						<div class="font-bold">上传自动压缩</div>
						<div class="text-xs opacity-60">在不明显降低画质的情况下减少体积</div>
					</span>
					<input
						type="checkbox"
						class="toggle toggle-primary"
						bind:checked={userState.preferences.autoCompress}
					/>
				</label>
			</div>

			<div class="form-control w-full">
				<label class="label cursor-pointer">
					<span class="label-text">
						<div class="font-bold">默认私有上传</div>
						<div class="text-xs opacity-60">上传的图片默认不出现在公开广场</div>
					</span>
					<input
						type="checkbox"
						class="toggle toggle-secondary"
						bind:checked={userState.preferences.privateUpload}
					/>
				</label>
			</div>
		</div>
	</div>
</div>
