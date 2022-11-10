module.exports = {
	production: process.env.ELEVENTY_ENV === "production",
	WEBSITE_URL: process.env.WEBSITE_URL || "https://docs.rome.tools",
};
