import type { SurrealID } from './SurrealID';

export interface Modification {
	id: SurrealID;
	downloadUrl: string;
	name: string;
	version: string;
	createdAt: Date;
}
