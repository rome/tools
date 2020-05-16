/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

type LockResolve<Key> = (lock: Lock<Key>) => void;

class Lock<Key> {
	constructor(locker: Locker<Key>, key: Key) {
		this.locker = locker;
		this.resolves = [];
		this.key = key;
	}

	locker: Locker<Key>;
	resolves: Array<LockResolve<Key>>;
	key: Key;

	addResolve(resolve: LockResolve<Key>) {
		this.resolves.push(resolve);
	}

	release() {
		const {resolves} = this;

		if (resolves.length === 0) {
			this.locker.locks.delete(this.key);
		} else {
			const resolve = resolves.shift();
			if (resolve === undefined) {
				throw new Error('Already validated resolved.length aboved');
			}
			resolve(this);
		}
	}
}

export default class Locker<Key> {
	constructor() {
		this.locks = new Map();
	}

	locks: Map<Key, Lock<Key>>;

	hasLock(id: Key): boolean {
		return this.locks.has(id);
	}

	getNewLock(key: Key): Lock<Key> {
		if (this.locks.has(key)) {
			throw new Error('Expected no lock to exist');
		}

		const lock = new Lock(this, key);
		this.locks.set(key, lock);
		return lock;
	}

	async getLock(key: Key): Promise<Lock<Key>> {
		const existingLock = this.locks.get(key);

		if (existingLock === undefined) {
			return this.getNewLock(key);
		} else {
			return new Promise((resolve) => {
				existingLock.addResolve(resolve);
			});
		}
	}

	async waitLock(key: Key): Promise<void> {
		if (this.hasLock(key)) {
			const lock = await this.getLock(key);
			lock.release();
		}
	}
}
