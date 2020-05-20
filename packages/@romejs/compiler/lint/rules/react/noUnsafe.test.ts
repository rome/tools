import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no unsafe",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.Component {
          UNSAFE_componentWillMount() {}
        }
				`,
				`
        class Hello extends React.Component {
          UNSAFE_componentWillReceiveProps() {}
        }
				`,
				`
        class Hello extends React.Component {
          UNSAFE_componentWillUpdate() {}
        }
        `,
				// VALID
				`
				class Hello extends React.Component {
          componentDidMount() {}
        }
				`,
			],
			{category: "lint/react/noUnsafe"},
		);
	},
);
