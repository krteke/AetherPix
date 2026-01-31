<script lang="ts">
	let currentPass = $state('');
	let newPass = $state('');
	let confirmPass = $state('');

	function updatePassword(e: Event) {
		e.preventDefault();
		if (newPass !== confirmPass) {
			alert('两次输入的新密码不一致');
			return;
		}
		alert('密码修改请求已发送');
		// Reset
		currentPass = '';
		newPass = '';
		confirmPass = '';
	}
</script>

<div class="card border border-base-200 bg-base-100 shadow transition-all duration-300">
	<div class="card-body">
		<h2 class="mb-4 card-title text-xl text-error">修改密码</h2>
		<form onsubmit={updatePassword} class="flex max-w-md flex-col gap-4">
			<div class="form-control">
				<label class="label"><span class="label-text">当前密码</span></label>
				<input
					type="password"
					bind:value={currentPass}
					required
					class="input-bordered input transition-all duration-300"
				/>
			</div>
			<div class="divider my-0"></div>
			<div class="form-control">
				<label class="label"><span class="label-text">新密码</span></label>
				<input
					type="password"
					bind:value={newPass}
					required
					minlength="8"
					class="input-bordered input transition-all duration-300"
				/>
				<label class="label"><span class="label-text-alt opacity-60">最少 8 个字符</span></label>
			</div>
			<div class="form-control">
				<label class="label"><span class="label-text">确认新密码</span></label>
				<input
					type="password"
					bind:value={confirmPass}
					required
					class="input-bordered input transition-all duration-300"
				/>
			</div>
			<div class="mt-4">
				<button class="btn text-white btn-error">更新密码</button>
			</div>
		</form>
	</div>
</div>
