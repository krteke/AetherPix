<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { auth } from '$lib/state/auth.svelte';
	import { msg } from '$lib/state/msg.svelte';
	import type { UserResponse } from '$lib/types/type';

	let isRegister = $state(false);
	let isLoading = $state(false);
	let isSending = $state(false);
	let username = $state('');
	let password = $state('');
	let email = $state('');
	let confirmPassword = $state('');
	let sendReg = $state(false);

	const userNameReg = /^[a-zA-Z0-9_-]+$/;

	async function register() {
		try {
			const res = await fetch('/api/auth/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username, password, email })
			});

			if (!res.ok) {
				const text = await res.text();
				msg.alert(`状态码: ${res.status}\nError: ${text}`, '注册失败', 'error');
				isLoading = false;
				return;
			}

			await msg.alert(
				`验证邮件已发送至${email}。\n请检查您的收件箱（包括垃圾邮件文件夹）并点击链接激活账号。`,
				'注册成功'
			);
			sendReg = true;
		} catch (error) {
			console.error(error);
			msg.alert(error as string, '注册失败', 'error');
			isLoading = false;
			return;
		} finally {
			isLoading = false;
		}
	}

	async function login() {
		try {
			const res = await fetch('/api/auth/login', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username, password })
			});

			if (!res.ok) {
				const text = await res.text();
				msg.alert(`状态码: ${res.status}\nError: ${text}`, '登录失败', 'error');
				isLoading = false;
				return;
			}

			const loginResponse: UserResponse = await res.json();
			auth.login(loginResponse);
			isLoading = false;
			await msg.alert('登录成功');
			goto(resolve('/'));
		} catch (error) {
			console.error(error);
			msg.alert(error as string, '登录失败', 'error');
			isLoading = false;
			return;
		}
	}

	const resendEmail = async () => {
		isSending = true;
		try {
			const res = await fetch('/api/auth/resend-verification-mail', {
				method: 'POST',
				body: JSON.stringify({ email })
			});

			if (!res.ok) {
				const text = await res.text();
				msg.alert(`状态码: ${res.status}\nError: ${text}`, '发送失败', 'error');
				isSending = false;
				return;
			}
			isSending = false;
			await msg.alert('发送成功，请查看邮箱');
		} catch (e) {
			console.error(e);
			msg.alert(e as string, '登录失败', 'error');
			isLoading = false;
			return;
		} finally {
			isSending = false;
		}
	};

	async function handleSubmit(e: Event) {
		e.preventDefault();
		isLoading = true;

		if (isRegister && password !== confirmPassword) {
			msg.alert('两次输入的密码不一致！', '请重新输入密码', 'warning');
			isLoading = false;
			return;
		}

		if (isRegister) {
			if (userNameReg.test(username) && username.trim().length >= 2) {
				await register();
			} else if (!userNameReg.test(username)) {
				msg.alert('用户名只能包含字母、数字、下划线和短横线', '用户名格式不正确！', 'warning');
			} else if (password.length < 6) {
				msg.alert('密码至少6位', '密码长度太短', 'warning');
			} else {
				msg.alert('用户名长度至少为2个字符', '用户名长度不正确！', 'warning');
			}
		} else {
			await login();
		}
	}
</script>

<div class="flex min-h-[80vh] flex-col items-center justify-center">
	<div class="card w-full max-w-sm border border-base-200 bg-base-100 shadow-2xl">
		<div class="card-body">
			<h2 class="mb-4 card-title justify-center text-3xl font-bold">
				{isRegister ? '创建账户' : '欢迎回来'}
			</h2>

			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="form-control">
					<label class="label" for="username">
						<span class="label-text">用户名</span>
					</label>
					<input
						bind:value={username}
						type="text"
						id="username"
						placeholder="username"
						class="input-bordered input w-full"
						minlength="2"
						required
					/>
				</div>
				{#if isRegister}
					<div class="form-control">
						<label class="label" for="email">
							<span class="label-text">邮箱</span>
						</label>
						<input
							bind:value={email}
							type="text"
							id="email"
							placeholder="example@example.com"
							class="input-bordered input w-full"
							required
						/>
					</div>
				{/if}
				<div class="form-control">
					<label class="label" for="password">
						<span class="label-text">密码</span>
					</label>
					<input
						bind:value={password}
						type="password"
						id="password"
						placeholder="••••••"
						class="input-bordered input w-full"
						minlength="6"
						required
					/>
				</div>
				{#if isRegister}
					<div class="form-control animate-fade-in-down">
						<label class="label" for="confirm-pass">
							<span class="label-text">确认密码</span>
						</label>
						<input
							bind:value={confirmPassword}
							type="password"
							id="confirm-pass"
							placeholder="••••••"
							class="input-bordered input w-full"
							class:input-error={confirmPassword && password != confirmPassword}
							required
						/>
					</div>
				{/if}
				{#if isRegister && confirmPassword && password !== confirmPassword}
					<span class="text-error">两次输入的密码不一致</span>
				{/if}

				{#if !isRegister}
					<a href={resolve('/auth/forgot')} class="ml-1 link font-bold link-primary"> 忘记密码 </a>
				{/if}
				{#if isRegister && sendReg}
					<button class="btn mb-2 w-full btn-warning" disabled={isSending} onclick={resendEmail}>
						{#if isSending}
							<span class="loading loading-spinner"></span>
						{/if}
						重新发送验证邮件
					</button>
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
