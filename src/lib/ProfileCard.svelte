<script lang="ts">
	import '@fortawesome/fontawesome-svg-core/styles.css';
	import { faPlay, faTrash, faWrench } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { ProfileWithMods } from '../types/Profile';
	import { Circle } from 'svelte-loading-spinners';
	import type { Writable } from 'svelte/store';

	export let profile: ProfileWithMods;
	export let playingProfile: Writable<ProfileWithMods | null>;
	export let onDelete: (profile: ProfileWithMods) => Promise<void>;
	export let onEdit: (profile: ProfileWithMods) => Promise<void>;
	export let onPlay: (profile: ProfileWithMods) => Promise<void>;

	let isPlaying = profile == $playingProfile;
	let shouldBeDisabled = $playingProfile != null;
	playingProfile.subscribe((playingProfile) => {
		isPlaying = profile == playingProfile;
		shouldBeDisabled = playingProfile != null;
	});
</script>

<div class="card flex w-full items-center rounded-xl border-[1px] border-white/20 !bg-surface-800/45">
	<header class="h6 m-4 text-center align-middle">{profile.name}</header>
	<div class="m-5 flex flex-1 justify-end">
		<div class="flex gap-x-2">
			<button
				type="button"
				class="btn-icon-md variant-filled-error btn-icon h-8 w-8 rounded-md"
				on:click={() => onDelete(profile)}
			>
				<FontAwesomeIcon icon={faTrash} style="color: white;" />
			</button>
			<button
				type="button"
				class="btn-icon-md variant-filled-surface btn-icon h-8 w-8 rounded-md"
				on:click={() => onEdit(profile)}
			>
				<FontAwesomeIcon icon={faWrench} style="color: white;" />
			</button>
			<button
				type="button"
				class="btn-icon-md variant-filled-success btn-icon h-8 w-8 rounded-md"
				disabled={shouldBeDisabled}
				on:click={() => onPlay(profile)}
			>
				{#if isPlaying}
					<Circle color="#FFFFFF" size="16" />
				{:else}
					<FontAwesomeIcon icon={faPlay} style="color: white;" />
				{/if}
			</button>
		</div>
	</div>
</div>
