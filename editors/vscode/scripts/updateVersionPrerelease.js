import {readFile, writeFile} from "node:fs/promises";
import {join, resolve} from "node:path";

const manifestPath = resolve(join("package.json"));

// read the package.json file
readFile(manifestPath, "utf8").then(async (value) => {
    const manifest = JSON.parse(value);
    const date = new Date();
    const dateParts = [date.getFullYear(), date.getMonth() + 1, date.getDate()];
    // update the version field
    manifest.version = `${manifest.version}-prerelease.${dateParts.join(".")}`;
    try {
        await writeFile(manifestPath, JSON.stringify(manifest, null, "\t"));
    } catch (_e) {
        console.log("Could not write the package.json file at " + manifestPath)
        process.exit(1);

    }
}).catch(() => {
    console.log("Could not read the package.json file at " + manifestPath)
    process.exit(1);
})


