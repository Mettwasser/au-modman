<script lang="ts">
	import { getModalStore, getToastStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings, ToastSettings } from '@skeletonlabs/skeleton';
	import ModCard from '$lib/ModCard.svelte';
	import { Pulse } from 'svelte-loading-spinners';
	import { loadMods } from '../../utils/loadMods.js';
	import { invoke } from '@tauri-apps/api';
	import type { Modification } from '../../types/Modification.js';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faFolder } from '@fortawesome/free-solid-svg-icons';
	import { open } from '@tauri-apps/api/dialog';
	import { exists, readDir } from '@tauri-apps/api/fs';
	import { loadSettingAuInstallDir } from '../../utils/loadSettings.js';

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	let amongUsInstallationPathTask = loadSettingAuInstallDir();

	async function setAmongUsInstallationDir() {
		const directoryPath = await open({
			title: 'Select the Among Us Installation Directory',
			directory: true
		});

		if (!directoryPath) return;

		const directory = await readDir(directoryPath as string, { recursive: true });

		// check whether Among Us.exe is NOT in the directory
		if (!directory.some((entry) => entry.name == 'Among Us.exe')) {
			// if so, cancel with an error
			const toast: ToastSettings = {
				message: `Invalid Directory: Couldn't find 'Among Us.exe' in the directory.`,
				background: 'variant-filled-error',
				timeout: 5000
			};
			toastStore.trigger(toast);
			return;
		}

		await invoke('set_config', { configName: 'au_install_dir', value: directoryPath }).catch((why) => {
			const toast: ToastSettings = {
				message: `Failed to Save the Installation Path: ${why}`,
				background: 'variant-filled-error',
				timeout: 5000
			};
			toastStore.trigger(toast);
		});
		amongUsInstallationPathTask = loadSettingAuInstallDir();
	}
</script>

<div class="flex w-4/5 flex-col items-center">
	<div class="flex h-full w-full flex-col items-center gap-6 overflow-y-auto rounded-xl">
		<div class="mt-4 flex w-full flex-1 flex-col items-center justify-center gap-4">
			{#await amongUsInstallationPathTask}
				<Pulse color="#FFFFFF" />
			{:then setting}
				<div class="flex w-full flex-1 flex-col justify-start">
					<h2 class="h2">Among Us Installation Path</h2>
					<div class="flex w-full gap-4">
						<input
							id="amongUsInstallationPathInputField"
							value={setting}
							autocomplete="off"
							class="input rounded-lg"
							type="text"
							readonly
							tabindex="-1"
						/>
						<div class="flex flex-1 justify-end">
							<button class="btn-icon-md btn-icon rounded-md !bg-surface-600" on:click={setAmongUsInstallationDir}>
								<FontAwesomeIcon icon={faFolder} fixedWidth={false} />
							</button>
						</div>
					</div>
				</div>
			{/await}
		</div>
	</div>
</div>
