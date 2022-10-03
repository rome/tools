const path = require("path");

require("dotenv").config({ path: path.resolve(__dirname, "..", ".env") });

/////

const constants = require("./constants.js");
const bodyParser = require("body-parser");
const express = require("express");
const fetch = require("node-fetch");
const morgan = require("morgan");
const url = require("url");
const pg = require("pg");
const app = express();

const db = new pg.Client();

const stripe = require("stripe")(process.env.STRIPE_SECRET);

let sentry;
if (process.env.SENTRY_DSN !== undefined) {
	sentry = require("@sentry/node");
	sentry.init({ dsn: process.env.SENTRY_DSN, tracesSampleRate: 1 });
}

const tiers = [
	{
		id: "supporter",
		name: "Supporter",
		type: "personal",
		metal: "copper",
		price: 10,
		rewards: ["Cosmetic Discord role", "Cosmetic label on GitHub issues"],
	},
	{
		id: "friend",
		name: "Friend",
		type: "personal",
		metal: "bronze",
		price: 25,
		rewards: ["Sticker"],
		previousRewards: [
			"Cosmetic Discord role",
			"Cosmetic label on GitHub issues",
		],
	},
	{
		id: "advocate",
		name: "Advocate",
		type: "personal",
		metal: "silver",
		price: 50,
		rewards: ["Sticker pack"],
		previousRewards: [
			"Cosmetic Discord role",
			"Cosmetic label on GitHub issues",
		],
	},
	{
		id: "champion",
		name: "Champion",
		type: "personal",
		metal: "gold",
		price: 100,
		rewards: [
			"Enamel pin",
			"Attribution in website credits",
			"Access to private Discord channel",
		],
		previousRewards: [
			"Cosmetic Discord role",
			"Cosmetic label on GitHub issues",
			"Sticker pack",
		],
	},
	{
		id: "benefactor",
		name: "Benefactor",
		type: "personal",
		metal: "platinum",
		price: 200,
		rewards: ["T-shirt", "Mug"],
		previousRewards: [
			"Cosmetic Discord role",
			"Cosmetic label on GitHub issues",
			"Sticker pack",
			"Enamel pin",
			"Attribution in website credits",
			"Access to private Discord channel",
		],
	},
	{
		id: "patron",
		name: "Patron",
		type: "personal",
		metal: "diamond",
		price: 500,
		rewards: [
			"Water bottle",
			"GitHub org badge",
			"Attribution in final release announcement",
		],
		previousRewards: [
			"Cosmetic Discord role",
			"Cosmetic label on GitHub issues",
			"Sticker pack",
			"Enamel pin",
			"Attribution in website credits",
			"Access to private Discord channel",
			"T-shirt",
			"Mug",
		],
	},
	{
		id: "business_bronze",
		name: "Business Bronze",
		type: "business",
		metal: "gold",
		price: 10_000,
		rewards: [
			"14 hours of migration support",
			"Attribution in final release announcement",
			"Tweet announcement",
		],
	},
	{
		id: "business_silver",
		name: "Business Silver",
		type: "business",
		metal: "platinum",
		price: 20_000,
		rewards: [
			"28 hours of migration support",
			"Logo on website homepage and GitHub README for 9 months",
			"Attribution in final release announcement",
			"Tweet announcement",
		],
	},
	{
		id: "business_gold",
		name: "Business Gold",
		type: "business",
		metal: "diamond",
		price: 50_000,
		rewards: [
			"56 hours of migration support",
			"Logo on website homepage and GitHub README for 18 months",
			"Prominent placement in final release announcement",
			"Tweet announcement",
		],
	},
];

function ensureBoolean(val) {
	return val === true;
}

function ensureString(str, maxLength) {
	if (typeof str !== "string") {
		return "";
	}

	if (maxLength !== undefined) {
		return str.slice(0, maxLength);
	}

	return str;
}

function normalizeCurrency(num) {
	if (typeof num !== "number" || isNaN(num)) {
		return 0;
	}

	// Don't allow negative numbers
	if (num < 0) {
		return num;
	}

	// Reduce precision to avoid decimal spam
	num = parseFloat(num.toPrecision(2));

	return num;
}

