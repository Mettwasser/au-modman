<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';

	let version = getVersion();
</script>

<div
	class="
	grid grid-cols-3 grid-rows-2
	gap-2
	child:text-center child:items-center child:justify-center
	child:gap-2
	child:home-card
	w-full
	"
>
	<div class="col-start-1 col-end-2">
		<header class="card-header h3">Last played</header>
		<section class="text-center">3.4.2024 21:55</section>
	</div>
	<div class="col-start-2 col-end-4">
		<header class="card-header h3">Profiles</header>
		<section class="text-center">7</section>
	</div>
	<div class="col-start-1 col-end-3">
		<header class="card-header h3">with ❤️ by Mettwasser</header>
	</div>
	<div class="col-start-3 col-end-4">
		{#await version then version}
			<header class="card-header h3">Version {version}</header>
		{/await}
	</div>
</div>
