/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyFilePath} from "@internal/path";
import {VoidCallback} from "@internal/typescript-helpers";
import {createDeferredPromise} from "@internal/async/index";
import {Event} from "@internal/events";

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

abstract class LockerNormalized<RawKey, MapKey> {
	constructor() {
		this.locks = new Map();
	}

	public locks: Map<MapKey, Lock<RawKey, MapKey>>;

	protected abstract normalizeKey(rawKey: RawKey): MapKey

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
		while (this.hasLock(key)) {
			const lock = await this.getLock(key);
			lock.release();
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

export type UnknownLocker = LockerNormalized<unknown, unknown>;

export class Locker<Key> extends LockerNormalized<Key, Key> {
	protected normalizeKey(key: Key): Key {
		return key;
	}
}

export class FilePathLocker extends LockerNormalized<AnyFilePath, string> {
	protected normalizeKey(path: AnyFilePath): string {
		return path.join();
	}
}

export class GlobalLock {
	constructor() {
		this.resolves = [];
		this.dependencies = 0;

		this.incrementEvent = new Event({
			name: "incrementEvent",
		});

		this.decrementEvent = new Event({
			name: "decrementEvent",
		});
	}

	private incrementEvent: Event<void, void>;
	private decrementEvent: Event<void, void>;
	private resolves: Array<VoidCallback>;
	private dependencies: number;

	attachLock(lock: GlobalLock) {
		lock.incrementEvent.subscribe(() => {
			this.dependencies++;
		});

		lock.decrementEvent.subscribe(() => {
			this.dependencies--;

			if (this.dependencies === 0) {
				this.unlock();
			}
		});
	}

	private unlock() {
		for (const resolve of this.resolves) {
			resolve();
		}
		this.resolves = [];
	}

	public async wrap<T>(fn: () => Promise<T>): Promise<T> {
		try {
			this.dependencies++;
			return await fn();
		} finally {
			this.dependencies--;

			if (this.dependencies === 0) {
				this.unlock();
			}
		}
	}

	public async wait() {
		if (this.dependencies > 0) {
			const {resolve, promise} = createDeferredPromise();
			this.resolves.push(resolve);
			await promise;
		}
	}
}
