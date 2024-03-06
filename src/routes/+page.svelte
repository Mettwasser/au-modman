<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';

	let version = getVersion();
</script>

<div
	class="
	grid grid-cols-3 grid-rows-2
	justify-center items-center
	gap-2
	mx-auto my-2
	child:h-full child:text-center child:bg-primary-hover-token child:items-center child:justify-center
	h-full
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
	{#await version then version}
		<div class="card p-4 flex gap-4 flex-col rounded-xl col-start-3 col-end-4 h-full">
			<header class="card-header h3">Version {version}</header>
		</div>
	{/await}
</div>
