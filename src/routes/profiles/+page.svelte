<script lang="ts">
	import ProfileCard from '$lib/ProfileCard.svelte';
	import { getModalStore, getToastStore, type ModalSettings, type ToastSettings } from '@skeletonlabs/skeleton';
	import { loadProfiles } from '../../utils/loadProfiles';
	import { Pulse } from 'svelte-loading-spinners';
	import { invoke } from '@tauri-apps/api';
	import type { ProfileWithMods } from '../../types/Profile';

	const modalStore = getModalStore();
	let toastStore = getToastStore();
	let task = loadProfiles();

	async function addProfile() {
		let added = await new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'component',
				component: 'addProfile',

				response: (r: boolean) => resolve(r)
			};

			modalStore.trigger(modal);
		});

		if (added) {
			task = loadProfiles();
		}
	}

	async function askDelete(profile: ProfileWithMods) {
		let promise = new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'confirm',
				title: 'Delete a Profile',
				body: `Do you really want to delete the Profile "${profile.name}"?`,
				response: (r: boolean) => {
					resolve(r);
				}
			};

			modalStore.trigger(modal);
		});

		let confirmed = await promise;
		if (confirmed) {
			await invoke('delete_profile', {
				profile: { ...profile, modifications: profile.modifications.map((mod) => mod.id) }
			}).catch((why) => {
				const toast: ToastSettings = {
					message: `Couldn't add Mod: ${why}`,
					background: 'variant-filled-error',
					timeout: 5000,
					hoverable: true
				};
				toastStore.trigger(toast);
			});

			task = loadProfiles();
		}
	}

	async function editProfile(profile: ProfileWithMods) {
		let confirmed = await new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'component',
				component: 'editProfile',
				meta: { profile },
				response(r) {
					resolve(r);
				}
			};

			modalStore.trigger(modal);
		});

		if (confirmed) {
			task = loadProfiles();
		}
	}

	async function launchProfile(profile: ProfileWithMods) {
		await invoke('launch_profile', { profile: { ...profile, modifications: profile.modifications.map((mod) => mod.id) } }).catch(
			(why) => {
				const toast: ToastSettings = {
					message: `Couldn't launch Profile: ${why}`,
					background: 'variant-filled-error',
					timeout: 5000,
					hoverable: true
				};
				toastStore.trigger(toast);
			}
		);
	}
</script>

<div class="flex w-4/5 flex-col items-center">
	<div class="flex w-full justify-end">
		<button type="button" on:click={addProfile} class="variant-filled-success btn btn-md rounded-md !text-white">
			Add Profile
		</button>
	</div>

	<!-- For spacing -->
	<div class="my-2" />
	<!--  -->

	<div class="flex h-full w-full flex-wrap justify-center overflow-y-auto rounded-xl bg-black bg-opacity-20">
		{#await task}
			<div class="flex flex-1 items-center justify-center">
				<Pulse color="#FFFFFF" />
			</div>
		{:then profiles}
			{#each profiles as profile (profile)}
				<div class="m-4 mx-6 basis-5/12">
					<ProfileCard {profile} onDelete={askDelete} onEdit={editProfile} onPlay={launchProfile} />
				</div>
			{/each}
		{/await}
	</div>
</div>
