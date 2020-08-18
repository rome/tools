export type ExtendedMapGetDefaultValue<Key, Value> = (key: Key) => Value;

// We do not extend Map, as that would allow us to satisfy the type and go places where we shouldn't such as over a
// bridge
export class ExtendedMap<Key, Value> {
	constructor(
		name: string,
		getDefaultValue?: ExtendedMapGetDefaultValue<Key, Value>,
	) {
		this.map = new Map();
		this.name = name;
		this.getDefaultValue = getDefaultValue;
	}

	private map: Map<Key, Value>;
	private name: string;
	private getDefaultValue: undefined | ExtendedMapGetDefaultValue<Key, Value>;

	toMap(): Map<Key, Value> {
		return this.map;
	}

	assert(key: Key, allowDefault: boolean = true): Value {
		const val = this.map.get(key);
		if (val !== undefined) {
			return val;
		}

		const {getDefaultValue} = this;
		if (getDefaultValue !== undefined && allowDefault) {
			const value = getDefaultValue(key);
			this.set(key, value);
			return value;
		} else {
			throw new Error(`Map ${this.name} does not contain key ${String(key)}`);
		}
	}

	get size(): number {
		return this.map.size;
	}

	has(key: Key): boolean {
		return this.map.has(key);
	}

	get(key: Key): undefined | Value {
		return this.map.get(key);
	}

	set(key: Key, value: Value) {
		this.map.set(key, value);
	}

	delete(key: Key) {
		return this.map.delete(key);
	}

	keys(): IterableIterator<Key> {
		return this.map.keys();
	}

	values(): IterableIterator<Value> {
		return this.map.values();
	}

	[Symbol.iterator](): IterableIterator<[Key, Value]> {
		return this.map[Symbol.iterator]();
	}
}
