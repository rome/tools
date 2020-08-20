import {test} from "rome";
import {extractFileList} from "./index";

test(
	"should extract file names from sample git output",
	async (t) => {
		const sampleInput = ` \
this is a sample string
only lines containing file names should be captured
?? file1.txt
A file2.txt
M file3.txt
K file4.txt
? file5.txt
?? file6
??A file7.txt
?? anything after`;

		const expectedMatch = [
			"file1.txt",
			"file2.txt",
			"file3.txt",
			"file6",
			"anything after",
		];

		t.looksLike(extractFileList(sampleInput), expectedMatch);
	},
);
