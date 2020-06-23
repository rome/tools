/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownFilePath} from "@romejs/path";

type LockResolve<RawKey, MapKey> = (lock: Lock<RawKey, MapKey>) => void;

class Lock<RawKey, MapKey> {
	constructor(
		locker: LockerNormalized<RawKey, MapKey>,
		rawKey: RawKey,
		mapKey: MapKey,
	) {
		this.locker = locker;
		this.resolves = [];
		this.rawKey = rawKey;
		this.mapKey = mapKey;
	}

	locker: LockerNormalized<RawKey, MapKey>;
	resolves: Array<LockResolve<RawKey, MapKey>>;
	rawKey: RawKey;
	mapKey: MapKey;

	addResolve(resolve: LockResolve<RawKey, MapKey>) {
		this.resolves.push(resolve);
	}

	release() {
		const {resolves} = this;

		if (resolves.length === 0) {
			this.locker.locks.delete(this.mapKey);
		} else {
			const resolve = resolves.shift();
			if (resolve === undefined) {
				throw new Error("Already validated resolved.length aboved");
			}
			resolve(this);
		}
	}
}

class LockerNormalized<RawKey, MapKey> {
	constructor() {
		this.locks = new Map();
	}

	locks: Map<MapKey, Lock<RawKey, MapKey>>;

	normalizeKey(rawKey: RawKey): MapKey {
		throw new Error("Unimplemented");
	}

	hasLock(key: RawKey): boolean {
		return this.locks.has(this.normalizeKey(key));
	}

	getNewLock(rawKey: RawKey): Lock<RawKey, MapKey> {
		const mapKey = this.normalizeKey(rawKey);
		if (this.locks.has(mapKey)) {
			throw new Error("Expected no lock to exist");
		}

		const lock = new Lock(this, rawKey, mapKey);
		this.locks.set(mapKey, lock);
		return lock;
	}

	async getLock(rawKey: RawKey): Promise<Lock<RawKey, MapKey>> {
		const key = this.normalizeKey(rawKey);
		const existingLock = this.locks.get(key);

		if (existingLock === undefined) {
			return this.getNewLock(rawKey);
		} else {
			return new Promise((resolve) => {
				existingLock.addResolve(resolve);
			});
		}
	}

	async waitLock(key: RawKey): Promise<void> {
		if (this.hasLock(key)) {
			const lock = await this.getLock(key);
			lock.release();
		}
	}

	async wrapLock<T>(key: RawKey, callback: () => T | Promise<T>): Promise<T> {
		const lock = await this.getLock(key);
		try {
			return await callback();
		} finally {
			lock.release();
		}
	}
}

export class Locker<Key> extends LockerNormalized<Key, Key> {
	normalizeKey(key: Key): Key {
		return key;
	}
}

export class FilePathLocker extends LockerNormalized<UnknownFilePath, string> {
	normalizeKey(path: UnknownFilePath): string {
		return path.join();
	}
}
