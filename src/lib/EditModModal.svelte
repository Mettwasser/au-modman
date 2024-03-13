<script lang="ts">
	import { getModalStore, type ToastSettings } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import type { SvelteComponent } from 'svelte';
	import { Circle } from 'svelte-loading-spinners';

	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	// stuff to return
	let oldProfileName: string = $modalStore[0].meta!.name;

	// bound values
	let profileName: string = $modalStore[0].meta!.name;
	let version: string = $modalStore[0].meta!.version;
	let confirmed = false;

	async function editMod() {
		if (!profileName) {
			const toast: ToastSettings = {
				message: 'All fields must contain a value!',
				background: 'variant-filled-error',
				timeout: 5000
			};
			toastStore.trigger(toast);
			return;
		}
		confirmed = true;
		await invoke('edit_modification', { oldName: oldProfileName, newName: profileName, version }).catch((why) => {
			const toast: ToastSettings = {
				message: `Couldn't edit Mod: ${why}`,
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
			<h2 class="mb-4 ml-4 text-center text-4xl">Mod Configuration</h2>
			<div
				class="
                !mb-4 mt-6 flex flex-col flex-wrap items-center gap-6 child:w-2/3 child:space-y-1 [&>div>input]:rounded-lg
                "
			>
				<div>
					<label for="modNameInputField">Mod Name</label>
					<input
						id="modNameInputField"
						autocomplete="off"
						bind:value={profileName}
						class="input"
						type="text"
						placeholder="Profile Name..."
					/>
				</div>
			</div>
			<div class="mr-5 flex justify-end">
				<button class="on variant-filled-error-400 btn outline-0" on:click={() => parent.onClose()}>Cancel</button>
				{#if !confirmed}
					<button class="variant-filled-success btn !text-white outline-0" on:click={editMod}>Save</button>
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
