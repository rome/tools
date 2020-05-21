import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react jsx fragments",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<Fragment></Fragment>",
				"<React.Fragment></React.Fragment>",
				"<Fragment><Foo /></Fragment>",
				"<React.Fragment><Foo /></React.Fragment>",
				"const Hello = <div><Fragment><Foo/></Fragment></div>",
				"const Hello = <div><React.Fragment><Foo/></React.Fragment></div>",
				"const Hello = <React.Fragment><Foo/></React.Fragment>",
				"const Hello = <Fragment><Foo/></Fragment>",
				`
				function Foo() {
					let bar = <React.Fragment><Foo /></React.Fragment>;
					return bar;
				}	
				`,
				`
				function Foo() {
					let bar = <Fragment><Foo/></Fragment>;
					return bar;
				}	
				`,
				`
				function Hello() {
					return <React.Fragment><Foo /></React.Fragment>
				}
				`,
				`function Hello() {
					return <Fragment><Foo /></Fragment>
				}
				`,
				"const Hello = () => <React.Fragment></React.Fragment>",
				"const Hello = () => <Fragment></Fragment>",
				// VALID
				"<></>",
				"<><Foo /></>",
				"<Fragment key='id'></Fragment>",
				"<React.Fragment key='id'><Foo /></React.Fragment>",
				"const Hello = <div><Fragment key='word'><Foo /></Fragment></div>",
				"const Hello = <>hello</>",
				"const Hello = <Fragment key='id'><Foo/></Fragment>",
				"const Hello = <React.Fragment key='id'><Foo/></React.Fragment>",
				`
				function Foo() {
					let bar = <React.Fragment key='word'></React.Fragment>;
					return bar;
				}	
				`,
				`
				function Hello() {
					return <React.Fragment key='id'><Foo /></React.Fragment>
				}
				`,
				`function Hello() {
					return <Fragment key='id'><Foo /></Fragment>
				}
				`,
				"const Hello = () => <></>",
				"const Hello = () => <React.Fragment key='id'></React.Fragment>",
				"const Hello = () => <Fragment key='id'></Fragment>",
			],
			{category: "lint/react/jsxFragments"},
		);
	},
);