function normalizeUsername(value) {
	value = ensureString(value);

	// Remove whitespace
	value = value.replace(/\s/g, "");

	// Remove leading @
	value = value.replace(/^@/, "");

	// Remove domain: Could return null
	value = ensureString(url.parse(value).pathname);

	// Remove leading slash
	value = value.replace(/^\//, "");

	// Remove non-alphanumeric, dashes, and underscores
	value = value.replace(/[^a-zA-Z0-9\-_]/g, "");

	return value;
}

function getTierFromAmount(price) {
	for (const tier of tiers) {
		if (tier.price === price) {
			return tier;
		}
	}

	return { id: "custom", name: "Custom", price, rewards: [] };
}

function wrapAsyncCallback(callback) {
	return (req, res, next) => {
		try {
			callback(req, res, next).catch(next);
		} catch (err) {
			next(err);
		}
	};
}

app.use(morgan("tiny"));

// rome-ignore lint(nursery/noUnusedVariables): false positive
app.use((req, res, next) => {
	res.setHeader("Access-Control-Allow-Origin", "*");
	res.setHeader("Access-Control-Allow-Headers", "Content-Type");
	next();
});

async function getContributions(limit) {
	const query = await db.query(
		`SELECT "publicName", "publicComment", "tierPrice", tip, github, "createdAt" FROM contributions WHERE paid = true AND public = true ORDER BY "createdAt" DESC LIMIT ${limit}`,
	);

	return query.rows.map((row) => {
		return {
			name: row.publicName,
			github:
				row.github === "" || row.publicName === "" ? undefined : row.github,
			comment: row.publicComment,
			amount: Number(row.tierPrice) + Number(row.tip),
			time: new Date(row.createdAt).valueOf(),
		};
	});
}

async function getTierStats() {
	return Promise.all(
		tiers.map(async (tier) => {
			if (tier.type === "business") {
				return { ...tier, count: 0 };
			}

			const query = await db.query(
				`SELECT COUNT(*) FROM contributions WHERE paid = true AND "tierPrice" = $1`,
				[tier.price],
			);
			return {
				...tier,
				count: query.rows.length === 0 ? 0 : Number(query.rows[0].count),
			};
		}),
	);
}

async function getProgressStats() {
	const [countQuery, totalQuery] = await Promise.all([
		db.query("SELECT COUNT(*) FROM contributions WHERE paid = true"),
		db.query(
			`SELECT SUM("tierPrice") as "tierPrice", SUM("tip") as tip FROM contributions WHERE paid = true`,
		),
	]);

	const count =
		countQuery.rows.length === 0 ? 0 : Number(countQuery.rows[0].count);
	let current =
		totalQuery.rows.length === 0
			? 0
			: Number(totalQuery.rows[0].tierPrice) + Number(totalQuery.rows[0].tip);

	// Hard code current balance of external donations
	current += 1_733;

	// Automatically set a target
	let target = Math.round((current + 50_000) / 50_000) * 50_000;
	if (target === 50_000) {
		// Target $100k when under $50k
		target = 100_000;
	}

	return { count, current, target };
}

let cachedStats;

function getStats() {
	if (cachedStats === undefined) {
		cachedStats = getFreshStats();
	}
	return cachedStats;
}

let cachedAllContributions;

function getAllContributions() {
	if (cachedAllContributions === undefined) {
		cachedAllContributions = getContributions("ALL");
	}
	return cachedAllContributions;
}

async function getFreshStats() {
	const [{ count, current, target }, recentContributions, tiers] =
		await Promise.all([
			getProgressStats(),
			getContributions(3),
			getTierStats(),
		]);

	return { count, current, target, recentContributions, tiers };
}

app.get(
	"/funding/stats",
	// rome-ignore lint(nursery/noUnusedVariables): false positive
	wrapAsyncCallback(async (req, res) => {
		res.json(await getStats());
	}),
);

app.get(
	"/funding/all",
	// rome-ignore lint(nursery/noUnusedVariables): false positive
	wrapAsyncCallback(async (req, res) => {
		res.json(await getAllContributions());
	}),
);

function generateRewardsDescription(tier) {
	const rewards = [...(tier.rewards || []), ...(tier.previousRewards || [])];

	if (rewards.length === 0) {
		return undefined;
	} else {
		return rewards.join(", ");
	}
}

app.post(
	"/funding/checkout",
	bodyParser.json(),
	// rome-ignore lint(nursery/noUnusedVariables): false positive
	wrapAsyncCallback(async (req, res) => {
		const { body } = req;

		const email = ensureString(body.email);
		const tierPrice = normalizeCurrency(body.tierPrice);
		const tip = normalizeCurrency(body.tip);

		if (email === "") {
			res.json({ error: "Missing email" });
			return;
		}

		const tier = getTierFromAmount(tierPrice);
		const lineItems = [];

		lineItems.push({
			price_data: {
				currency: "usd",
				product_data: {
					name: `${tier.name} Tier`,
					description: generateRewardsDescription(tier),
					metadata: { id: tier.id },
				},
				unit_amount: tierPrice * 100,
			},
			quantity: 1,
		});

		if (tip > 0) {
			lineItems.push({
				price_data: {
					currency: "usd",
					product_data: { name: "Tip" },
					unit_amount: tip * 100,
				},
				quantity: 1,
			});
		}

		const session = await stripe.checkout.sessions.create({
			payment_method_types: ["card"],
			line_items: lineItems,
			mode: "payment",
			customer_email: email,
			success_url: `${process.env.WEBSITE_URL}/funding/checkout-complete`,
			cancel_url: `${process.env.WEBSITE_URL}/funding/`,
			billing_address_collection: "required",
			// Retain body in case of some database corruption
			metadata: req.body,
			// Don't request shipping address for custom donators
			shipping_address_collection:
				tier.id !== "custom" && tierPrice > 10
					? constants.stripeShippingCollection
					: undefined,
		});

		const isPublic = ensureBoolean(body.public);
		const publicName = ensureString(body.publicName, 100);
		const publicComment = ensureString(body.publicComment, 500);
		const twitter = normalizeUsername(body.twitter);
		const github = normalizeUsername(body.github);
		const discord = normalizeUsername(body.discord);

		await db.query(
			`INSERT INTO contributions ("stripeSession", "email", "tierId", "tierPrice", "public", "publicName", "publicComment", "tip", "twitter", "github", "discord") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);`,
			[
				session.id,
				email,
				tier.id,
				tierPrice,
				isPublic,
				publicName,
				publicComment,
				tip,
				twitter,
				github,
				discord,
			],
		);

		res.json({ id: session.id });
	}),
);

app.post(
	"/stripe-webhook",
	bodyParser.raw({ type: "application/json" }),
	wrapAsyncCallback(async (req, res) => {
		const payload = req.body;
		const sig = req.headers["stripe-signature"];
		const event = stripe.webhooks.constructEvent(
			payload,
			sig,
			process.env.STRIPE_WEBHOOK_SECRET,
		);

		// Handle the checkout.session.completed event
		if (event.type === "checkout.session.completed") {
			const session = event.data.object;
			await db.query(
				`UPDATE contributions SET paid = true WHERE "stripeSession" = $1 `,
				[session.id],
			);

			// Refresh stats
			cachedStats = undefined;
			cachedAllContributions = undefined;

			// Purge cache from Cloudflare
			await fetch(
				`https://api.cloudflare.com/client/v4/zones/${process.env.CF_ZONE_ID}/purge_cache`,
				{
					method: "POST",
					headers: {
						"Content-Type": "application/json",
						Authorization: `Bearer ${process.env.CF_SECRET}`,
					},
					body: JSON.stringify({
						files: [
							`${process.env.API_URL}/funding/stats`,
							`${process.env.API_URL}/funding/all`,
						],
					}),
				},
			);
		}

		res.status(200);
		res.end();
	}),
);

// rome-ignore lint(nursery/noUnusedVariables): false positive
app.use(function (err, req, res, next) {
	// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
	if (sentry !== undefined) {
		sentry.captureException(err);
	}
	console.error(err.stack);
	res.status(500);
	res.end("Internal server error");
	next;
});

async function main() {
	await db.connect();

	const port = Number(process.env.API_PORT || 8_081);

	app.listen(port, () => {
		console.log(`API server listening on port ${port}!`);
	});
}

main().catch((err) => {
	// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
	if (sentry !== undefined) {
		sentry.captureException(err);
	}
	console.error(err.stack);
	process.exit(1);
});
