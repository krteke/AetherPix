<script lang="ts">
	import { resolve } from '$app/paths';
	import GlobalModal from '$lib/components/GlobalModal.svelte';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import { onMount } from 'svelte';
	import './layout.css';
	import { auth } from '$lib/state/auth.svelte';

	let { children } = $props();

	const logout = () => {
		auth.logout();
	};

	onMount(() => {
		auth.init();
	});
</script>

<div class="min-h-screen bg-base-100 text-base-content transition-colors duration-300 ease-in-out">
	<div class="navbar border-b border-base-200 bg-base-100 px-4 shadow-sm sm:px-8">
		<div class="flex-1">
			<a
				href={resolve('/')}
				class="btn font-mono text-xl font-bold tracking-tighter text-primary btn-ghost"
			>
				⚡️ AetherPix
			</a>
		</div>
		<div class="flex-none gap-2">
			<ul class="menu menu-horizontal hidden px-1 font-medium sm:flex">
				{#if auth.isAdmin}
					<li><a href={resolve('/admin')}>控制面板</a></li>
				{/if}
				{#if !auth.isLoggedIn}
					<!-- <ul class="menu menu-horizontal hidden px-1 sm:flex"> -->
					<!-- <li><a href={resolve('/albums')} class="font-medium">图库</a></li> -->
					<li><a href={resolve('/login')}>登录</a></li>
				{:else}
					<li><button onclick={logout}>退出登录</button></li>
				{/if}
			</ul>
			<ThemeToggle />
		</div>
	</div>

	<main class="container mx-auto p-4 sm:p-8">
		{@render children()}
	</main>
</div>
<GlobalModal />
