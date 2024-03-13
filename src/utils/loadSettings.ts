import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';
import { invoke } from '@tauri-apps/api';

export const loadSettingAuInstallDir: () => Promise<string | null> = () =>
	// @ts-ignore
	invoke('get_config', { configName: 'au_install_dir' }).catch((why) => {
		const toast: ToastSettings = {
			message: `Failed to load Mods: ${why}`,
			background: 'variant-filled-error',
			timeout: 5000,
			hoverable: true
		};
		getToastStore().trigger(toast);
	});
