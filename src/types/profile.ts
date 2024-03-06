import type { Modification } from './modification';

export interface Profile {
	name: string;
	folder_location: string;
	created_at: Date;
	changed_at: Date | null;
	modifications: Array<Modification>;
}
