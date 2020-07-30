/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CheckProvider} from "../types";
import {
	DiagnosticAdvice,
	Diagnostics,
	descriptions,
} from "@internal/diagnostics";
import {JSRoot} from "@internal/ast";
import Hub from "../Hub";
import E from "../types/errors/E";
import T from "../types/T";
import OpenT from "../types/OpenT";
import buildGraph from "./buildGraph";
import {TransformProjectDefinition} from "@internal/compiler";

export default async function check(
	opts: {
		ast: JSRoot;
		project: TransformProjectDefinition;
		provider: CheckProvider;
	},
): Promise<Diagnostics> {
	const hub = await buildGraph({
		ast: opts.ast,
		connected: true,
		provider: opts.provider,
		project: opts.project,
	});
	resolveGraph(hub);
	return hub.context.diagnostics.getDiagnostics();
}

function isError(t: undefined | T): boolean {
	return t !== undefined && t instanceof E;
}

function resolveGraph(hub: Hub): Diagnostics {
	const {graph, utils, context} = hub;

	// we track caught errors here as if a normal type returns a error in it's reduce() method

	// then it will be added to the graph, however we'd have already dealt with it
	const caughtErrors: Set<T> = new Set();

	for (const node of graph.nodes) {
		const lower = node.value;

		// unconnected node, we'll resolve these if they've been connected to any nodes
		if (lower instanceof OpenT) {
			continue;
		}

		// see if this reduces to a type error
		const reduced = utils.reduce(lower);
		if (reduced instanceof E) {
			if (caughtErrors.has(reduced)) {
				continue;
			} else {
				caughtErrors.add(reduced);
			}

			let {description, lowerTarget, upperTarget} = reduced.getError();

			// ignore errors inside
			if (isError(lowerTarget) || isError(upperTarget)) {
				continue;
			}

			let advice: DiagnosticAdvice = [];

			if (upperTarget !== undefined) {
				const marker =
					upperTarget && !(upperTarget instanceof reduced.constructor)
						? utils.humanize(upperTarget)
						: undefined;
				const {originLoc} = upperTarget;

				if (originLoc !== undefined && marker !== undefined) {
					advice.push({
						type: "log",
						category: "info",
						text: marker,
					});
				} else if (originLoc !== undefined) {
					advice.push({
						type: "frame",
						location: {
							filename: originLoc.filename,
							start: originLoc.start,
							end: originLoc.end,
							marker,
						},
					});
				}
			}

			description = {
				...description,
				advice: [...advice, ...description.advice],
			};

			context.addNodeDiagnostic(
				lowerTarget.originNode,
				description,
				{
					marker: lowerTarget && !(lowerTarget instanceof reduced.constructor)
						? utils.humanize(lowerTarget)
						: undefined,
				},
			);
			continue;
		}

		// ignore unconnected nodes
		if (node.lines.length === 0) {
			continue;
		}

		for (const line of node.lines) {
			const upper = line.value;
			const compatibility = utils.checkCompability(upper, lower);

			if (compatibility.type === "incompatible") {
				// ignore associated errors, as they've already been handled
				if (isError(compatibility.lower) || isError(compatibility.upper)) {
					continue;
				}

				context.addNodeDiagnostic(
					compatibility.lower.originNode,
					descriptions.TYPE_CHECK.INCOMPATIBILITY(
						utils.humanize(upper),
						upper.originLoc,
					),
					{
						marker: utils.humanize(compatibility.lower),
					},
				);
			}
		}
	}

	return context.diagnostics.getDiagnostics();
}
