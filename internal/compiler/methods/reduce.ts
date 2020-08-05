/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyVisitors,
	CompilerContext,
	ExitSignal,
	Path,
	PathOptions,
	signals,
} from "@internal/compiler";
import {AnyNode, AnyNodes, visitorKeys as allVisitorKeys} from "@internal/ast";
import {isNodeLike} from "@internal/js-ast-utils";
import {AnyVisitor} from "../types";
import {pretty} from "@internal/pretty-format";
import {
	EnterSignal,
	ParentSignal,
	RemoveSignal,
	ReplaceSignal,
	RetainSignal,
} from "../signals";
import {AnyVisitorState} from "../lib/VisitorState";

/**
 * Validate the return value of an enter or exit transform
 */
function validateSignal(transformName: string, signal: ExitSignal, path: Path) {
	// Verify common mistake of forgetting to return something
	if (typeof signal === "undefined") {
		throw new Error(
			"Returned `undefined` from transform " +
			transformName +
			". If you meant to delete this node then use `return" +
			" REDUCE_REMOVE`, otherwise if you want to keep it then use `return path.node;`",
		);
	}

	// Ignore some constants that will be handled later
	if (signal.type === "REMOVE") {
		return;
	}

	// Handle returning an array of nodes
	if (signal.type === "REPLACE" && Array.isArray(signal.value)) {
		// keyed nodes cannot be replaced with an array of nodes
		if (path.opts.noArrays === true) {
			throw new Error(
				`Cannot replace this keyed node ${path.parent.type}[${path.opts.nodeKey}] with an array of nodes - originated from transform ${transformName}`,
			);
		}
		return;
	}

	// Verify that it's a valid node
	if (signal.type === "REPLACE" && !isNodeLike(signal.value)) {
		throw new Error(
			`Expected a return value of a plain object with a \`type\` property or a reduce constant - originated from 'transform ${transformName}`,
		);
	}
}

// Consider a replace signal with the same value as the path to be a retain signal
// Many reasons we could emit a replace when we mean a retain just by nature of
// passing nodes around.
function isRetainSignal(
	node: AnyNode,
	signal: ExitSignal,
): signal is RetainSignal {
	switch (signal.type) {
		case "RETAIN":
			return true;

		case "REPLACE":
			return node === signal.value;

		default:
			return false;
	}
}

function maybeFork(path: Path, signal: ReplaceSignal | RetainSignal): Path {
	if (isRetainSignal(path.node, signal)) {
		return path;
	} else {
		const {value} = signal;
		if (Array.isArray(value)) {
			throw new Error(
				"Should have already refined away a replace of Arrays with shouldBailReduce",
			);
		}
		return path.fork(value);
	}
}

// Process a parent signal. If it refers to this node, it's a replacement, otherwise bubble it up.
function normalizeParentSignalReturn(
	node: AnyNode,
	signal: ParentSignal,
): ExitSignal {
	if (signal.parent === node) {
		return signal.signal;
	} else {
		return signal;
	}
}

/**
 * Given a return value from a transform, determine if we should bail out.
 * Bailing out means returning the actual signal and making the parent reduce
 * call handle it (if any).
 */
function shouldBailReduce(
	signal: EnterSignal,
): signal is RemoveSignal | ParentSignal {
	if (signal.type === "REPLACE" && Array.isArray(signal.value)) {
		// We just return the array of nodes, without transforming them
		// reduce() calls higher in the chain will splice this array and do it's
		// own transform call so when the transform is performed on the node it's
		// in it's correct place in the tree
		return true;
	}

	// This node is being removed, no point recursing into it
	if (signal.type === "REMOVE") {
		return true;
	}

	// Bail on parent signals. We'll be handled higher in the tree.
	if (signal.type === "PARENT") {
		return true;
	}

	return false;
}

export function reduceNode(
	ast: AnyNode,
	visitors: AnyVisitor | AnyVisitors,
	context: CompilerContext,
	pathOpts: PathOptions = {},
): AnyNodes {
	const res = _reduceSignal(
		ast,
		Array.isArray(visitors) ? visitors : [visitors],
		context,
		pathOpts,
	);

	switch (res.type) {
		case "REMOVE":
			throw new Error(
				pretty`reduceEntry: Invalid symbol returned from reduceChild. Result: ${res}`,
			);

		case "PARENT":
			throw new Error(
				pretty`reduceEntry: Invalid parent signal returned from reduceChild. Parent was not in the tree. Result: ${res}`,
			);

		case "RETAIN":
			return ast;

		case "REPLACE":
			return res.value;
	}
}

export function reduceSignal(
	ast: AnyNode,
	visitors: AnyVisitor | AnyVisitors,
	context: CompilerContext,
	pathOpts: PathOptions = {},
): ExitSignal {
	return _reduceSignal(
		ast,
		Array.isArray(visitors) ? visitors : [visitors],
		context,
		pathOpts,
	);
}

type PopState = Set<AnyVisitorState>;

