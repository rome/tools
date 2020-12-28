import {test} from "rome";
import {template} from "@internal/js-ast-utils/template";

test(
	"verify template generation",
	(t) => {
		t.inlineSnapshot(
			template.expression`foo.bar`,
			'JSMemberExpression {\n\tobject: JSReferenceIdentifier {name: "foo"}\n\tproperty: JSStaticMemberProperty {value: JSIdentifier {name: "bar"}}\n}',
		);

		t.inlineSnapshot(
			template.statement`const hello = world`,
			'JSVariableDeclarationStatement {\n\tdeclaration: JSVariableDeclaration {\n\t\tkind: "const"\n\t\tdeclarations: Array [\n\t\t\tJSVariableDeclarator {\n\t\t\t\tid: JSBindingIdentifier {name: "hello"}\n\t\t\t\tinit: JSReferenceIdentifier {name: "world"}\n\t\t\t}\n\t\t]\n\t}\n}',
		);
	},
);
