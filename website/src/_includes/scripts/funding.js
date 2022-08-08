// {% include scripts/funding-utils.js %}
// global formatCurrency, setRecentContributions
//# State
let selectedTier = undefined;
let existingToast = undefined;

let loadingStripePromise = undefined;
let loadedStripe = false;

//# Elements

const modalContainer = document.querySelector(".modal-container");
const modalInner = document.querySelector(".modal-inner");
const modal = document.querySelector(".modal");
const main = document.querySelector(".main");

const closeButton = document.querySelector(".modal-close");
const modalAnchors = document.querySelectorAll(`.modal a[href^="#"]`);

const detailsStepIndicator = document.querySelector(".form-steps .details");
const reviewStepIndicator = document.querySelector(".form-steps .review");

const progressFillContainer = document.querySelector(".progress-fill");
const progressContainer = document.querySelector(".progress");
const progressLoading = document.querySelector(".progress-loading");
const progressFillText = document.querySelector(
	".progress-fill .progress-text",
);
const progressTotalText = document.querySelector(".progress > .progress-text");
const donationCount = document.querySelector(".donation-count");

const individualTiersContainer = document.querySelector(
	".individual-tiers-container",
);
const businessTiersContainer = document.querySelector(
	".business-tiers-container",
);
const hideIfPublicContribution = document.querySelectorAll(".public-hide");
const publicCheckbox = document.querySelector("input[name=public]");

const customInputForm = document.querySelector(".custom-input");
const customTierInput = document.querySelector(".custom-input input");
const customTierSelectButton = document.querySelector(".custom-input button");

const checkoutTotal = document.querySelector(".checkout-total");
const tipInput = document.querySelector("input[name=add-donation]");
const tipInputContainer = document.querySelector(".add-donation-container");
const checkoutButton = document.querySelector(".stripe-checkout");
const emailInput = document.querySelector("input[name=email]");

const detailsForm = document.querySelector(".modal form.details");
const reviewForm = document.querySelector(".modal form.review");
const detailsFormFocusableElements = getFocusableElements(detailsForm);
const reviewFormFocusableElements = getFocusableElements(reviewForm);

//# Utilities

function loadStripe() {
	if (loadedStripe) {
		return Promise.resolve();
	}

	if (loadingStripePromise !== undefined) {
		return loadingStripePromise;
	}

	loadingStripePromise = new Promise((resolve, reject) => {
		const script = document.createElement("script");
		script.src = "https://js.stripe.com/v3/";

		script.onload = () => {
			loadingStripePromise = undefined;
			loadedStripe = true;
			resolve();
		};

		script.onerror = (err) => {
			loadingStripePromise = undefined;
			addErrorToast("while loading the Stripe API", err);
			reject(err);
		};

		document.head.appendChild(script);
	});

	return loadingStripePromise;
}

function addErrorToast(description, err, msg) {
	if (typeof Sentry !== "undefined") {
		Sentry.captureException(err);
	}
	console.error(err);

	if (existingToast) {
		existingToast.remove();
		existingToast = undefined;
	}

	const toast = document.createElement("div");
	toast.classList.add("toast", "error");
	existingToast = toast;

	function addText(text) {
		const p = document.createElement("p");
		p.textContent = text;
		toast.appendChild(p);
	}

	let mainText = `An error occured ${description}`;
	if (msg === undefined) {
		msg = err.message;
	}
	if (msg !== undefined && msg !== "") {
		mainText = `${mainText}: "${msg}"`;
	}
	addText(mainText);

	const target = modalContainer.hidden ? main : modal;
	target.prepend(toast);
	toast.focus();
}

async function wrapFetch(opts, attempt = 0) {
	try {
		const res = await fetch(opts.url, opts.options);

		// On >500 errors, retry again in a few seconds
		// Only attempt this twice
		if (res.status > 500 && res.status < 599 && attempt < 2) {
			return new Promise((resolve) => {
				setTimeout(() => {
					resolve(wrapFetch(opts, attempt + 1));
				}, 2_000);
			});
		}

		if (res.status !== 200) {
			addErrorToast(
				opts.errorSuffix,
				new Error(`${opts.url} returned a ${res.status} status`),
				`Server returned ${res.status}`,
			);
			return;
		}

		try {
			const data = await res.json();
			if (
				typeof data === "object" &&
				data != null &&
				typeof data.error === "string"
			) {
				addErrorToast(
					opts.errorSuffix,
					new Error(`Network response ${JSON.stringify(data)}`),
					data.error,
				);
				return;
			}

			opts.then(data);
		} catch (err) {
			addErrorToast(opts.errorSuffix, err);
		}
	} catch (err) {
		addErrorToast(opts.errorSuffix, err, "Network request failed");
	}
}

