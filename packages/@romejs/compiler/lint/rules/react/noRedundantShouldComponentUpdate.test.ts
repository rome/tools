import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no redundant should component update",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.PureComponent {
          componentShouldUpdate() {}
        }
				`,
				`
        class Hello extends PureComponent {
          componentShouldUpdate() {}
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
