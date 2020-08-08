/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownFilePath} from "@internal/path";

type LockResolve<RawKey, MapKey> = (lock: Lock<RawKey, MapKey>) => void;

class Lock<RawKey, MapKey> {
	constructor(locker: LockerNormalized<RawKey, MapKey>, mapKey: MapKey) {
		this.locker = locker;
		this.resolves = [];
		this.mapKey = mapKey;
	}

	private locker: LockerNormalized<RawKey, MapKey>;
	private resolves: Array<LockResolve<RawKey, MapKey>>;
	private mapKey: MapKey;

	public addResolve(resolve: LockResolve<RawKey, MapKey>) {
		this.resolves.push(resolve);
	}

	public release() {
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

	public locks: Map<MapKey, Lock<RawKey, MapKey>>;

	protected normalizeKey(rawKey: RawKey): MapKey {
		throw new Error("Unimplemented");
	}

	public hasLock(key: RawKey): boolean {
		return this.locks.has(this.normalizeKey(key));
	}

	public getNewLock(rawKey: RawKey): Lock<RawKey, MapKey> {
		const mapKey = this.normalizeKey(rawKey);
		if (this.locks.has(mapKey)) {
			throw new Error("Expected no lock to exist");
		}

		const lock = new Lock(this, mapKey);
		this.locks.set(mapKey, lock);
		return lock;
	}

	public async getLock(rawKey: RawKey): Promise<Lock<RawKey, MapKey>> {
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

	public async waitLockDrained(key: RawKey): Promise<void> {
		if (this.hasLock(key)) {
			const lock = await this.getLock(key);
			lock.release();

			if (this.hasLock(key)) {
				return this.waitLockDrained(key);
			}
		}
	}

	public async waitLock(key: RawKey): Promise<void> {
		if (this.hasLock(key)) {
			const lock = await this.getLock(key);
			lock.release();
		}
	}

	public async wrapLock<T>(
		key: RawKey,
		callback: () => T | Promise<T>,
	): Promise<T> {
		const lock = await this.getLock(key);
		try {
			return await callback();
		} finally {
			lock.release();
		}
	}
}

export class Locker<Key> extends LockerNormalized<Key, Key> {
	protected normalizeKey(key: Key): Key {
		return key;
	}
}

export class SingleLocker extends LockerNormalized<void, void> {
	protected normalizeKey(key: void): void {
		return key;
	}
}

export class FilePathLocker extends LockerNormalized<UnknownFilePath, string> {
	protected normalizeKey(path: UnknownFilePath): string {
		return path.join();
	}
}
