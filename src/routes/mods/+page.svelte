<script lang="ts">
	import { getModalStore, getToastStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings, ToastSettings } from '@skeletonlabs/skeleton';
	import ModCard from '$lib/ModCard.svelte';
	import { Pulse } from 'svelte-loading-spinners';
	import { loadMods } from '../../utils/loadMods.js';
	import { invoke } from '@tauri-apps/api';
	import type { Modification } from '../../types/Modification.js';

	const modalStore = getModalStore();

	let task = loadMods();
	let toastStore = getToastStore();

	async function addMod() {
		let added = await new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'component',
				component: 'addMod',
				response(r) {
					resolve(r);
				}
			};

			modalStore.trigger(modal);
		});

		if (added) {
			task = loadMods();
		}
	}

	async function askDelete(mod: Modification) {
		let promise = new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'confirm',
				title: 'Delete a Mod',
				body: 'Do you really want to delete this Mod? Note that this will delete all Profiles related to this mod as well.',
				response: (r: boolean) => {
					resolve(r);
				}
			};

			modalStore.trigger(modal);
		});

		let confirmed = await promise;
		if (confirmed) {
			await invoke('delete_modification', { modification: mod }).catch((why) => {
				const toast: ToastSettings = {
					message: `Couldn't add Mod: ${why}`,
					background: 'variant-filled-error',
					timeout: 5000,
					hoverable: true
				};
				toastStore.trigger(toast);
			});

			task = loadMods();
		}
	}

	async function editMod(mod: Modification) {
		let confirmed = await new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'component',
				component: 'editMod',
				meta: { ...mod },
				response(r) {
					resolve(r);
				}
			};

			modalStore.trigger(modal);
		});

		if (confirmed) {
			task = loadMods();
		}
	}
</script>

<div class="flex w-4/5 flex-col items-center">
	<div class="flex w-full justify-end">
		<button type="button" on:click={addMod} class="variant-filled-success btn btn-md rounded-md !text-white"> Add Mod </button>
	</div>

	<!-- For spacing -->
	<div class="my-2" />
	<!--  -->

	<div class="flex h-full w-full flex-col items-center gap-3 overflow-y-auto rounded-xl bg-black bg-opacity-20 p-4">
		{#await task}
			<div class="flex flex-1 items-center justify-center">
				<Pulse color="#FFFFFF" />
			</div>
		{:then mods}
			{#each mods as mod (mod)}
				<ModCard {mod} onDelete={askDelete} onEdit={editMod}></ModCard>
			{/each}
		{/await}
	</div>
</div>
