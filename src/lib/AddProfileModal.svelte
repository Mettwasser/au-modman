<script lang="ts">
	$$restProps;
	import { ListBox, ListBoxItem, getModalStore, type ToastSettings, getToastStore } from '@skeletonlabs/skeleton';
	import { filterMods } from '../utils/filterList';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faCheck } from '@fortawesome/free-solid-svg-icons';
	import { loadMods } from '../utils/loadMods';
	import { Circle, Pulse } from 'svelte-loading-spinners';
	import type { Modification } from '../types/Modification';
	import type { SvelteComponent } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	// stuff to return
	let profileName: string = '';
	let selectedMods: Modification[] = [];
	let confirmed = false;

	// vars for searching through the mods
	let filter = '';
	let mods = loadMods();

	async function addProfile() {
		if (!profileName) {
			const toast: ToastSettings = {
				message: 'All fields must contain a value!',
				background: 'variant-filled-error',
				timeout: 5000
			};
			toastStore.trigger(toast);
			return;
		}

		if (selectedMods.length == 0) {
			const toast: ToastSettings = {
				message: 'Please select at least 1 modification!',
				background: 'variant-filled-error',
				timeout: 5000
			};
			toastStore.trigger(toast);
			return;
		}

		confirmed = true;
		await invoke('add_profile', { profileName: profileName, modifications: selectedMods }).catch((why) => {
			const toast: ToastSettings = {
				message: `Couldn't add Profile: ${why}`,
				background: 'variant-filled-error',
				timeout: 5000,
				hoverable: true
			};
			toastStore.trigger(toast);
			confirmed = false;
		});

		if (confirmed) {
			$modalStore[0].response!(true);
			modalStore.close();
		}
	}
</script>

{#if $modalStore[0]}
	<div class="max-h-[650px] w-[650px] rounded-lg bg-surface-800">
		<div class="flex flex-col justify-start space-y-2 p-8">
			<h2 class="mb-4 ml-4 text-center text-4xl">Add a Profile</h2>
			<div
				class="
                !mb-4 mt-6 flex flex-col flex-wrap items-center gap-6 child:w-2/3 child:space-y-1 [&>div>input]:rounded-lg
                "
			>
				<div>
					<label for="profileName">Profile Name</label>
					<input
						id="profileName"
						autocomplete="off"
						bind:value={profileName}
						class="input"
						type="text"
						placeholder="Profile Name..."
					/>
				</div>

				<div>
					<h3 class="mb-4 ml-4 text-center text-2xl">Mods</h3>
					<input class="input" autocomplete="off" type="search" bind:value={filter} placeholder="Search..." />
					<div class="h-full max-h-60 overflow-y-auto">
						{#await mods}
							<Pulse color="#FFFFFF" />
						{:then mods}
							<ListBox active="variant-filled-success !text-white" hover="hover:variant-soft-success" multiple class="w-full">
								{#each filterMods(mods, filter) as mod (mod)}
									<ListBoxItem bind:group={selectedMods} name="medium" value={mod}>
										<svelte:fragment slot="trail">
											{#if selectedMods.includes(mod)}
												<FontAwesomeIcon icon={faCheck} />
											{/if}
										</svelte:fragment>
										{mod.name}
									</ListBoxItem>
								{/each}
							</ListBox>
						{/await}
					</div>
				</div>
			</div>
			<div class="mr-5 flex justify-end space-x-2">
				<button class="on variant-filled-error-400 btn outline-0" on:click={() => parent.onClose()}>Cancel</button>
				{#if !confirmed}
					<button class="variant-filled-success btn !text-white outline-0" on:click={addProfile}>Create</button>
				{:else}
					<!-- LOCK SCREEN -->
					<div class="fixed inset-0 z-[9999] bg-black opacity-30" />
					<div class="flex items-center justify-center">
						<Circle color="#FFFFFF" size="35" />
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}
