import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {AnyNode, JSClassHead} from "@internal/ast";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";

// The classnode type
type ClassNode = {
	node: AnyNode;
	order: number;
	name: string;
};
const SortGroups = {
	STATIC_METHODS: 0,
	LIFECYCLE: 1,
	EVERYTHING_ELSE: 2,
	RENDER: 3,
};

// The list of methods and variables that are part of the lifecycle group. Order matters
const lifecycleTypes: {
	[name: string]: {
		type: string;
		order: number;
	};
} = {
	"displayName": {type: "property", order: 0},
	"propTypes": {type: "property", order: 1},
	"contextTypes": {type: "property", order: 2},
	"childContextTypes": {type: "property", order: 3},
	"mixins": {type: "property", order: 4},
	"statics": {type: "property", order: 5},
	"defaultProps": {type: "property", order: 6},
	"constructor": {type: "method", order: 7},
	"getDefaultProps": {type: "method", order: 8},
	"state": {type: "property", order: 9},
	"getInitialState": {type: "method", order: 10},
	"getChildContext": {type: "method", order: 11},
	"getDerivedStateFromProps": {type: "method", order: 12},
	"componentWillMount": {type: "method", order: 13},
	"UNSAFE_componentWillMount": {type: "method", order: 14},
	"componentDidMount": {type: "method", order: 15},
	"componentWillReceiveProps": {type: "method", order: 16},
	"UNSAFE_componentWillReceiveProps": {type: "method", order: 17},
	"shouldComponentUpdate": {type: "method", order: 18},
	"componentWillUpdate": {type: "method", order: 19},
	"UNSAFE_componentWillUpdate": {type: "method", order: 20},
	"getSnapshotBeforeUpdate": {type: "method", order: 21},
	"componentDidUpdate": {type: "method", order: 22},
	"componentDidCatch": {type: "method", order: 23},
	"componentWillUnmount": {type: "method", order: 24},
};

const numberLifecycle = 25;

function isReactComponent(node: JSClassHead): boolean {
	// Check if it extends React.Component or Component, and React.PureCompnent and PureComponent
	return (
		node !== undefined &&
		node.superClass !== undefined &&
		(doesNodeMatchPattern(node.superClass, "React.Component") ||
		doesNodeMatchPattern(node.superClass, "Component") ||
		doesNodeMatchPattern(node.superClass, "React.PureComponent") ||
		doesNodeMatchPattern(node.superClass, "PureComponent"))
	);
}

function isStaticMethod(node: AnyNode): boolean {
	if (node.type === "JSClassMethod") {
		return node.kind === "method" && node.meta.static === true;
	}

	return false;
}

function isLifecycleComponent(node: AnyNode): boolean {
	if (node.type === "JSClassMethod") {
		if (node.key.value.type !== "JSIdentifier") {
			return false;
		}
		const lifecycleName = node.key.value.name;

		// Check that it is a lifecycle method, and that the
		// type is the same (property vs method)
		return (
			lifecycleName in lifecycleTypes &&
			lifecycleTypes[lifecycleName].type === "method"
		);
	} else if (node.type === "JSClassProperty") {
		if (node.key.value.type !== "JSIdentifier") {
			return false;
		}
		const lifecycleName = node.key.value.name;

		// Check that it is a lifecycle method, and that the
		// type is the same (property vs method)
		return (
			lifecycleName in lifecycleTypes &&
			lifecycleTypes[lifecycleName].type === "property"
		);
	}

	return false;
}

function isRenderMethod(node: AnyNode): boolean {
	if (node.type === "JSClassMethod") {
		return (
			node.kind === "method" &&
			node.key.value.type === "JSIdentifier" &&
			node.key.value.name === "render"
		);
	}

	return false;
}

// Takes a node and converts it to a number which corresponds to the order where it should be
function convertNodeToOrder(node: AnyNode): number {
	if (isLifecycleComponent(node)) {
		// This type defines the signature we need to get the name. We know this exists
		// because we test for it in isLifecycleComponent() but typescript doesnt know that
		type JSClassNameSignature = {
			key: {
				value: {
					name: string;
				};
			};
		};
		const lifecycleName: string = (<JSClassNameSignature>node).key.value.name;

		return SortGroups.LIFECYCLE + lifecycleTypes[lifecycleName].order;
	} else if (isRenderMethod(node)) {
		return SortGroups.RENDER + numberLifecycle;
	} else if (isStaticMethod(node)) {
		return SortGroups.STATIC_METHODS;
	} else {
		return SortGroups.EVERYTHING_ELSE + numberLifecycle;
	}
}

function getNodeName(node: AnyNode): string {
	if (
		(node.type === "JSClassMethod" || node.type === "JSClassProperty") &&
		node.key.value !== undefined &&
		node.key.value.type === "JSIdentifier"
	) {
		return node.key.value.name;
	}

	return "";
}