// This method is pretty gnarly and deeply nested but is very important from a performance perspective
function _reduceSignal(
	origNode: AnyNode,
	visitors: AnyVisitors,
	context: CompilerContext,
	pathOpts: PathOptions,
): ExitSignal {
	// Initialize first path
	let path: Path = new Path(origNode, context, pathOpts);

	const popState: PopState = new Set();

	try {
		// Perform enter transforms
		for (const visitor of visitors) {
			const {enter} = visitor;
			if (enter === undefined) {
				continue;
			}

			// Fetch state
			const state = context.getVisitorState(visitor);
			state.setCurrentPath(path);

			// Call transformer
			let signal = enter(path, state);
			if (state.checkPushed()) {
				// If we inserted new state then remember to pop it off when we're done
				popState.add(state);
			}

			if (!path.context.frozen) {
				// When returning this symbol, it indicates we should skip the subtree
				if (signal.type === "SKIP") {
					return signals.retain;
				}

				// Validate the return value
				validateSignal(visitor.name, signal, path);

				// Check if we need to bail out. See the comment for shouldBailReduce on what that means
				if (shouldBailReduce(signal)) {
					return signal;
				}

				// Create new path if node has been changed
				path = maybeFork(path, signal);
			}
		}

		// Reduce the children
		let {node} = path;
		const visitorKeys = allVisitorKeys.get(node.type);
		if (visitorKeys !== undefined) {
			// Build the ancestry paths that we'll pass to each child path
			const ancestryPaths = pathOpts.ancestryPaths || [];
			let childAncestryPaths: Array<Path> = [path].concat(ancestryPaths);

			// Reduce the children
			for (const key of visitorKeys) {
				// rome-ignore lint/ts/noExplicitAny
				const oldVal = (node as any)[key];

				if (Array.isArray(oldVal)) {
					let children: Array<AnyNode> = oldVal;

					// When removing items from the children array, we decrement this offset and subtract it
					// whenever looking up to get the correct position
					let childrenOffset = 0;

					// This needs to be calculated beforehand as the length of the array may change when removing
					// items
					let length = children.length;

					for (let i = 0; i < length; i++) {
						// Calculate the correct index that this children can be found at
						const correctedIndex = childrenOffset + i;

						// Get the child
						const child = children[correctedIndex];

						// An array may be mixed containing [undefined, Node] etc so check that it's actually a valid node
						// An example of a property with empty elements is an JSArrayExpression with holes
						if (isNodeLike(child)) {
							// Run transforms on this node
							const newSignal = _reduceSignal(
								child,
								visitors,
								context,
								{
									noScopeCreation: pathOpts.noScopeCreation,
									parentScope: path.scope,
									ancestryPaths: childAncestryPaths,
									listKey: correctedIndex,
									nodeKey: key,
								},
							);

							if (newSignal.type === "PARENT") {
								return normalizeParentSignalReturn(node, newSignal);
							}

							// If this item has been changed then...
							if (!isRetainSignal(child, newSignal) && !context.frozen) {
								// Clone the children array
								children = children.slice();

								// Check if the item is to be deleted
								// REDUCE_REMOVE or an empty array are considered equivalent
								if (
									newSignal.type === "REMOVE" ||
									(Array.isArray(newSignal.value) &&
									newSignal.value.length === 0)
								) {
									// Remove the item from the array
									children.splice(correctedIndex, 1);

									// Since the array now has one less item, change the offset so all
									// future indices will be correct
									childrenOffset--;
								} else if (Array.isArray(newSignal.value)) {
									// Remove the previous, and add the new items to the array
									children.splice(correctedIndex, 1, ...newSignal.value);

									// We increase the length of the array so that this loop covers
									// the newly inserted nodes
									// `childrenOffset` is not used here because that's just used to
									// skip elements
									length += newSignal.value.length;

									// Revisit the current index, this is necessary as there's now a
									// new node at this position
									i--;
								} else {
									// Otherwise it's a valid node so set it
									children[correctedIndex] = newSignal.value;

									// Revisit the current index, the node has changed and some
									// transforms may care about it
									i--;
								}

								// Mutate the original node - funky typing since Flow doesn't understand the mutation
								node = ({...node, [key]: children} as AnyNode);

								// Create a new node path
								path = path.fork(node);

								// And create a new ancestry array for subsequent children
								childAncestryPaths = [path].concat(ancestryPaths);
							}
						}
					}
				} else if (isNodeLike(oldVal)) {
					// Run transforms on this node
					let newSignal: undefined | ExitSignal = _reduceSignal(
						oldVal,
						visitors,
						context,
						{
							noScopeCreation: pathOpts.noScopeCreation,
							parentScope: path.scope,
							ancestryPaths: childAncestryPaths,
							noArrays: true,
							nodeKey: key,
						},
					);

					if (newSignal.type === "PARENT") {
						return normalizeParentSignalReturn(node, newSignal);
					}

					// If this value has been changed then...
					if (!isRetainSignal(oldVal, newSignal) && !context.frozen) {
						let newValue = undefined;
						if (newSignal.type === "REPLACE") {
							newValue = newSignal.value;
						} else if (newSignal.type === "REMOVE") {
							// If the node is deleted then use `undefined` instead
							newValue = undefined;
						}

						// When replacing a key value, we cannot replace it with an array
						if (Array.isArray(newValue)) {
							throw new Error(
								"Cannot replace a key value node with an array of nodes",
							);
						}

						// Mutate the original object - funky typing since Flow doesn't understand the mutation
						node = ({...node, [key]: newValue} as AnyNode);

						// Create a new node path for it
						path = path.fork(node);

						// And create a new ancestry array for subsequent children
						childAncestryPaths = [path].concat(ancestryPaths);
					}
				} else {
					// not sure what this is...
				}
			}
		}

		// Run exit visitors
		for (const visitor of visitors) {
			if (visitor.exit === undefined) {
				continue;
			}

			const state = context.getVisitorState(visitor);
			state.setCurrentPath(path);

			const signal = visitor.exit(path, state);

			if (!path.context.frozen) {
				validateSignal(visitor.name, signal, path);

				if (shouldBailReduce(signal)) {
					return signal;
				} else {
					path = maybeFork(path, signal);
				}
			}
		}

		if (context.frozen) {
			return signals.retain;
		} else {
			return signals.maybeReplace(origNode, path.node);
		}
	} finally {
		for (const state of popState) {
			state.pop();
		}
	}
}
