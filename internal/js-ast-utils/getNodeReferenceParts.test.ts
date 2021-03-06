/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {getNodeReferenceParts} from "./getNodeReferenceParts";
import {template} from "./template";

test(
	"getNodeReferenceParts",
	(t) => {
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`foo`),
			'{\n\tbailed: false\n\tparts: [\n\t\t{\n\t\t\tvalue: "foo"\n\t\t\tnode: JSReferenceIdentifier {name: "foo"}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`foo.bar`),
			'{\n\tbailed: false\n\tparts: [\n\t\t{\n\t\t\tvalue: "foo"\n\t\t\tnode: JSReferenceIdentifier {name: "foo"}\n\t\t}\n\t\t{\n\t\t\tvalue: "bar"\n\t\t\tnode: JSIdentifier {name: "bar"}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`this.bar`),
			'{\n\tbailed: false\n\tparts: [\n\t\t{\n\t\t\tvalue: "this"\n\t\t\tnode: JSThisExpression {}\n\t\t}\n\t\t{\n\t\t\tvalue: "bar"\n\t\t\tnode: JSIdentifier {name: "bar"}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`this.bar[bar]`),
			'{\n\tbailed: true\n\tparts: [\n\t\t{\n\t\t\tvalue: "this"\n\t\t\tnode: JSThisExpression {}\n\t\t}\n\t\t{\n\t\t\tvalue: "bar"\n\t\t\tnode: JSIdentifier {name: "bar"}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`import.meta`),
			'{\n\tbailed: false\n\tparts: [\n\t\t{\n\t\t\tvalue: "import"\n\t\t\tnode: JSMetaProperty {\n\t\t\t\tmeta: JSIdentifier {name: "import"}\n\t\t\t\tproperty: JSIdentifier {name: "meta"}\n\t\t\t}\n\t\t}\n\t\t{\n\t\t\tvalue: "meta"\n\t\t\tnode: JSMetaProperty {\n\t\t\t\tmeta: JSIdentifier {name: "import"}\n\t\t\t\tproperty: JSIdentifier {name: "meta"}\n\t\t\t}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`foo['bar']`),
			'{\n\tbailed: false\n\tparts: [\n\t\t{\n\t\t\tvalue: "foo"\n\t\t\tnode: JSReferenceIdentifier {name: "foo"}\n\t\t}\n\t\t{\n\t\t\tvalue: "bar"\n\t\t\tnode: JSStringLiteral {value: "bar"}\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			getNodeReferenceParts(template.expression`foo[bar]`),
			'{\n\tbailed: true\n\tparts: [\n\t\t{\n\t\t\tvalue: "foo"\n\t\t\tnode: JSReferenceIdentifier {name: "foo"}\n\t\t}\n\t]\n}',
		);
	},
);
