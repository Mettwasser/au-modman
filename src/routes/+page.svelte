<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';

	let version = getVersion();
</script>

<div
	class="
	grid grid-cols-3 grid-rows-2
	gap-2
	h-full
	child:text-center child:bg-primary-hover-token child:items-center child:justify-center
	"
>
	<div class="card p-4 flex gap-4 flex-col rounded-xl col-start-1 col-end-2">
		<header class="card-header h3">Last played</header>
		<section class="text-center">3.4.2024 21:55</section>
	</div>
	<div class="card p-4 flex gap-4 flex-col rounded-xl col-start-2 col-end-4">
		<header class="card-header h3">Profiles</header>
		<section class="text-center">7</section>
	</div>
	<div class="card p-4 flex gap-4 rounded-xl col-start-1 col-end-3">
		<header class="card-header h3">with ❤️ by Mettwasser & Vali</header>
	</div>
	<div class="card p-4 flex gap-4 flex-col rounded-xl col-start-3 col-end-4 h-full">
		{#await version then version}
			<header class="card-header h3">Version {version}</header>
		{/await}
	</div>
</div>
