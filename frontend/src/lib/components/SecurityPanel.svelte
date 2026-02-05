<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';

	let currentPass = $state('');
	let newPass = $state('');
	let confirmPass = $state('');
	let isLoading = $state(false);

	async function updatePassword(e: Event) {
		e.preventDefault();
		if (isLoading) return;
		isLoading = true;

		try {
			const res = await fetch('/api/auth/reset/password', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					password: currentPass,
					newPassword: newPass
				})
			});

			if (!res.ok) {
				msg.alert(await res.text(), '修改密码失败', 'error');
			}

			msg.alert('密码修改成功', '修改密码成功', 'success');
		} catch (error) {
			console.error(error);
			msg.alert('修改密码失败', '修改密码失败', 'error');
		} finally {
			currentPass = '';
			newPass = '';
			confirmPass = '';
			isLoading = false;
		}
	}
</script>

<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
	<div class="card-body">
		<h2 class="mb-4 card-title text-xl text-error">修改密码</h2>
		<form onsubmit={updatePassword} class="flex max-w-md flex-col gap-4">
			<div>
				<label class="label" for="current_pass">当前密码</label>
				<input
					id="current_pass"
					type="password"
					bind:value={currentPass}
					required
					class="input-bordered input transition-all duration-300"
				/>
			</div>
			<div class="divider my-0"></div>
			<div>
				<label class="label" for="new_pass">新密码</label>
				<input
					id="new_pass"
					type="password"
					bind:value={newPass}
					required
					minlength="8"
					maxlength="128"
					class="input-bordered input transition-all duration-300"
				/>
				<label class="label"><span class="opacity-60">最少 8 个字符</span></label>
			</div>
			<div>
				<label class="label" for="confirm_pass"><span>确认新密码</span></label>
				<input
					id="confirm_pass"
					type="password"
					bind:value={confirmPass}
					required
					maxlength="128"
					class:input-error={confirmPass && newPass !== confirmPass}
					class="input-bordered input transition-all duration-300"
				/>
			</div>
			{#if confirmPass && newPass !== confirmPass}
				<span class="text-error">两次输入的密码不一致</span>
			{/if}
			<div class="mt-4">
				<button class="btn text-white transition-all duration-300 btn-error" disabled={isLoading}>
					{#if isLoading}<span
							class="loading loading-xs loading-spinner transition-all duration-300"
						></span>{/if}
					更新密码</button
				>
			</div>
		</form>
	</div>
</div>
