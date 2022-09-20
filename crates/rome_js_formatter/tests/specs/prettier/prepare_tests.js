const { promises: fs } = require("fs");
const path = require("path");
const prettier = require("prettier");

if (process.argv.length < 3) {
    console.error("Usage: node prepare_tests.js <prettier root>");
    process.exit(2);
}

const PRETTIER_ROOT = path.resolve(process.argv[2], "tests/format");

// Recursively traverse the test directory to search for snapshots files
async function traverseDir(dir, config) {
    for (const entry of await fs.readdir(dir, { withFileTypes: true })) {
        if (entry.isDirectory()) {
            await traverseDir(path.resolve(dir, entry.name), config);
            continue;
        }

        if (entry.isFile()) {
            // Ignore all non-snapshot files
            if (entry.name !== "jsfmt.spec.js.snap") {
                continue;
            }

            // Load the snapshot as CommonJS
            const snapshot = require(path.join(dir, entry.name));

            // Snapshot files are stored in __snapshots__/jsfmt.spec.js.snap,
            // iterate on all files in the parent directory
            for (const file of await fs.readdir(path.resolve(dir, ".."))) {
                // for each file, check it has an associated snapshot (the
                // jsfmt.spec.js files don't have one for instance)
                const key = `${file} format 1`;
                if (key in snapshot && typeof snapshot[key] === "string") {
                    // Compute a relative path from the Prettier root directory
                    // to this file, then an absolute path using the rome_js_formatter
                    // specs directory as a root instead
                    const filePath = path.resolve(dir, "..", file);
                    const relPath = path.relative(PRETTIER_ROOT, filePath);
                    const outPath = path.resolve(__dirname, relPath);

                    // Copy the snapshot input file, ensuring the
                    // parent directory exists
                    const outDir = path.resolve(outPath, "..");
                    await fs.mkdir(outDir, { recursive: true });
                    await fs.copyFile(filePath, outPath);

                    // Extract the expected output from the snapshot text
                    const OUTPUT = "=====================================output=====================================";
                    const FOOTER = "================================================================================";

                    let snapContent = snapshot[key];
                    const start = snapContent.match(new RegExp(OUTPUT + "\\n"));
                    const end = snapContent.match(new RegExp("\\n" + FOOTER));

                    const startOffset = start.index + start[0].length;
                    const endOffset = end.index;
                    snapContent = snapContent.substring(startOffset, endOffset);
                    try {
                        // We need to reformat prettier snapshot
                        // because Rome and Prettier have different default options
                        snapContent = prettier.format(snapContent, config);
                    } catch (error) {
                        console.error(`Prettier format error in ${filePath}: ${error}`);
                    }
                    // Write the expected output to an additional prettier-snap
                    // file in the specs directory
                    const snapFile = path.basename(file) + ".prettier-snap";
                    await fs.writeFile(
                        path.resolve(outDir, snapFile),
                        snapContent,
                    );
                }
            }
        }
    }
}

const PRETTIER_ROOT_JS = path.resolve(PRETTIER_ROOT, "js");
const PRETTIER_ROOT_JSX = path.resolve(PRETTIER_ROOT, "jsx");
const PRETTIER_ROOT_TS = path.resolve(PRETTIER_ROOT, "typescript");

const defaultConfig = {
    trailingComma: "all",
    tabWidth: 2,
    printWidth: 80,
    singleQuote: false,
    useTabs: false,
};

async function main() {
    console.log("Extracting tests from %s ...", PRETTIER_ROOT_JS);
    await traverseDir(PRETTIER_ROOT_JS, {
        ...defaultConfig,
        parser: "babel",
    });

		console.log("Extracting tests from %s ...", {PRETTIER_ROOT_JSX});
		await traverseDir(PRETTIER_ROOT_JSX, {
			...defaultConfig,
			parser: "babel",
		});

    console.log("Extracting tests from %s ...", PRETTIER_ROOT_TS);
    await traverseDir(PRETTIER_ROOT_TS, {
        ...defaultConfig,
        parser: "typescript"
    });
}

main().catch(
    (err) => {
        console.error(err);
        process.exit(1);
    },
);
