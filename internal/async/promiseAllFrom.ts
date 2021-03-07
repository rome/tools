export default function promiseAllFrom<V, T>(
	iterable: Iterable<V>,
	callback: (value: V) => Promise<T> | T,
): Promise<T[]> {
	return Promise.all(Array.from(iterable, callback));
}
