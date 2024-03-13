import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';
import { invoke } from '@tauri-apps/api';
import type { ProfileWithMods } from '../types/Profile';

export const loadProfiles: () => Promise<ProfileWithMods[]> = () =>
	// @ts-ignore
	invoke('get_profiles').catch((why) => {
		const toast: ToastSettings = {
			message: `Failed to load Profiles: ${why}`,
			background: 'variant-filled-error',
			timeout: 5000,
			hoverable: true
		};
		getToastStore().trigger(toast);
	});
