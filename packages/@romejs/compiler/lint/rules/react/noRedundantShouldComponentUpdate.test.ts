import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react no redundant should component update",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.PureComponent {
          shouldComponentUpdate() {}
        }
				`,
				`
        class Hello extends PureComponent {
          shouldComponentUpdate() {}
        }
        `,
				// VALID
				`
        class Hello extends React.PureComponent {
          componentDidMount() {}
        }
        `,
				`
        class Hello extends PureComponent {
          componentDidMount() {}
        }
        `,
			],
			{category: "lint/react/noRedundantShouldComponentUpdate"},
		);
	},
);
