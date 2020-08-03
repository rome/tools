import {Consumer} from "@internal/consume";
import https = require("https");
import {consumeJSON} from "@internal/codec-json";
import {ROOT, modifyGeneratedFile} from "./_utils";
import {escapeXHTMLEntities} from "@internal/html-parser";
import {parseCLIFlagsFromProcess} from "@internal/cli-flags";

type Contributor = {
	username: string;
	avatar: string;
};

type Contributors = Array<Contributor>;

function get(
	url: string,
	token: string,
): Promise<{
	nextURL: undefined | string;
	data: Consumer;
}> {
	return new Promise((resolve, reject) => {
		const req = https.get(
			url,
			{
				headers: {
					"User-Agent": "@romefrontend",
					Authorization: `token ${token}`,
				},
			},
			(res) => {
				const {headers} = res;
				let nextURL: undefined | string;

				if (typeof headers.link === "string") {
					const match = headers.link.match(/<([^>]+)>; rel="next"/);
					if (match != null) {
						nextURL = match[1];
					}
				}

				let buff = "";

				res.setEncoding("utf8");
				res.on(
					"data",
					(chunk) => {
						buff += chunk;
					},
				);

				res.on(
					"end",
					() => {
						try {
							resolve({
								nextURL,
								data: consumeJSON({
									input: buff,
								}),
							});
						} catch (err) {
							reject(err);
						}
					},
				);
			},
		);

		req.on(
			"error",
			(err) => {
				reject(err);
			},
		);
	});
}

async function getContributors(
	url: string,
	token: string,
): Promise<Contributors> {
	let contributors: Contributors = [];

	const {data, nextURL} = await get(url, token);

	for (const elem of data.asIterable()) {
		contributors.push({
			username: elem.get("login").asString(),
			avatar: elem.get("avatar_url").asString(),
		});
	}

	if (nextURL === undefined) {
		return contributors;
	} else {
		return [...contributors, ...(await getContributors(nextURL, token))];
	}
}

export async function main(args: Array<string>) {
	const {token} = await parseCLIFlagsFromProcess({
		args,
		defineFlags(c) {
			return {
				token: c.get("token").asString(),
			};
		},
	}).init();

	await modifyGeneratedFile(
		{
			path: ROOT.append("website", "src", "credits.md"),
			scriptName: "update-contributor-credits",
		},
		async () => {
			const contributors = await getContributors(
				"https://api.github.com/repos/romefrontend/rome/contributors",
				token,
			);
			const lines: Array<string> = [];

			lines.push(`<ul class="team-list credits">`);

			for (const {username, avatar} of contributors) {
				lines.push(
					`<li><a href="https://github.com/romefrontend/rome/commits?author=${encodeURIComponent(
						username,
					)}">`,
				);
				lines.push(`<img src="${escapeXHTMLEntities(avatar)}">`);
				lines.push(`<span>${escapeXHTMLEntities(username)}</span>`);
				lines.push("</a></li>");
			}

			lines.push("</ul>");

			return {lines};
		},
	);

	return 0;
}
