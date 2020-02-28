/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createMessageFactory} from '@romejs/messages';

export default createMessageFactory({
  SINGLE_QUOTE_USAGE: 'You can only use double quoted strings',
  TRAILING_COMMA_VALUE: 'Trailing comma is only allowed after a value',
  UNCLOSED_STRING: 'Unclosed string',
  UNCLOSED_BLOCK_COMMENT: 'Unclosed block comment',
  MISTAKEN_ARRAY_IDENTITY:
    'Trying to use an array element as an object property. Did you mean to make an object?',
  REDUNDANT_COMMA: 'Redundant comma',

  EMPTY_INPUT_IN_JSON: 'Empty input',
  PROPERTY_KEY_UNQUOTED_IN_JSON: 'Property keys must be quoted in JSON',
  IMPLICIT_OBJECT_IN_JSON: 'Objects must be wrapped in curly braces in JSON',
  COMMENTS_IN_JSON: "Comments aren't allowed in JSON",
  TRAILING_COMMA_IN_JSON: "Trailing commas aren't allowed in JSON",
  REGEX_IN_JSON: "Regular expressions aren't allowed in JSON",
  UNKNOWN_WORD_IN_JSON: "$0 isn't a valid JSON word",
  STRING_NEWLINES_IN_JSON:
    'Newlines aren\'t allowed in JSON, you insert a newline by escaping it like this "\\n"',
  UNDEFINED_IN_JSON:
    "undefined isn't allowed in JSON, you could use null instead",
  BIGINT_IN_JSON: "Bigints aren't allowed in JSON",
  NUMERIC_SEPARATORS_IN_JSON: 'Numeric separators are not allowed in JSON',
});
