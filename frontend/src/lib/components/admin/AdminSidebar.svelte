<script lang="ts">
	let { activeTab = $bindable() } = $props<{ activeTab: string }>();

	const items = [
		{
			id: 'overview',
			label: '仪表盘概览',
			icon: 'M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z'
		},
		{
			id: 'users',
			label: '用户管理',
			icon: 'M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z'
		},
		{
			id: 'system',
			label: '系统设置',
			icon: 'M10.34 15.84c-.688-.06-1.386-.09-2.09-.09H7.5a4.5 4.5 0 110-9h.75c.704 0 1.402-.03 2.09-.09 1.428 1.295 1.348 3.692-.128 4.793a3.003 3.003 0 000 4.397c1.476 1.1 1.556 3.498.128 4.793z'
		}
	];
</script>

<div class="flex min-h-[calc(100vh-4rem)] w-full flex-col gap-2 bg-base-200 p-4 md:w-64">
	<div class="mb-4 px-4 py-2">
		<h2 class="flex items-center gap-2 text-xl font-black tracking-tighter">
			<span class="badge badge-sm badge-error">ADMIN</span>
			控制台
		</h2>
	</div>
	<ul class="menu w-full gap-2 rounded-box">
		{#each items as item, i (i)}
			<li>
				<button
					class={activeTab === item.id ? 'active bg-neutral font-bold text-neutral-content' : ''}
					onclick={() => (activeTab = item.id)}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="h-5 w-5"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d={item.icon} />
					</svg>
					{item.label}
				</button>
			</li>
		{/each}
	</ul>
</div>
