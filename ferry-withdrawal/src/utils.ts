export function isEqual(first: Uint8Array, second: Uint8Array): boolean {
	if (first.length !== second.length) {
		return false;
	}
	return first.every((value, index) => value === second[index]);
}

export function maxBigInt(...args: bigint[]) {
	return BigInt(args.reduce((min, current) => (current > min ? current : min)));
}

export function minBigInt(...args: bigint[]) {
	return BigInt(args.reduce((min, current) => (current < min ? current : min)));
}

export async function asyncFilter<T>(arr: T[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}
