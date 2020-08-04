/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import UnknownT from "./types/UnknownT";
import AnyT from "./types/AnyT";
import E from "./types/errors/E";
import Hub from "./Hub";
import T, {TypeCompatibilityReturn} from "./types/T";
import {StaticMarkup, markup} from "@internal/markup";

class ReduceRecursionError extends Error {}

const TYPE_COMPATIBLE: TypeCompatibilityReturn = {type: "compatible"};

const MAX_DEPTH = 100;

export class HumanBuilder {
	constructor() {
		this.stack = new Set();
		this.usedAliases = new Set();
		this.aliases = new Map();
	}

	stack: Set<T>;
	aliases: Map<T, StaticMarkup>;
	usedAliases: Set<string>;

	isRecursive(t: T): boolean {
		if (t.human !== undefined) {
			return false;
		}

		if (this.aliases.has(t)) {
			return true;
		}

		if (this.stack.has(t)) {
			return true;
		}

		return false;
	}

	humanize(type: T): StaticMarkup {
		// Check if we already have a human form for this type
		if (type.human !== undefined) {
			return type.human;
		}

		// Check if we have an already created alias
		if (this.aliases.has(type)) {
			const alias = this.aliases.get(type);
			if (alias === undefined) {
				throw new Error("Expected alias");
			}
			return alias;
		}

		// Generate an alias if we've determined this as recursive
		if (this.isRecursive(type)) {
			const alias = markup`Alias${type.id}`;
			this.aliases.set(type, alias);
			return alias;
		}

		// Setup the stack and call
		this.stack.add(type);
		try {
			let humanized = type.humanize(this);

			// Check if an alias was created
			const alias = this.aliases.get(type);
			if (alias !== undefined) {
				humanized = markup`${alias} = ${humanized}`;
			}
			return humanized;
		} finally {
			this.stack.delete(type);
		}
	}
}

export default class Utils {
	constructor(hub: Hub) {
		this.reduceCatchers = new Set();
		this.reduceCache = new Map();
		this.reduceStack = new Set();
		this.compatibilityDepth = 0;
		this.hub = hub;
		this.debug = false;
	}

	compatibilityDepth: number;
	reduceCatchers: Set<Set<T>>;
	reduceCache: Map<T, T>;
	reduceStack: Set<T>;
	debug: boolean;
	hub: Hub;

	inspect(t: T, safe: boolean = false): string {
		const prevDebug = this.debug;
		this.debug = true;

		const data: Map<string, string> = new Map();
		data.set("id", String(t.id));

		const {originLoc, originEvaluator} = t;
		if (originLoc === undefined) {
			data.set("origin", "unknown");
		} else {
			data.set(
				"origin",
				`${String(originLoc.filename)}:${String(originLoc.start.line)}:${String(
					originLoc.start.column,
				)}`,
			);
		}
		if (originEvaluator !== undefined) {
			data.set("evaluator", originEvaluator);
		}

		const dataStr = Array.from(data.keys()).map((key) =>
			`${key}: ${String(data.get(key))}`
		).join(", ");

		let info = `${t.getConstructor().type}<`;
		if (!safe) {
			info += `${this.humanize(t)}, `;
		}
		info += `${dataStr}>`;

		this.debug = prevDebug;
		return info;
	}

	assertClosed() {
		if (!this.debug) {
			this.hub.assertClosed();
		}
	}

	explodeUnion(type: T): Array<T> {
		return Array.from(new Set(this.reduce(type).explodeUnion()));
	}

	isCompatibleWith(a: T, b: T): boolean {
		return this.checkCompability(a, b).type === "compatible";
	}

	checkCompability(a: T, b: T): TypeCompatibilityReturn {
		this.assertClosed();

		const lower = this.reduce(a);
		const upper = this.reduce(b);

		// Exact same type
		if (lower === upper) {
			return TYPE_COMPATIBLE;
		}

		// Any types shouldn't cause errors
		if (lower instanceof AnyT || upper instanceof AnyT) {
			return TYPE_COMPATIBLE;
		}

		// Simple check for call stack limits
		if (this.compatibilityDepth > MAX_DEPTH) {
			throw new Error(
				`Max depth exceeded when checking compatibility of ${lower.inspect()} to ${upper.inspect()}`,
			);
		}

		const cached = lower.compatibilityCache.get(upper);
		if (cached === undefined) {
			lower.compatibilityCache.set(
				upper,
				{
					type: "incompatible",
					lower,
					upper,
				},
			);
		} else {
			return cached;
		}

		// Check this relationship for compatibility
		this.compatibilityDepth++;
		let ret;
		try {
			ret = lower.compatibleWith(upper);
		} catch (err) {
			if (err instanceof ReduceRecursionError) {
				ret = TYPE_COMPATIBLE;
			} else {
				throw err;
			}
		} finally {
			this.compatibilityDepth--;
		}

		let res: undefined | TypeCompatibilityReturn;
		if (ret === true) {
			res = TYPE_COMPATIBLE;
		} else if (ret === false) {
			res = {type: "incompatible", lower: a, upper: b};
		} else if (ret instanceof E) {
			res = {type: "incompatible", lower: a, upper: ret};
		} else {
			res = ret;
		}

		lower.compatibilityCache.set(upper, res);

		return res;
	}

	humanize(type: T): StaticMarkup {
		this.assertClosed();

		return new HumanBuilder().humanize(type);
	}

	reduce(type: T): T {
		//
		this.assertClosed();

		//
		const cached = this.reduceCache.get(type);
		if (cached !== undefined) {
			return cached;
		}

		// Check if we're already trying to reduce this node, in that case this is a recursion error
		if (this.reduceStack.has(type)) {
			//throw new ReduceRecursionError(`Reduce recursion error for ${this.inspect(type, true)}`);
			return new UnknownT(type.scope, type.originNode);
		}

		//
		if (this.reduceStack.size > MAX_DEPTH) {
			throw new Error("Max depth exceeded when reducing");
		}

		this.reduceStack.add(type);

		if (this.reduceCatchers.size) {
			for (const set of this.reduceCatchers) {
				set.add(type);
			}
		}

		try {
			const reduced = type.reduce();

			if (reduced === undefined) {
				throw new Error(
					`The reduce() method for ${this.inspect(type, true)} returned null`,
				);
			}

			if (reduced.getConstructor().type === "OpenT") {
				throw new Error(
					`The reduce() method for ${this.inspect(type, true)} returned an OpenT. This should never be possible. It likely forgot to return utils.reduce() on it.`,
				);
			}

			if (!this.debug) {
				this.reduceCache.set(type, reduced);
			}

			return reduced;
		} finally {
			this.reduceStack.delete(type);
		}
	}

	reduceCatch(
		type: T,
	): {
		final: T;
		involved: Set<T>;
	} {
		const involved: Set<T> = new Set();
		this.reduceCatchers.add(involved);

		const final: T = this.reduce(type);
		this.reduceCatchers.delete(involved);

		return {final, involved};
	}
}
