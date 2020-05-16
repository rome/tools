/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {PrivateName} from '@romejs/js-ast';
import {Token, concat} from '../../tokens';

export default function PrivateName(builder: Builder, node: PrivateName): Token {
	return concat(['#', builder.tokenize(node.id, node)]);
}
