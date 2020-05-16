/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {AnyNode, TSConditionalType} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';
import {printConditionalExpression} from '../expressions/ConditionalExpression';

export default function TSConditionalType(
	builder: Builder,
	node: TSConditionalType,
	parent: AnyNode,
): Token {
	return printConditionalExpression(
		concat([
			builder.tokenize(node.checkType, node),
			space,
			'extends',
			space,
			builder.tokenize(node.extendsType, node),
		]),
		builder.tokenize(node.trueType, node),
		builder.tokenize(node.falseType, node),
		parent,
		node.trueType,
		node.falseType,
	);
}
