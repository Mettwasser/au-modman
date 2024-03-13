import { writable, type Writable } from 'svelte/store';
import type { ProfileWithMods } from './types/Profile';

export const playingProfile: Writable<ProfileWithMods | null> = writable(null);
