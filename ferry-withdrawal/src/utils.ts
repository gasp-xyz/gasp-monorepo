export function isEqual(first: Uint8Array, second: Uint8Array): boolean {
	if (first.length !== second.length) {
		return false;
	}
	return first.every((value, index) => value === second[index]);
}
