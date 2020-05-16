/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {ArrayHole} from '@romejs/js-ast';
import {Token} from '../../tokens';

export default function ArrayHole(builder: Builder, node: ArrayHole): Token {
	return builder.tokenizeInnerComments(node, false);
}
