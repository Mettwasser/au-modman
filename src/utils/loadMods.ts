import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';
import type { Modification } from '../types/Modification';
import { invoke } from '@tauri-apps/api';

export const loadMods: () => Promise<Modification[]> = () =>
	// @ts-ignore
	invoke('get_modifications').catch((why) => {
		const toast: ToastSettings = {
			message: `Failed to load Mods: ${why}`,
			background: 'variant-filled-error',
			timeout: 5000,
			hoverable: true
		};
		getToastStore().trigger(toast);
	});
