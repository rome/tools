import {test} from "rome";
import {InlineState} from "@internal/markdown-parser/State";

test(
	"State should return the correct delimiters and if they are connected",
	(t) => {
		const state = new InlineState();

		state.registerStartOfDelimiter(1, "Emphasis");
		state.registerStartOfDelimiter(3, "Strong");

		state.connectDelimiter(6, "Strong");

		t.truthy(state.isDelimiterClosed(6, "Strong"));
		t.is(state.isDelimiterClosed(6, "Strong"), 3);
		t.falsy(state.isDelimiterClosed(1, "Emphasis"));
	},
);

test(
	"State should return the correct indexes when connecting delimiters when they are registered in even number",
	(t) => {
		const state = new InlineState();

		state.registerStartOfDelimiter(1, "Emphasis");
		state.registerStartOfDelimiter(3, "Strong");
		state.registerStartOfDelimiter(10, "Emphasis");
		state.registerStartOfDelimiter(24, "Strong");

		state.connectDelimiter(50, "Strong");
		state.connectDelimiter(100, "Emphasis");
		state.connectDelimiter(300, "Strong");
		state.connectDelimiter(450, "Emphasis");

		t.is(state.isDelimiterClosed(450, "Emphasis"), 1);
		t.is(state.isDelimiterClosed(300, "Strong"), 3);
		t.is(state.isDelimiterClosed(10, "Emphasis"), false);
		t.is(state.isDelimiterClosed(24, "Strong"), false);
	},
);

test(
	"State should return the correct indexes when connecting delimiters when registered in odd number",
	(t) => {
		const state = new InlineState();

		state.registerStartOfDelimiter(1, "Emphasis");
		state.registerStartOfDelimiter(3, "Strong");
		state.registerStartOfDelimiter(10, "Emphasis");
		state.registerStartOfDelimiter(24, "Strong");

		state.connectDelimiter(50, "Strong");
		state.connectDelimiter(100, "Strong");
		state.connectDelimiter(300, "Emphasis");

		t.is(state.isDelimiterClosed(1, "Emphasis"), false);
		t.is(state.isDelimiterClosed(100, "Strong"), 3);
		t.is(state.isDelimiterClosed(300, "Emphasis"), 10);
		t.is(state.isDelimiterClosed(24, "Strong"), false);
	},
);

test(
	"State have the correct delimiters",
	(t) => {
		const state = new InlineState();

		state.registerStartOfDelimiter(1, "Emphasis");
		state.connectDelimiter(50, "Emphasis");

		state.registerStartOfDelimiter(70, "Strong");
		state.connectDelimiter(100, "Strong");

		state.registerStartOfDelimiter(150, "Emphasis");
		state.connectDelimiter(175, "Emphasis");

		state.registerStartOfDelimiter(179, "Strong");
		state.connectDelimiter(200, "Strong");

		const delimiters = Array.from(state.getDelimiters());

		t.is(delimiters[0].start, 1);
		t.is(delimiters[0].end, 50);
		t.is(delimiters[0].delimiterType, "Emphasis");

		t.is(delimiters[1].start, 70);
		t.is(delimiters[1].end, 100);
		t.is(delimiters[1].delimiterType, "Strong");

		t.is(delimiters[2].start, 150);
		t.is(delimiters[2].end, 175);
		t.is(delimiters[2].delimiterType, "Emphasis");

		t.is(delimiters[3].start, 179);
		t.is(delimiters[3].end, 200);
		t.is(delimiters[3].delimiterType, "Strong");
	},
);
