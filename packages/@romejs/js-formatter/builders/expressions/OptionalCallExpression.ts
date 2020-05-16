/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {OptionalCallExpression} from '@romejs/js-ast';
import CallExpression from './CallExpression';

export default function OptionalCallExpression(
	builder: Builder,
	node: OptionalCallExpression,
): Token {
	return CallExpression(builder, node);
}
