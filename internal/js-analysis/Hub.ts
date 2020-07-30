/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import T from "./types/T";
import {CompilerContext, TransformProjectDefinition} from "@internal/compiler";
import {JSRoot} from "@internal/ast";
import Graph from "./Graph";
import Evaluator from "./Evaluator";
import Utils from "./Utils";

const statuses = {
	OPEN: 0,
	CLOSING: 1,
	CLOSED: 2,
};

type HubStatus = number;

export default class Hub {
	constructor(ast: JSRoot, project: TransformProjectDefinition) {
		this.context = new CompilerContext({
			ast,
			project,
			origin: {
				category: "typeChecking",
			},
		});
		this.utils = new Utils(this);
		this.graph = new Graph();
		this.evaluator = new Evaluator(this, ast.filename);
		this.status = statuses.OPEN;
	}

	status: HubStatus;
	evaluator: Evaluator;
	graph: Graph<T>;
	context: CompilerContext;
	utils: Utils;

	close() {
		this.status = statuses.CLOSING;

		for (const [node] of this.graph.nodesByValue) {
			this.utils.reduce(node);
		}

		this.status = statuses.CLOSED;
	}

	isClosing(): boolean {
		return this.status === statuses.CLOSING;
	}

	isOpen(): boolean {
		return this.isClosing() || this.status === statuses.OPEN;
	}

	isClosed(): boolean {
		return this.isClosing() || this.status === statuses.CLOSED;
	}

	assertOpen() {
		if (this.isClosed() && this.isClosing() === false) {
			throw new Error(
				"This method can only be called when the graph has been open",
			);
		}
	}

	assertClosed() {
		if (this.isOpen() && this.isClosing() === false) {
			throw new Error(
				"This method can only be called when the graph has been closed",
			);
		}
	}
}
