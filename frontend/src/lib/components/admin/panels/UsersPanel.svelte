<script lang="ts">
	import { adminStore } from '$lib/state/admin.svelte';

	let searchTerm = $state('');
	let isModalOpen = $state(false);

	// 新用户表单
	let newUser = $state({ name: '', email: '' });

	// 过滤逻辑
	let filteredUsers = $derived(
		adminStore.users.filter(
			(u) =>
				u.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
				u.email.toLowerCase().includes(searchTerm.toLowerCase())
		)
	);

	function handleAddUser() {
		if (newUser.name && newUser.email) {
			adminStore.addUser(newUser.name, newUser.email);
			newUser = { name: '', email: '' };
			isModalOpen = false;
		}
	}

	function confirmDelete(id: number) {
		if (confirm('确定要永久删除该用户及其所有图片吗？此操作不可逆。')) {
			adminStore.deleteUser(id);
		}
	}
</script>

<div class="card h-full border border-base-200 bg-base-100 shadow">
	<div class="card-body">
		<div class="mb-4 flex items-center justify-between">
			<h2 class="card-title">用户列表</h2>
			<div class="flex gap-2">
				<input
					type="text"
					placeholder="搜索用户..."
					class="input-bordered input input-sm w-full max-w-xs"
					bind:value={searchTerm}
				/>
				<button class="btn btn-sm btn-primary" onclick={() => (isModalOpen = true)}>
					+ 添加用户
				</button>
			</div>
		</div>

		<div class="overflow-x-auto">
			<table class="table w-full table-zebra">
				<thead>
					<tr>
						<th>用户</th>
						<th>角色</th>
						<th>状态</th>
						<th>存储</th>
						<th>加入时间</th>
						<th class="text-right">操作</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredUsers as user (user.id)}
						<tr>
							<td>
								<div class="flex items-center gap-3">
									<div class="placeholder avatar">
										<div class="w-8 rounded-full bg-neutral text-neutral-content">
											<span class="text-xs">{user.name[0]}</span>
										</div>
									</div>
									<div>
										<div class="font-bold">{user.name}</div>
										<div class="text-xs opacity-50">{user.email}</div>
									</div>
								</div>
							</td>
							<td>
								{#if user.role === 'admin'}
									<span class="badge badge-outline badge-sm badge-primary">管理员</span>
								{:else}
									<span class="badge badge-ghost badge-sm">普通用户</span>
								{/if}
							</td>
							<td>
								{#if user.status === 'active'}
									<span class="badge gap-1 badge-xs badge-success">正常</span>
								{:else}
									<span class="badge gap-1 badge-xs badge-error">封禁</span>
								{/if}
							</td>
							<td class="font-mono text-xs">{user.storageUsed}</td>
							<td class="text-xs opacity-70">{user.joinedAt}</td>
							<td class="text-right">
								{#if user.role !== 'admin'}
									<button
										class="btn btn-xs {user.status === 'active' ? 'btn-warning' : 'btn-success'}"
										onclick={() => adminStore.toggleUserStatus(user.id)}
									>
										{user.status === 'active' ? '封禁' : '解封'}
									</button>
									<button
										class="btn btn-outline btn-xs btn-error"
										onclick={() => confirmDelete(user.id)}
									>
										删除
									</button>
								{:else}
									<span class="pr-2 text-xs italic opacity-30">不可操作</span>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>

<!-- 添加用户 Modal -->
{#if isModalOpen}
	<div class="modal-open modal">
		<div class="modal-box">
			<h3 class="text-lg font-bold">添加新用户</h3>
			<div class="flex flex-col gap-4 py-4">
				<input
					type="text"
					placeholder="用户名"
					class="input-bordered input w-full"
					bind:value={newUser.name}
				/>
				<input
					type="email"
					placeholder="邮箱地址"
					class="input-bordered input w-full"
					bind:value={newUser.email}
				/>
			</div>
			<div class="modal-action">
				<button class="btn" onclick={() => (isModalOpen = false)}>取消</button>
				<button class="btn btn-primary" onclick={handleAddUser}>确认添加</button>
			</div>
		</div>
	</div>
{/if}
