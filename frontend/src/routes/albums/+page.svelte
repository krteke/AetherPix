<script lang="ts">
	// 模拟相册数据
	let albums = $state([
		{ id: 1, name: '默认相册', count: 42, cover: 'https://picsum.photos/id/10/200/200' },
		{ id: 2, name: '壁纸收藏', count: 15, cover: 'https://picsum.photos/id/28/200/200' },
		{ id: 3, name: '开发截图', count: 120, cover: 'https://picsum.photos/id/3/200/200' }
	]);

	let newAlbumName = $state('');
	let modal: HTMLDialogElement; // 绑定 dialog 元素

	function createAlbum() {
		if (!newAlbumName) return;
		albums = [
			...albums,
			{
				id: Date.now(),
				name: newAlbumName,
				count: 0,
				cover: 'https://via.placeholder.com/200?text=Empty' // 默认封面
			}
		];
		newAlbumName = '';
		modal.close();
	}
</script>

<div class="mb-8 flex items-center justify-between">
	<h1 class="text-3xl font-bold">我的相册</h1>
	<button class="btn btn-primary" onclick={() => modal.showModal()}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="mr-1 h-5 w-5"
		>
			<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
		</svg>
		新建相册
	</button>
</div>

<div class="grid grid-cols-2 gap-6 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
	{#each albums as album, i (i)}
		<a
			href="/albums/{album.id}"
			class="group card border border-base-200 bg-base-100 shadow-md transition-all duration-300 hover:-translate-y-1 hover:shadow-xl"
		>
			<figure class="px-4 pt-4">
				<div class="relative aspect-square w-full overflow-hidden rounded-xl bg-base-200">
					<!-- 模拟相册层叠效果 -->
					<div
						class="absolute top-2 right-2 -z-10 h-full w-full translate-x-1 -translate-y-1 rounded-xl bg-base-300"
					></div>
					<img
						src={album.cover}
						alt={album.name}
						class="h-full w-full object-cover transition-transform group-hover:scale-105"
					/>
				</div>
			</figure>
			<div class="card-body items-center p-4 text-center">
				<h2 class="card-title text-base">{album.name}</h2>
				<p class="text-xs text-base-content/60">{album.count} 张图片</p>
			</div>
		</a>
	{/each}
</div>

<!-- DaisyUI Modal: 新建相册 -->
<dialog bind:this={modal} class="modal">
	<div class="modal-box">
		<h3 class="text-lg font-bold">新建相册</h3>
		<p class="py-4">请输入新相册的名称：</p>
		<input
			type="text"
			placeholder="例如：旅行照片"
			class="input-bordered input w-full"
			bind:value={newAlbumName}
			onkeydown={(e) => e.key === 'Enter' && createAlbum()}
		/>
		<div class="modal-action">
			<form method="dialog">
				<!-- if there is a button in form, it will close the modal -->
				<button class="btn">取消</button>
			</form>
			<button class="btn btn-primary" onclick={createAlbum}>创建</button>
		</div>
	</div>
	<!-- 点击背景关闭 -->
	<form method="dialog" class="modal-backdrop">
		<button>close</button>
	</form>
</dialog>
