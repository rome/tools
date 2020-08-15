import {test} from "rome";
import MappingList from "@internal/codec-source-map/MappingList";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {Mapping} from "@internal/codec-source-map/types";

function generateMapping(
	generatedLine: number,
	generatedColumn: number,
	originalLine: number,
	originalColumn: number,
): Mapping {
	return {
		name: undefined,
		source: undefined,
		original: {
			line: ob1Coerce1(originalLine),
			column: ob1Coerce0(originalColumn),
		},
		generated: {
			line: ob1Coerce1(generatedLine),
			column: ob1Coerce0(generatedColumn),
			index: ob1Coerce0(0),
		},
	};
}

test(
	"verify returned array order",
	async (t) => {
		const list = new MappingList();

		const mapping1 = generateMapping(1, 6, 1, 0);
		const mapping2 = generateMapping(1, 0, 1, 6);
		const mapping3 = generateMapping(3, 12, 3, 8);
		const mapping4 = generateMapping(6, 0, 1, 8);

		list.add(mapping1);
		list.add(mapping2);
		list.add(mapping3);
		list.add(mapping4);

		const array = list.toArray();

		t.is(array[0], mapping2);
		t.is(array[1], mapping1);
		t.is(array[2], mapping3);
		t.is(array[3], mapping4);
	},
);
