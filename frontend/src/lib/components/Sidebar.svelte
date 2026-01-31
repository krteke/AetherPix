<script lang="ts">
	let { activeTab = $bindable() } = $props<{ activeTab: string }>();

	const menuItems = [
		{
			id: 'profile',
			label: '个人资料',
			icon: 'M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z'
		},
		{
			id: 'security',
			label: '账号安全',
			icon: 'M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z'
		},
		{
			id: 'stats',
			label: '数据统计',
			icon: 'M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z'
		},
		{
			id: 'prefs',
			label: '偏好设置',
			icon: 'M10.34 15.84c-.688-.06-1.386-.09-2.09-.09H7.5a4.5 4.5 0 110-9h.75c.704 0 1.402-.03 2.09-.09 1.428 1.295 1.348 3.692-.128 4.793a3.003 3.003 0 000 4.397c1.476 1.1 1.556 3.498.128 4.793z'
		}
	];
</script>

<ul
	class="menu h-fit w-full gap-1 rounded-box border border-base-200 bg-base-100 p-2 transition-all duration-300 md:w-64"
>
	<li class="mb-2 menu-title px-4 transition-all duration-300">设置中心</li>
	{#each menuItems as item, i (i)}
		<li>
			<button
				class={activeTab === item.id ? 'active font-bold' : ''}
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