function changeInputValue(input, value) {
	input.value = value;
	input.dispatchEvent(new Event("input", { bubbles: true }));
}

function closeModal() {
	if (checkoutButton.disabled) {
		// Do not allow closing the modal if we are in the process of checking out
		return;
	}

	selectedTier = undefined;
	updateModal();
}

function collapseModal() {
	modalContainer.classList.add("collapsed");
	document.body.style.removeProperty("overflow");

	if (!CSS.supports("zoom", "0.1")) {
		modalContainer.classList.add("collapsed-no-zoom");
	}
}

//# Modal closure

modalContainer.addEventListener("click", (e) => {
	if (e.target === modalContainer || e.target === modalInner) {
		if (modalContainer.classList.contains("collapsed")) {
			modalContainer.classList.remove("collapsed");
			document.body.style.overflow = "hidden";
		} else {
			closeModal();
		}
	}
});

closeButton.addEventListener("click", closeModal);

// Close modal when anchor links are clicked inside of it

for (const tag of modalAnchors) {
	tag.addEventListener("click", collapseModal);
}

//# Set the active modal and step

function setModalSteps(finalize) {
	detailsForm.hidden = finalize;
	reviewForm.hidden = !finalize;

	let active = detailsStepIndicator;
	let inactive = reviewStepIndicator;

	if (finalize) {
		active = reviewStepIndicator;
		inactive = detailsStepIndicator;
	}

	inactive.classList.remove("active");
	inactive.classList.remove("complete");
	active.classList.remove("complete");

	if (finalize) {
		detailsStepIndicator.classList.add("complete");
		detailsStepIndicator.classList.add("link");
		detailsStepIndicator.setAttribute("role", "link");
	} else {
		detailsStepIndicator.classList.remove("link");
		detailsStepIndicator.removeAttribute("role");
	}

	inactive.removeAttribute("role", "heading");
	inactive.removeAttribute("aria-level", "1");

	active.classList.add("active");
	active.setAttribute("role", "heading");
	active.setAttribute("aria-level", "1");
}

detailsStepIndicator.addEventListener("click", () => {
	if (detailsStepIndicator.classList.contains("link")) {
		setModalSteps(false);
	}
});

function openModal(tier) {
	modalContainer.classList.remove("collapsed");

	setModalSteps(false);

	// Remove tier description if one exists
	const existingTierDescription = document.querySelector(".modal .tier");
	// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
	if (existingTierDescription != null) {
		existingTierDescription.remove();
	}

	reviewForm.insertBefore(
		tier.preview,
		reviewForm.querySelector(".summary-heading"),
	);

	selectedTier = tier;
	updateModal();
}

function updateModal() {
	if (selectedTier === undefined) {
		modalContainer.hidden = true;
		document.body.style.removeProperty("overflow");
		return;
	}

	modalContainer.hidden = false;
	document.body.style.overflow = "hidden";

	modalContainer.scrollTop = 0;

	updateCheckoutTotal();

	emailInput.focus();
}

function getFocusableElements(target) {
	return target.querySelectorAll(
		'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
	);
}

// Trap focus in modal when open
// Credit https://uxdesign.cc/how-to-trap-focus-inside-modal-to-make-it-ada-compliant-6a50f9a70700
document.addEventListener("keydown", function (e) {
	let isTabPressed = e.key === "Tab" || e.keyCode === 9;
	if (!isTabPressed) {
		return;
	}

	// Only trap focus if modal is visible
	if (modalContainer.hidden) {
		return;
	}

	// Select active step
	const focusableElements =
		detailsForm.hidden ? reviewFormFocusableElements : detailsFormFocusableElements;
	const firstFocusableElement = focusableElements[0];
	const lastFocusableElement = focusableElements[focusableElements.length - 1];

	let focusElement;

	if (e.shiftKey) {
		if (document.activeElement === firstFocusableElement) {
			focusElement = lastFocusableElement;
		}
	} else {
		if (document.activeElement === lastFocusableElement) {
			focusElement = firstFocusableElement;
		}
	}

	if (focusElement !== undefined) {
		e.preventDefault();
		focusElement.focus();
	}
});

//# Details submit

detailsForm.addEventListener("submit", (e) => {
	e.preventDefault();

	if (emailInput.matches(":invalid")) {
		alert("Email is required");
		return;
	}

	setModalSteps(true);
});

//# Checkout submit

