import {test} from "rome";
import "@internal/core";
import {
	Chrome,
	getAllBrowserNames,
	getAllBrowserUsages,
	getBrowser,
} from "@internal/browser-features";

test(
	"test browser querying",
	(t) => {
		t.is(
			getBrowser({
				name: "firefox",
			}).getName(),
			"Firefox",
		);

		t.is(
			getBrowser({
				name: "firefox",
			}).getName(),
			"Firefox",
		);

		t.is(
			getBrowser({
				name: "Firefox",
			}).getName(),
			"Firefox",
		);

		t.throws(() => {
			getBrowser({
				name: "dgsbhjsjkl",
			});
		});

		t.is(
			getBrowser({
				name: "FF",
			}).getName(),
			"Firefox",
		);

		t.is(
			getBrowser({
				name: "firefox",
				version: 84,
			}).getVersion(),
			84,
		);

		t.throws(() => {
			getBrowser({
				name: "firefox",
				version: 679_934,
			});
		});
	},
);

test(
	"test browser caching",
	(t) => {
		t.is(getBrowser({name: "chrome"}), getBrowser({name: "Chr."}));

		t.not(
			getBrowser({name: "chrome", version: 4}),
			getBrowser({name: "chrome"}),
		);

		t.is(
			getBrowser({name: "chrome", version: 4}),
			getBrowser({name: "Chr.", version: 4}),
		);

		t.is(
			getBrowser({name: "chrome", version: new Chrome().getCurrentVersion()}),
			getBrowser({name: "chrome"}),
		);
	},
);

test(
	"misc browser feature tests",
	(t) => {
		t.notThrows(() => {
			getAllBrowserNames();
		});

		t.notThrows(() => {
			getAllBrowserUsages();
		});
	},
);
