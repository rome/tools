import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";

test(
	"should push a new property",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				lint: {
					globals: [],
				},
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({
				commandName: "config push",
				args: ["lint.globals", "jQuery"],
			});
		},
	),
);

test(
	"should disable an existing property",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				lint: {
					enabled: true,
				},
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({commandName: "config disable", args: ["lint.enabled"]});
		},
	),
);

test(
	"should enable an existing property",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				lint: {
					enabled: false,
				},
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({commandName: "config enable", args: ["lint.enabled"]});
		},
	),
);

test(
	"should set a new value for an existing property",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				format: {
					indentSize: 2,
				},
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({
				commandName: "config set",
				args: ["format.indentSize", "9"],
			});
		},
	),
);

test(
	"should remove an existing property",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				format: {
					indentSize: 2,
				},
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({
				commandName: "config remove",
				args: ["format.indentSize"],
			});
		},
	),
);


test(
	"should pop a value from an existing array",
	createIntegrationTest(
		{
			files: {},
			projectConfig: {
				root: true,
				name: "dummy",
				lint: {
					globals: ["beforeEach"]
				}
			},
			disableTest: true,
		},
		async (t, {client}) => {
			await client.query({
				commandName: "config pop",
				args: ["lint.globals", "beforeEach"],
			});
		},
	),
);
