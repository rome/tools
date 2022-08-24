module.exports = {
	production: process.env.ELEVENTY_ENV === "production",
	WEBSITE_URL: process.env.WEBSITE_URL || "https://rome.tools",
	API_DOMAIN: process.env.API_DOMAIN || "https://api.rome.tools",
	// Prod: pk_live_51HfdGeFXA6dCpYSpI1yUt3vEbUE0AAV0swlTUuBL7XSdzSERrqkNAsitFuTaqDxdS7HcIs5wf0PG4Mqtys01LANs00GteBJgws
	// Test: pk_test_51HfdGeFXA6dCpYSpScbeo75rVpehHpEPqE5QysKBwJVTnN1NjAgEuGkEHfsgQpBe1KlYwWSeTaUa5ELDSJBPKjzI00ku8lUamF
	STRIPE_PUBLIC:
		process.env.STRIPE_CLIENT ||
		"pk_live_51HfdGeFXA6dCpYSpI1yUt3vEbUE0AAV0swlTUuBL7XSdzSERrqkNAsitFuTaqDxdS7HcIs5wf0PG4Mqtys01LANs00GteBJgws",
};
