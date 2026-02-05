<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';

	const localStorage = $state([
		{ name: 'access key id', key: 'aws_access_key_id', edit: false, value: '' },
		{ name: 'secret access key', key: 'aws_secret_access_key', edit: false, value: '' },
		{ name: 'region', key: 'aws_region', edit: false, value: '' },
		{ name: 'endpoint', key: 'aws_endpoint_url', edit: false, value: '' },
		{ name: 'origin bucket', key: 'origin_bucket_name', edit: false, value: '' },
		{ name: 'preview bucket', key: 'preview_bucket_name', edit: false, value: '' },
		{ name: 'webp bucket', key: 'webp_bucket_name', edit: false, value: '' }
	]);

	const r2Storage = $state([
		{ name: 'access key id', key: 'r2_access_key_id', edit: false, value: '' },
		{ name: 'secret access key', key: 'r2_secret_access_key', edit: false, value: '' },
		{ name: 'bucket name', key: 'r2_bucket_name', edit: false, value: '' },
		{ name: 'region', key: 'r2_region', edit: false, value: '' },
		{ name: 'endpoint', key: 'r2_endpoint_url', edit: false, value: '' }
	]);

	async function save(name: string, value: string) {
		try {
			const res = await fetch('/api/setting/update', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ name, value })
			});

			if (!res.ok) {
				msg.alert('保存失败\n ' + (await res.text()), '保存失败', 'error');
			}

			msg.alert('保存成功', '保存成功', 'success');
		} catch (error) {
			console.error(error);
			msg.alert('保存失败', '保存失败', 'error');
		}
	}
</script>

<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">本地储存(Garage)</h2>
			{#each localStorage as storage (storage.name)}
				<div class="flex w-full justify-between">
					<label class="label" for={storage.name}>{storage.name}</label>
					<input
						id={storage.name}
						type="text"
						class="input-bordered input w-fit transition-all duration-300"
						disabled={!storage.edit}
						bind:value={storage.value}
					/>
					<div class="flex justify-end gap-2">
						<button class="btn btn-sm btn-primary" onclick={() => (storage.edit = true)}
							>编辑</button
						>
						<button
							class="btn btn-sm btn-primary"
							onclick={async () => {
								await save(storage.key, storage.value);
								storage.edit = false;
							}}>保存</button
						>
					</div>
				</div>
			{/each}
		</div>
	</div>

	<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
		<div class="card-body">
			<h2 class="mb-4 card-title text-lg">Cloudflare R2</h2>
			{#each r2Storage as storage (storage.name)}
				<div class="flex w-full justify-between">
					<label class="label" for={storage.name}>{storage.name}</label>
					<input
						id={storage.name}
						type="text"
						class="input-bordered input w-fit transition-all duration-300"
						disabled={!storage.edit}
						bind:value={storage.value}
					/>
					<div class="flex justify-end gap-2">
						<button class="btn btn-sm btn-primary" onclick={() => (storage.edit = true)}
							>编辑</button
						>
						<button
							class="btn btn-sm btn-primary"
							onclick={async () => {
								await save(storage.key, storage.value);
								storage.edit = false;
							}}>保存</button
						>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
