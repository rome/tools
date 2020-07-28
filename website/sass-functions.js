const types = require("node-sass").types;
const path = require("path");
const fs = require("fs");
const {base64Encode} = require("./utils");

const stylesDir = path.join(__dirname, "src", "styles");
const staticDir = path.join(__dirname, "static");

function find(name) {
	const possible = [path.join(stylesDir, name), path.join(staticDir, name)];

	for (const loc of possible) {
		if (fs.existsSync(loc)) {
			return loc;
		}
	}

	throw new Error(`Unable to resolve ${name} in inline-url()`);
}

module.exports = {
	"inline-url($file)": function(file) {
		const filePath = find(file.getValue());
		const ext = filePath.split(".").pop();
		const buffer = fs.readFileSync(filePath);
		return types.String(`"${base64Encode(buffer, ext)}"`);
	},
};
