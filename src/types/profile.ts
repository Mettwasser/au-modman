import type { Modification } from './Modification';
import type { SurrealID } from './SurrealID';

export interface ProfileWithMods {
	id: SurrealID;
	name: string;
	folderLocation: string;
	createdAt: string;
	modifications: Array<Modification>;
}
export interface Profile {
	id: SurrealID;
	name: string;
	folderLocation: string;
	createdAt: string;
	modifications: Array<SurrealID>;
}
