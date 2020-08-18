/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Evaluator, {HydrateData, HydrateTypeFactory} from "../Evaluator";
import {SourceLocation} from "@internal/parser-core";
import {AnyNode} from "@internal/ast";
import Graph from "../Graph";
import {Scope} from "../scopes";
import Hub from "../Hub";
import Utils, {HumanBuilder} from "../Utils";
import {StaticMarkup} from "@internal/markup";
import {ExtendedMap} from "@internal/collections";

let counter = 0;

export type SerialTypeFactory = (type: T) => string;

export type TypeCompatibilityReturn =
	| {
			type: "compatible";
		}
	| {
			type: "incompatible";
			lower: T;
			upper: T;
		};

export default class T {
	constructor(scope: Scope, originNode: undefined | AnyNode) {
		this.human = undefined;
		this.scope = scope;

		const {hub} = scope;
		this.hub = hub;
		this.utils = hub.utils;
		this.evaluator = hub.evaluator;
		this.originEvaluator = scope.evaluator.evaluatingType;

		// setup graph
		this.graph = scope.evaluator.graph;
		this.graph.addNode(this);

		this.originNode = originNode;
		this.originLoc = originNode === undefined ? undefined : originNode.loc;
		this.id = `${String(process.pid)}:${String(counter++)}`;

		this.compatibilityCache = new Map();
	}

	public static type = "T";
	protected utils: Utils;
	protected evaluator: Evaluator;
	protected graph: Graph<T>;
	protected hub: Hub;
	public scope: Scope;

	public compatibilityCache: Map<T, TypeCompatibilityReturn>;

	public human: undefined | StaticMarkup;
	public id: string;

	public originNode: undefined | AnyNode;
	public originLoc: undefined | SourceLocation;
	public originEvaluator: undefined | string;

	public getConstructor(): typeof T {
		// @ts-ignore
		return this.constructor;
	}

	public setHuman(human: undefined | StaticMarkup) {
		this.human = human;
	}

	public shouldMatch(type: T) {
		this.hub.assertOpen();
		this.graph.addLine(this, type);
	}

	public hasConnections(): boolean {
		return this.graph.hasConnections(this);
	}

	public explodeUnion(): Array<T> {
		return [this];
	}

	public compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		return otherType instanceof this.constructor;
	}

	public clone() {
		const idsToType: ExtendedMap<string, T> = new ExtendedMap("idsToType");

		const addType: SerialTypeFactory = (type: T) => {
			const reduced = this.utils.reduce(type);
			idsToType.set(type.id, type);
			return reduced.id;
		};

		const data = this.serialize(addType);

		const getType: HydrateTypeFactory = (id: unknown): T => {
			if (typeof id !== "string") {
				throw new Error("Expected id to be a string");
			}

			return idsToType.assert(id);
		};

		return this.getConstructor().hydrate(
			this.scope,
			this.originNode,
			data,
			getType,
		);
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		throw new Error(`Unimplemented ${this.type}.hydrate`);
	}

	public serialize(addType: SerialTypeFactory): HydrateData {
		throw new Error(
			`Unimplemented ${this.getConstructor().type}.prototype.serialize`,
		);
	}

	public reduce(): T {
		return this;
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		const reduced = this.utils.reduce(this);
		if (reduced === this) {
			throw new Error("unimplemented");
		} else {
			return builder.humanize(reduced);
		}
	}

	public inspect() {
		return this.utils.inspect(this);
	}
}
