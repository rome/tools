import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react no useless fragment",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<>{foo}</>",
					"<p><>foo</></p>",
					"<></>",
					"<React.Fragment>foo</React.Fragment>",
					"<Fragment>foo</Fragment>",
					`<section>
					<>
						<div />
						<div />
					</>
				</section>
				`,
				],
				valid: [
					`
				<>
					<Foo />
					<Bar />
				</>
				`,
					"<>foo {bar}</>",
					"<> {foo}</>",
					"const cat = <>meow</>",
					"const cat = () => <>meow</>",
					`
				function cat() {
					return <>meow</>;
				}
				`,
					`
				function cat() {
					const foo = <>meow</>;
					return foo;
				}
				`,
					`
				<SomeComponent>
					<>
						<div />
						<div />
					</>
				</SomeComponent>
				`,
					"<Fragment key={item.id}>{item.value}</Fragment>",
				],
			},
			{category: "lint/react/noUselessFragment"},
		);
	},
);
