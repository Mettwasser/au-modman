<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';

	let version = getVersion();
</script>

<div
	class="
	grid grid-cols-5 grid-rows-2
	gap-2
	child:text-center child:items-center child:justify-center
	child:gap-2
	child:home-card
	w-full
	h-full
	"
>
	<div class="col-start-1 col-end-3 !bg-gradient-to-br from-primary-300/80 to-tertiary-700/80">
		<header class="card-header h3">Last played</header>
		<section class="text-center">3.4.2024 21:55</section>
	</div>
	<div class="col-start-3 col-end-6 !bg-gradient-to-bl from-success-400/80 to-secondary-400/80">
		<header class="card-header h3">Profiles</header>
		<section class="text-center">7</section>
	</div>
	<div class="col-start-1 col-end-4 !bg-gradient-to-tr from-warning-400/80 to-error-500/80">
		<header class="card-header h3">with ❤️ by Mettwasser</header>
	</div>
	<div class="col-start-4 col-end-6 !bg-gradient-to-tl from-error-400/80 to-tertiary-400/80">
		{#await version then version}
			<header class="card-header h3">Version {version}</header>
		{/await}
	</div>
</div>
