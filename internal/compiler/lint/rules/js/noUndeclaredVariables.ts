/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

const NODE_VARIABLES_SET = new Set([
	"require",
	"__dirname",
	"__filename",
	"module",
	"exports",
]);

const BROWSER_VARIABLES_SET = new Set([
	"fetch",
	"document",
	"window",
	"Worker",
	"cancelAnimationFrame",
	"requestAnimationFrame",
	"WebSocket",
	"alert",
	"Blob",
	"navigator",
	"Element",
	"Text",
	"Document",
	"performance",
]);

// This is gross...
const TS_VARIABLES_SET = new Set([
	"AggregateErrorConstructor",
	"ArrayBufferConstructor",
	"ArrayBufferLike",
	"ArrayBufferTypes",
	"ArrayBufferView",
	"ArrayConstructor",
	"ArrayLike",
	"AsyncGenerator",
	"AsyncGeneratorFunction",
	"AsyncGeneratorFunctionConstructor",
	"AsyncIterable",
	"AsyncIterableIterator",
	"AsyncIterator",
	"Atomics",
	"BigInt64ArrayConstructor",
	"BigIntConstructor",
	"BigIntToLocaleStringOptions",
	"BigUint64ArrayConstructor",
	"BooleanConstructor",
	"BufferEncoding",
	"BufferSource",
	"CallableFunction",
	"Capitalize",
	"ClassDecorator",
	"ConcatArray",
	"Console",
	"ConstructorParameters",
	"DataViewConstructor",
	"DateConstructor",
	"ErrorConstructor",
	"EvalErrorConstructor",
	"Exclude",
	"Extract",
	"FinalizationRegistryConstructor",
	"FlatArray",
	"Float32ArrayConstructor",
	"Float64ArrayConstructor",
	"FunctionConstructor",
	"Generator",
	"GeneratorFunction",
	"GeneratorFunctionConstructor",
	"IArguments",
	"ImportMeta",
	"InstanceType",
	"Int16ArrayConstructor",
	"Int32ArrayConstructor",
	"Int8ArrayConstructor",
	"Iterable",
	"IterableIterator",
	"Iterator",
	"IteratorResult",
	"IteratorReturnResult",
	"IteratorYieldResult",
	"Lowercase",
	"MapConstructor",
	"MethodDecorator",
	"NewableFunction",
	"NodeJS",
	"NodeRequire",
	"NonNullable",
	"NumberConstructor",
	"ObjectConstructor",
	"Omit",
	"OmitThisParameter",
	"ParameterDecorator",
	"Parameters",
	"Partial",
	"Pick",
	"PromiseConstructor",
	"PromiseConstructorLike",
	"PromiseFulfilledResult",
	"PromiseLike",
	"PromiseRejectedResult",
	"PromiseSettledResult",
	"PropertyDecorator",
	"PropertyDescriptor",
	"PropertyDescriptorMap",
	"PropertyKey",
	"ProxyConstructor",
	"ProxyHandler",
	"RangeErrorConstructor",
	"Readonly",
	"ReadonlyArray",
	"ReadonlyMap",
	"ReadonlySet",
	"Record",
	"ReferenceErrorConstructor",
	"RegExpConstructor",
	"RegExpExecArray",
	"RegExpMatchArray",
	"RequestInit",
	"Required",
	"ReturnType",
	"SetConstructor",
	"SharedArrayBufferConstructor",
	"StringConstructor",
	"SymbolConstructor",
	"SyntaxErrorConstructor",
	"TemplateStringsArray",
	"Thenable",
	"ThisParameterType",
	"ThisType",
	"TypeErrorConstructor",
	"TypedPropertyDescriptor",
	"URIErrorConstructor",
	"Uint16ArrayConstructor",
	"Uint32ArrayConstructor",
	"Uint8ArrayConstructor",
	"Uint8ClampedArrayConstructor",
	"Uncapitalize",
	"Uppercase",
	"WeakMapConstructor",
	"WeakRefConstructor",
	"WeakSetConstructor",
]);

export default createVisitor({
	name: "js/noUndeclaredVariables",
	enter(path) {
		const {node, scope} = path;

		if (
			node.type === "JSReferenceIdentifier" ||
			node.type === "JSXReferenceIdentifier"
		) {
			const {name} = node;
			const binding = scope.getBinding(name);

			const isDefined =
				binding !== undefined ||
				scope.isGlobal(name) ||
				BROWSER_VARIABLES_SET.has(name) ||
				NODE_VARIABLES_SET.has(name) ||
				TS_VARIABLES_SET.has(name);

			if (!isDefined) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_UNDECLARED_VARIABLES(
						name,
						scope.getBindingNames(),
					),
				);
			}
		}

		return signals.retain;
	},
});