reviewForm.addEventListener("submit", (e) => {
	e.preventDefault();

	if (checkoutButton.disabled) {
		return;
	}

	// Do some backend magic...
	const oldCheckoutButtonText = checkoutButton.textContent;
	checkoutButton.disabled = true;
	checkoutButton.textContent = "Submitting...";

	const data = {
		public: document.querySelector("input[name=public]").checked,
		publicName: document.querySelector("input[name=public-name]").value,
		publicComment: document.querySelector("textarea[name=public-comment]").value,
		tip: getTip(),
		tierPrice: selectedTier.price,
		email: emailInput.value,
		twitter: document.querySelector("input[name=twitter]").value,
		github: document.querySelector("input[name=github]").value,
		discord: document.querySelector("input[name=discord]").value,
	};

	const stripeLoad = loadStripe();

	wrapFetch({
		errorSuffix: "while submitting checkout form",
		url: "{{ env.API_DOMAIN }}/funding/checkout",
		options: {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify(data),
		},
		then: (res) => {
			return stripeLoad.then(() => {
				const stripe = Stripe("{{ env.STRIPE_PUBLIC }}");
				localStorage.setItem("checkout-tier", selectedTier.name);
				stripe.redirectToCheckout({ sessionId: res.id });
				return new Promise(() => {});
			});
		},
	}).then(() => {
		checkoutButton.textContent = oldCheckoutButtonText;
		checkoutButton.disabled = false;
	});
});

//# Calculate checkout total

function updateCheckoutTotal() {
	const tip = getTip();

	// Highlight button
	for (const [price, elem] of addDonationButtons) {
		elem.classList.remove("active");

		let isActive = false;

		if (price === tip && tipInputContainer.hidden) {
			isActive = true;
		}

		if (price == null && !tipInputContainer.hidden) {
			isActive = true;
		}

		if (isActive) {
			elem.classList.add("active");
		}
	}

	const selectedTierPrice = selectedTier === undefined ? 0 : selectedTier.price;

	let total = formatCurrency(selectedTierPrice);

	if (tip !== undefined) {
		total = `${formatCurrency(selectedTierPrice + tip)} = ${formatCurrency(
			selectedTierPrice,
		)} Tier + ${formatCurrency(tip)} Tip`;
	}

	checkoutTotal.textContent = total;
}

function getTip() {
	const value = tipInput.value;
	if (value === "" || value === "0" || tipInput.matches(":invalid")) {
		return undefined;
	} else {
		return Number(value);
	}
}

tipInput.addEventListener("change", updateCheckoutTotal);
tipInput.addEventListener("keyup", updateCheckoutTotal);

const tips = [10, 20, 50, 100, undefined];

const addDonationButtonContainer = document.querySelector(
	".add-donation-buttons",
);
const addDonationButtons = [];

for (const price of tips) {
	const item = document.createElement("li");

	const button = document.createElement("button");
	button.textContent = price === undefined ? "Custom" : formatCurrency(price);
	item.appendChild(button);

	button.addEventListener("click", (e) => {
		e.preventDefault();

		if (price === undefined && tipInputContainer.hidden) {
			tipInputContainer.hidden = false;
			changeInputValue(tipInput, "0");
			tipInput.focus();
		} else {
			tipInputContainer.hidden = true;
			if (button.classList.contains("active")) {
				changeInputValue(tipInput, "0");
			} else {
				changeInputValue(tipInput, price);
			}
		}

		updateCheckoutTotal();
	});

	addDonationButtonContainer.appendChild(item);
	addDonationButtons.push([price, button]);
}

//# Custom tier selection

function toggleCustomTierSelectButton() {
	const price = Number(customTierInput.value);
	customTierSelectButton.hidden =
		customTierInput.matches(":invalid") || price <= 0;
}

customTierInput.addEventListener("change", toggleCustomTierSelectButton);
customTierInput.addEventListener("keyup", toggleCustomTierSelectButton);

customInputForm.addEventListener("submit", (e) => {
	e.preventDefault();

	if (!customTierSelectButton.hidden) {
		const price = Number(customTierInput.value);

		const preview = document.createElement("div");
		preview.classList.add("tier", "copper");

		const heading = document.createElement("h4");
		heading.textContent = `Custom ${formatCurrency(price)}`;
		preview.appendChild(heading);

		openModal({ name: "Custom", preview, price });
	}
});

//# Error and success presentation

// TODO

//# Hide public inputs if necessary

publicCheckbox.addEventListener("change", () => {
	for (const elem of hideIfPublicContribution) {
		elem.hidden = !publicCheckbox.checked;
	}
});

//# Form saving and restoration

const saveElements = modalContainer.querySelectorAll(
	"textarea, input[type=text], input[type=email], input[type=checkbox], input[type=number]",
);

for (const elem of saveElements) {
	const storageKey = `form-${elem.name}`;

	elem.addEventListener("change", () => {
		localStorage.setItem(storageKey, JSON.stringify(elem.value));
	});

	let saved = localStorage.getItem(storageKey);
	if (saved == null || saved === "") {
		continue;
	}

	saved = JSON.parse(saved);

	if (elem.type === "checkbox") {
		elem.checked = saved === "on";
		changeInputValue(elem, elem.value);
	} else {
		changeInputValue(elem, saved);
	}
}

