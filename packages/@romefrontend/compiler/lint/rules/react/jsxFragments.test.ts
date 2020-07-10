import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"react jsx fragments",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<Fragment><Foo /><Foo /></Fragment>",
					"<React.Fragment><Foo /><Foo /></React.Fragment>",
					"const Hello = <div><Fragment><Foo /><Foo /></Fragment></div>",
					"const Hello = <div><React.Fragment><Foo /><Foo /></React.Fragment></div>",
					"const Hello = <React.Fragment><Foo /><Foo /></React.Fragment>",
					"const Hello = <Fragment><Foo /><Foo /></Fragment>",
					dedent`
					function Foo() {
						let bar = <React.Fragment><Foo /><Foo /></React.Fragment>;
						return bar;
					}
					`,
					dedent`
					function Foo() {
						let bar = <Fragment><Foo /><Foo /></Fragment>;
						return bar;
					}
					`,
					dedent`
					function Hello() {
						return <React.Fragment><Foo /><Foo /></React.Fragment>
					}
					`,
					dedent`function Hello() {
						return <Fragment><Foo /><Foo /></Fragment>
					}
					`,
					"const Hello = () => <React.Fragment><Foo /><Foo /></React.Fragment>",
					"const Hello = () => <Fragment><Foo /><Foo /></Fragment>",
				],
				valid: [
					"<><Foo /><Foo /></>",

					"<Fragment key='id'></Fragment>",
					"<React.Fragment key='id'><Foo /></React.Fragment>",
					"const Hello = <div><Fragment key='word'><Foo /></Fragment></div>",
					"const Hello = <>hello</>",
					"const Hello = <Fragment key='id'><Foo/></Fragment>",
					"const Hello = <React.Fragment key='id'><Foo/></React.Fragment>",
					dedent`
					function Foo() {
						let bar = <React.Fragment key='word'></React.Fragment>;
						return bar;
					}
					`,
					dedent`
					function Hello() {
						return <React.Fragment key='id'><Foo /></React.Fragment>
					}
					`,
					dedent`
					function Hello() {
						return <Fragment key='id'><Foo /></Fragment>
					}
					`,
					"const Hello = () => <></>",
					"const Hello = () => <React.Fragment key='id'></React.Fragment>",
					"const Hello = () => <Fragment key='id'></Fragment>",
				],
			},
			{category: "lint/react/jsxFragments"},
		);
	},
);
