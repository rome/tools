function base64Encode(buffer, ext) {
	if (ext === "svg") {
		return `data:image/svg+xml;utf8,${encodeURIComponent(buffer.toString())}`;
	}

	let mime = `image/${ext}`;
	if (ext === "woff2") {
		mime = "application/x-font-woff";
	}

	return `data:${mime};base64,${buffer.toString("base64")}`;
}

exports.base64Encode = base64Encode;
