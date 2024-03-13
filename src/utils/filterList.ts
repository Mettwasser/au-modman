import type { Modification } from '../types/Modification';

export function filterMods(list: Modification[], filter: string): Modification[] {
	if (filter) return list.filter((item) => item.name.toLocaleLowerCase().includes(filter.toLocaleLowerCase()));
	else return list;
}
