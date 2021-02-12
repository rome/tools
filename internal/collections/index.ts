export type ExtendedMapGetDefaultValue<Key, Value> = (key: Key) => Value;

// We do not extend Map, as that would allow us to satisfy the type and go places where we shouldn't such as over a
// bridge
export class ExtendedMap<Key, Value> implements Map<Key, Value> {
	constructor(
		name: string,
		getDefaultValue?: ExtendedMapGetDefaultValue<Key, Value>,
	) {
		this.map = new Map();
		this.name = name;
		this.getDefaultValue = getDefaultValue;
		this[Symbol.toStringTag] = "ExtendedMap";
	}

	public [Symbol.toStringTag]: string;
	private map: Map<Key, Value>;
	private name: string;
	private getDefaultValue: undefined | ExtendedMapGetDefaultValue<Key, Value>;

	public clear() {
		this.map.clear();
	}

	public forEach(
		callback: (value: Value, key: Key, thisArg: Map<Key, Value>) => void,
	): void {
		return this.map.forEach(callback);
	}

	public toMap(): Map<Key, Value> {
		return this.map;
	}

	public assert(key: Key, allowDefault: boolean = true): Value {
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

	public has(key: Key): boolean {
		return this.map.has(key);
	}

	public get(key: Key): undefined | Value {
		return this.map.get(key);
	}

	public set(key: Key, value: Value): this {
		this.map.set(key, value);
		return this;
	}

	public delete(key: Key) {
		return this.map.delete(key);
	}

	public keys(): IterableIterator<Key> {
		return this.map.keys();
	}

	public values(): IterableIterator<Value> {
		return this.map.values();
	}

	public entries(): IterableIterator<[Key, Value]> {
		return this.map.entries();
	}

	[Symbol.iterator](): IterableIterator<[Key, Value]> {
		return this.map[Symbol.iterator]();
	}
}

type Primitive = string | number | boolean;

type MappedSetSerializer<Real, Serial extends Primitive> = (
	real: Real,
) => [Serial, Real];

// Allow values of a set to be treated like primitives
export class MappedSet<Real, Serial extends Primitive> implements Set<Real> {
	constructor(
		serialize: MappedSetSerializer<Real, Serial>,
		entries?: Iterable<Real>,
	) {
		this.map = new Map();
		this.set = new Set();
		this.serialize = serialize;
		this[Symbol.toStringTag] = "MappedSet";

		if (entries !== undefined) {
			for (const path of entries) {
				this.add(path);
			}
		}
	}

	private map: Map<Serial, Real>;
	private set: Set<Real>;
	private serialize: MappedSetSerializer<Real, Serial>;
	public [Symbol.toStringTag]: string;

	get size(): number {
		return this.set.size;
	}

	public forEach(
		callback: (value: Real, value2: Real, set: MappedSet<Real, Serial>) => void,
	): void {
		for (const value of this) {
			callback(value, value, this);
		}
	}

	public entries(): IterableIterator<[Real, Real]> {
		return this.set.entries();
	}

	public keys(): IterableIterator<Real> {
		return this.set.keys();
	}

	public values(): IterableIterator<Real> {
		return this.set.values();
	}

	public [Symbol.iterator](): IterableIterator<Real> {
		return this.set.keys()[Symbol.iterator]();
	}

	public has(real: Real) {
		return this.map.has(this.serialize(real)[0]);
	}

	public add(raw: Real): this {
		const [serial, real] = this.serialize(raw);
		if (!this.map.has(serial)) {
			this.map.set(serial, real);
			this.set.add(real);
		}
		return this;
	}

	public delete(real: Real): boolean {
		const [serial] = this.serialize(real);
		if (this.map.has(serial)) {
			this.set.delete(this.map.get(serial) as Real);
			this.map.delete(serial);
			return true;
		} else {
			return false;
		}
	}

	public clear() {
		this.map.clear();
	}
}

type MappedKeyMapSerializer<RealKey, SerialKey extends Primitive> = (
	real: RealKey,
) => [SerialKey, RealKey];

// Allow values of a set to be treated like primitives
export class MappedKeyMap<RealKey, SerialKey extends Primitive, Value>
	implements Map<RealKey, Value> {
	constructor(
		serialize: MappedKeyMapSerializer<RealKey, SerialKey>,
		entries?: [RealKey, Value][],
	) {
		this.serialToValue = new Map();
		this.serialToRealKey = new Map();

		this.serialize = serialize;
		this[Symbol.toStringTag] = "MappedMap";

		if (entries !== undefined) {
			for (const [key, value] of entries) {
				this.set(key, value);
			}
		}
	}

	private serialize: MappedKeyMapSerializer<RealKey, SerialKey>;

	private serialToValue: Map<SerialKey, Value>;
	private serialToRealKey: Map<SerialKey, RealKey>;

	public [Symbol.toStringTag]: string;

	public get size(): number {
		return this.serialToValue.size;
	}

	public forEach(
		callback: (
			value: Value,
			key: RealKey,
			map: MappedKeyMap<RealKey, SerialKey, Value>,
		) => void,
	): void {
		for (const [key, value] of this) {
			callback(value, key, this);
		}
	}

	public *[Symbol.iterator](): IterableIterator<[RealKey, Value]> {
		for (const [serialKey, value] of this.serialToValue) {
			const realKey = this.serialToRealKey.get(serialKey)!;
			yield [realKey, value];
		}
	}

	public entries(): IterableIterator<[RealKey, Value]> {
		return this[Symbol.iterator]();
	}

	public clear() {
		this.serialToValue.clear();
		this.serialToRealKey.clear();
	}

	public keys(): IterableIterator<RealKey> {
		return this.serialToRealKey.values();
	}

	public values(): IterableIterator<Value> {
		return this.serialToValue.values();
	}

	public delete(realKey: RealKey): boolean {
		const [serialKey] = this.serialize(realKey);
		this.serialToRealKey.delete(serialKey);
		return this.serialToValue.delete(serialKey);
	}

	public has(key: RealKey): boolean {
		return this.serialToValue.has(this.serialize(key)[0]);
	}

	public assert(realKey: RealKey): Value {
		const item = this.get(realKey);
		if (item === undefined) {
			const [serialKey] = this.serialize(realKey);
			throw new Error(`Could not find element for ${serialKey}`);
		} else {
			return item;
		}
	}

	public get(key: RealKey): undefined | Value {
		return this.serialToValue.get(this.serialize(key)[0]);
	}

	public set(rawRealKey: RealKey, value: Value): this {
		const [serialKey, realKey] = this.serialize(rawRealKey);
		this.serialToRealKey.set(serialKey, realKey);
		this.serialToValue.set(serialKey, value);
		return this;
	}
}
