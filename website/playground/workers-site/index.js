import { getAssetFromKV } from "@cloudflare/kv-asset-handler";

/**
 * The DEBUG flag will do two things that help during development:
 * 1. we will skip caching on the edge, which makes it easier to
 *    debug.
 * 2. we will return an error message on exception in your Response rather
 *    than the default 404.html page.
 */
const DEBUG = false;

addEventListener("fetch", (event) => {
	try {
		event.respondWith(handleEvent(event));
	} catch (e) {
		if (DEBUG) {
			return event.respondWith(
				new Response(e.message || e.toString(), { status: 500 }),
			);
		}
		event.respondWith(new Response("Internal Error", { status: 500 }));
	}
});

async function handleEvent(event) {
	let options = {};

	/**
   * You can add custom logic to how we fetch your assets
   * by configuring the function `mapRequestToAsset`
   */
	// options.mapRequestToAsset = handlePrefix(/^\/docs/)

	try {
		if (DEBUG) {
			// customize caching
			options.cacheControl = { bypassCache: true };
		}
		const page = await getAssetFromKV(event, options);

		// allow headers to be altered
		const response = new Response(page.body, page);

		response.headers.set("X-XSS-Protection", "1; mode=block");
		response.headers.set("X-Content-Type-Options", "nosniff");
		response.headers.set("X-Frame-Options", "DENY");
		response.headers.set("Referrer-Policy", "unsafe-url");
		response.headers.set("Feature-Policy", "none");

		return response;
	} catch (e) {
		// if an error is thrown try to serve the asset at 404.html
		if (!DEBUG) {
			try {
				let notFoundResponse = await getAssetFromKV(event, {
					mapRequestToAsset: (req) =>
						new Request(`${new URL(req.url).origin}/404.html`, req),
				});

				return new Response(notFoundResponse.body, {
					...notFoundResponse,
					status: 404,
				});
			} catch {}
		}

		return new Response(e.message || e.toString(), { status: 500 });
	}
}
