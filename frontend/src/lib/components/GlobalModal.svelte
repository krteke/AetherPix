<script lang="ts">
	import { msg } from '$lib/state/msg.svelte';
	import { fade, scale } from 'svelte/transition';

	// 根据类型获取对应的颜色和图标
	let typeConfig = $derived.by(() => {
		switch (msg.type) {
			case 'success':
				return { color: 'text-success', btn: 'btn-success', icon: 'check-circle' };
			case 'warning':
				return { color: 'text-warning', btn: 'btn-warning', icon: 'exclamation-triangle' };
			case 'error':
				return { color: 'text-error', btn: 'btn-error', icon: 'x-circle' };
			default:
				return { color: 'text-info', btn: 'btn-info', icon: 'information-circle' };
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (msg.isOpen && e.key === 'Escape') {
			msg.handleAction(false);
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if msg.isOpen}
	<div
		class="modal-open modal z-9999 modal-bottom bg-base-300/60 backdrop-blur-sm sm:modal-middle"
		transition:fade={{ duration: 150 }}
		role="dialog"
	>
		<div
			class="relative modal-box border border-base-200 shadow-2xl"
			transition:scale={{ start: 0.95, duration: 150 }}
		>
			<button
				class="btn absolute top-2 right-2 btn-circle btn-ghost btn-sm"
				onclick={() => msg.handleAction(false)}>✕</button
			>

			<div
				class="flex flex-col items-center gap-4 text-center sm:flex-row sm:items-start sm:text-left"
			>
				<div class={typeConfig.color}>
					{#if msg.type === 'success'}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-10 w-10"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					{:else if msg.type === 'warning'}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-10 w-10"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
							/></svg
						>
					{:else if msg.type === 'error'}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-10 w-10"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-10 w-10"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					{/if}
				</div>

				<div class="flex-1">
					<h3 class="text-lg font-bold">{msg.title}</h3>
					<p class="py-2 whitespace-pre-wrap text-base-content/80">{msg.content}</p>
				</div>
			</div>

			<div class="modal-action mt-6 justify-center sm:justify-end">
				{#if msg.showCancel}
					<button class="btn" onclick={() => msg.handleAction(false)}>
						{msg.cancelText}
					</button>
				{/if}

				<button
					class="btn {typeConfig.btn} min-w-20 text-white"
					onclick={() => msg.handleAction(true)}
				>
					{msg.confirmText}
				</button>
			</div>
		</div>

		<button class="modal-backdrop" onclick={() => msg.handleAction(false)} aria-label="Close modal"
		></button>
	</div>
{/if}
