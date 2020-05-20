import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no did mount set state",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.Component {
          componentDidMount() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends React.Component {
          componentDidMount() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentDidMount() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentDidMount() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				// VALID
				`
        class Hello extends React.Component {
          componentDidMount() {
            if (condition) {
              this.setState({
                name: 'John'
              });
            }
          }
        }
        `,
			],
			{category: "lint/react/noDidMountSetState"},
		);
	},
);
