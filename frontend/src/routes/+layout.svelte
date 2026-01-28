<script lang="ts">
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import './layout.css';
	let { children } = $props(); // Svelte 5 获取 slot 的方式

	// 简单的路由模拟，实际项目使用 href 跳转
	// 这里为了演示方便，假设你已经有了路由系统
</script>

<div class="min-h-screen bg-base-100 text-base-content transition-colors duration-300 ease-in-out">
	<!-- Navbar -->
	<div class="navbar border-b border-base-200 bg-base-100 px-4 shadow-sm sm:px-8">
		<div class="flex-1">
			<a href="/" class="btn text-xl font-bold tracking-tighter text-primary btn-ghost">
				⚡️ FlashImg
			</a>
		</div>
		<div class="flex-none gap-2">
			<!-- 菜单链接 -->
			<ul class="menu menu-horizontal hidden px-1 sm:flex">
				<li><a href="/" class="font-medium">图库</a></li>
				<li><a href="/login" class="font-medium">登录</a></li>
			</ul>

			<!-- 主题切换 Controller (DaisyUI) -->
			<ThemeToggle />

			<!-- 移动端头像菜单 (模拟) -->
			<!-- <div class="dropdown dropdown-end sm:hidden">
				<div tabindex="0" role="button" class="btn avatar btn-circle btn-ghost">
					<div class="w-10 rounded-full">
					</div>
				</div>
				<ul
					tabindex="0"
					class="dropdown-content menu z-[1] mt-3 w-52 menu-sm rounded-box bg-base-100 p-2 shadow"
				>
					<li><a href="/">图库</a></li>
					<li><a href="/login">登录</a></li>
				</ul>
			</div> -->
		</div>
	</div>

	<!-- 页面内容插槽 -->
	<main class="container mx-auto p-4 sm:p-8">
		{@render children()}
	</main>
</div>