// Solution comes from https://en.wikipedia.org/wiki/Longest_increasing_subsequence
function findOutOfOrderNodes(
	seq: Array<ClassNode>,
): {
	nodesToMove: Array<ClassNode>;
	nodesInOrder: Array<ClassNode>;
} {
	if (seq.length <= 1) {
		return {nodesToMove: [], nodesInOrder: []};
	}

	let longestSubsequencePredecessorIndex = new Array(seq.length);
	let smallestValueIndex = new Array(seq.length + 1);

	let L = 0;
	for (let i = 0; i < seq.length; i++) {
		// Binary search for the largest positive j ≤ L such that X[M[j]] <= X[i]
		let lo = 1;
		let hi = L;
		while (lo <= hi) {
			const mid = Math.ceil((lo + hi) / 2);
			if (seq[smallestValueIndex[mid]].order <= seq[i].order) {
				lo = mid + 1;
			} else {
				hi = mid - 1;
			}
		}
		// After searching, lo is 1 greater than the length of the longest prefix of X[i]
		let newL = lo;

		// The predecessor of X[i] is the last index of the subsequence of length newL-1
		longestSubsequencePredecessorIndex[i] = smallestValueIndex[newL - 1];
		smallestValueIndex[newL] = i;

		if (newL > L) {
			// If we found a subsequence longer than any we've found yet, update L
			L = newL;
		}
	}

	// Reconstruct the inverse of the longest increasing subsequence
	let nodesToMove = Array(Math.max(seq.length - L - 1, 0));
	let nodesInOrder = new Array(L);
	let longestSubsequenceIndex = smallestValueIndex[L];
	let inverseIndex = nodesToMove.length;
	let subsequenceIndex = L - 1;
	// This part was adapted to get the increasing subsequence, but also the rest
	// of the values in the list (i.e. the values the user will be warned of)
	for (let i = seq.length - 1; i > -1; i += -1) {
		if (i === longestSubsequenceIndex) {
			nodesInOrder[subsequenceIndex] = seq[i];
			subsequenceIndex--;
			longestSubsequenceIndex = longestSubsequencePredecessorIndex[longestSubsequenceIndex];
		} else {
			nodesToMove[inverseIndex] = seq[i];
			inverseIndex--;
		}
	}
	return {nodesToMove, nodesInOrder};
}

export default createVisitor({
	name: "react/useSortComp",
	enter(path) {
		const {node} = path;

		// Look for classes and Check if the classes are react components
		if (
			(node.type === "JSClassDeclaration" || node.type === "JSClassExpression") &&
			node.meta !== undefined &&
			isReactComponent(node.meta) &&
			node.meta.body
		) {
			// The list of nodes in the class
			const classNodes = node.meta.body;

			// This is the list of nodes, except it will be showing their expected order.
			// Any order values that are not in order will be the ones we need to warn the
			// user about. We just find the position where the order should be and we know
			// which 2 nodes to tell the user about
			const orderList = classNodes.map((classNode) => {
				return {
					node: classNode,
					order: convertNodeToOrder(classNode),
					name: getNodeName(classNode),
				};
			});

			// Find longest increasing subsequence to find the elements we don't want to move
			// as well as the elements we want to move
			const {nodesToMove, nodesInOrder} = findOutOfOrderNodes(orderList);

			// sort the nodesToMove based on order so we can more easily find where to put them
			nodesToMove.sort((a, b) => {
				return a.order - b.order;
			});

			// For each element to move, figure out where to move it to
			let toMoveIndex = 0;
			let inOrderIndex = 0;
			for (let i = 0; toMoveIndex < nodesToMove.length; i++) {
				const toMove = nodesToMove[toMoveIndex];
				const inOrder = nodesInOrder[inOrderIndex];

				// If nodeToMove can be placed before nodeInOrder or nodeInOrder is at the end of the list
				if (
					toMove.order < inOrder.order ||
					inOrderIndex === nodesInOrder.length - 1
				) {
					let position: "before" | "after" = "before";
					// If its the last one in the in order list
					if (inOrderIndex === nodesInOrder.length - 1) {
						// Add after error
						position = "after";
					}

					let errorNode = toMove.node;

					// Make the warning message point to the function name
					if (
						toMove.node.type === "JSClassMethod" ||
						toMove.node.type === "JSClassProperty"
					) {
						errorNode = toMove.node.key;
					}

					// Add diagnostics to the node
					path.context.addNodeDiagnostic(
						errorNode,
						descriptions.LINT.REACT_USE_SORT_COMP(
							inOrder.name,
							toMove.name,
							position,
						),
					);

					toMoveIndex++;
				} else {
					inOrderIndex++;
				}
			}
		}

		return signals.retain;
	},
});