// Show custom input if we hydrated it with a value that doesn't correspond with a button

if (
	tipInput.value !== "" && !document.querySelector(
		`.add-donation-buttons[data-price="${tipInput.value}"]`,
	)
) {
	tipInputContainer.hidden = false;
	updateCheckoutTotal();
}

//# Progress

function buildTierButton(tier) {
	const button = document.createElement("div");
	button.classList.add("tier", tier.metal);
	button.role = "button";

	const header = document.createElement("div");
	header.classList.add("header");
	button.appendChild(header);

	const title = document.createElement("h4");
	title.textContent = tier.name;
	header.appendChild(title);

	const headerRight = document.createElement("div");
	headerRight.classList.add("header-right");
	header.appendChild(headerRight);

	const price = document.createElement("div");
	price.classList.add("price");
	price.textContent = formatCurrency(tier.price);
	headerRight.appendChild(price);

	const already = document.createElement("div");
	already.classList.add("already");
	if (tier.count > 0 && tier.type === "personal") {
		already.textContent = `${tier.count.toLocaleString()} ${
			tier.count === 1 ? "person" : "people"
		} selected this tier`;
	}
	headerRight.appendChild(already);

	const rewards = buildList(tier.rewards);

	if (tier.previousRewards === undefined) {
		button.appendChild(rewards);
	} else {
		const previousRewards = buildList(tier.previousRewards);

		const previousWrapper = document.createElement("div");
		previousWrapper.classList.add("previous");

		const heading = document.createElement("strong");
		heading.textContent = "From previous tier";

		previousWrapper.appendChild(heading);
		previousWrapper.appendChild(previousRewards);

		const columns = document.createElement("div");
		columns.classList.add("columns");
		columns.appendChild(rewards);
		columns.appendChild(previousWrapper);
		button.appendChild(columns);
	}

	return button;
}

function buildList(strs) {
	const list = document.createElement("ul");
	for (const str of strs) {
		const item = document.createElement("li");
		item.textContent = str;
		list.appendChild(item);
	}
	return list;
}

function addTier(tier, interactive) {
	const button = buildTierButton(tier);
	button.classList.add("clickable");

	if (tier.type === "business") {
		businessTiersContainer.appendChild(button);
	} else {
		individualTiersContainer.appendChild(button);
	}

	if (interactive) {
		button.addEventListener("click", () => {
			openModal({ ...tier, preview: buildTierButton(tier) });
		});
	}
}

// {% include scripts/recent-contributions.js %}

function processStats(res, interactive) {
	// May have already been set
	progressFillContainer.style.removeProperty("min-width");
	progressFillContainer.style.removeProperty("width");

	progressFillText.textContent = formatCurrency(Math.round(res.current));
	progressTotalText.textContent = formatCurrency(res.target);
	progressLoading.hidden = true;
	donationCount.textContent = res.count.toLocaleString();

	if (interactive) {
		function show() {
			const percent = Math.min(100, (100 / res.target) * res.current);
			progressFillContainer.style.minWidth = `${progressFillContainer.clientWidth}px`;
			progressFillContainer.style.width = "0";

			requestAnimationFrame(() => {
				progressFillContainer.style.width = `${percent}%`;
			});
		}

		if (typeof IntersectionObserver === "undefined") {
			show();
		} else {
			// Animate the progress fill only when it's visible to draw attention
			let observer = new IntersectionObserver(
				(changes) => {
					for (const { isIntersecting } of changes) {
						if (!isIntersecting) {
							continue;
						}

						observer.unobserve(progressContainer);
						show();
					}
				},
				{ threshold: 1 },
			);
			observer.observe(progressContainer);
		}
	}

	individualTiersContainer.textContent = "";
	businessTiersContainer.textContent = "";

	for (const tier of res.tiers) {
		addTier(tier, interactive);
	}

	setRecentContributions(res.recentContributions);
}

//# Init

toggleCustomTierSelectButton();

// Load cached stats (may need a breaker here if the data structure changes too much)
const cachedStatus = localStorage.getItem("stats");
if (cachedStatus != null) {
	processStats(JSON.parse(cachedStatus), false);
}

// Fetch latest stats
wrapFetch({
	errorSuffix: "while loading donations",
	url: "{{ env.API_DOMAIN }}/funding/stats",
	then: (res) => {
		localStorage.setItem("stats", JSON.stringify(res));
		processStats(res, true);
	},
});

window.addEventListener("error", function (event) {
	addErrorToast("that wasn't handled", event);
});
