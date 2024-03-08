export function filterList(list: string[], filter: string): string[] {
	if (filter) return list.filter((item) => item.toLocaleLowerCase().includes(filter.toLocaleLowerCase()));
	else return list;
}
