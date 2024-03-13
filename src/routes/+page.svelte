<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { getVersion } from '@tauri-apps/api/app';

	let lastPlayed: Promise<string> = invoke('get_config', { configName: 'last_played' });
	let profileCount: Promise<number> = invoke('get_profile_count');
	let version = getVersion();
</script>

<div
	class="
	child:home-card grid h-full
	w-full
	grid-cols-5 grid-rows-2 gap-2
	child:items-center
	child:justify-center
	child:gap-2
	child:text-center
	"
>
	<div class="col-start-1 col-end-3 !bg-gradient-to-br from-primary-300/80 to-tertiary-700/80">
		<header class="h3 card-header">Last played</header>
		<section class="text-center">
			{#await lastPlayed then lastPlayed}
				{new Date(lastPlayed).toDateString()}
			{:catch}
				-
			{/await}
		</section>
	</div>
	<div class="col-start-3 col-end-6 !bg-gradient-to-bl from-success-400/80 to-secondary-400/80">
		<header class="h3 card-header">Profiles</header>
		<section class="text-center">
			{#await profileCount then profiles}
				{profiles}
			{/await}
		</section>
	</div>
	<div class="col-start-1 col-end-4 !bg-gradient-to-tr from-warning-400/80 to-error-500/80">
		<header class="h3 card-header">with ❤️ by Mettwasser</header>
	</div>
	<div class="col-start-4 col-end-6 !bg-gradient-to-tl from-error-400/80 to-tertiary-400/80">
		{#await version then version}
			<header class="h3 card-header">Version {version}</header>
		{/await}
	</div>
</div>
