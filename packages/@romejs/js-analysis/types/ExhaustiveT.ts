/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {Scope} from "../scopes";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory} from "./T";
import {HumanBuilder} from "../Utils";
import E, {ErrorDefinition} from "./errors/E";
import AnyT from "./AnyT";
import {descriptions} from "@romejs/diagnostics";

class ENotExhaustive extends E {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		target: T,
		only: T,
		extraenous: Array<T>,
	) {
		super(scope, originNode);
		this.target = target;
		this.only = only;
		this.extraenous = extraenous;
	}

	target: T;
	only: T;
	extraenous: Array<T>;

	static type = "ENotExhaustive";

	getError(): ErrorDefinition {
		return {
			description: descriptions.TYPE_CHECK.NOT_EXHAUSTIVE(
				this.utils.humanize(this.only),
				this.utils.humanize(this.target),
			),
			lowerTarget: this.target,
		};
	}
}

export default class ExhaustiveT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, target: T, only: T) {
		super(scope, originNode);
		this.target = target;
		this.only = only;
	}

	target: T;
	only: T;

	static type = "ExhaustiveT";

	serialize(addType: SerialTypeFactory): HydrateData {
		return {
			target: addType(this.target),
			only: addType(this.only),
		};
	}

	static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new ExhaustiveT(
			scope,
			originNode,
			getType(data.target),
			getType(data.only),
		);
	}

	reduce(): T {
		const target = this.utils.reduce(this.target);
		const only = this.utils.reduce(this.only);
		if (target instanceof AnyT || only instanceof AnyT) {
			return this.only;
		}

		const targetCandidates = this.utils.explodeUnion(target);
		const onlyCandidates = this.utils.explodeUnion(only);

		const extraneous = [];
		for (const possible of targetCandidates) {
			let compatible = false;

			for (const otherType of onlyCandidates) {
				if (this.utils.isCompatibleWith(possible, otherType)) {
					compatible = true;
				}
			}

			if (compatible === false) {
				extraneous.push(possible);
			}
		}

		if (extraneous.length === 0) {
			return target;
		} else {
			return new ENotExhaustive(
				this.scope,
				this.originNode,
				this.target,
				this.only,
				extraneous,
			);
		}
	}

	humanize(builder: HumanBuilder): string {
		return `exhaustive ${builder.humanize(this.target)} should only match ${builder.humanize(
			this.target,
		)}`;
	}
}
