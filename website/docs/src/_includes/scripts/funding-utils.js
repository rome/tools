// Funding utils
function formatCurrency(num = 0) {
	const text = num.toLocaleString("en-US");
	if (Number.isInteger(num)) {
		return `$${text}`;
	} else {
		// Format decimals with two digit precision
		// This is mainly so people can donate $4.2 and have it display as $4.20 lol
		const int = Math.floor(num);
		const dec = num - int;
		return `$${int.toLocaleString("en-US")}.${dec.toFixed(2).slice(2)}`;
	}
}

const SECOND = 1_000;
const MINUTE = 60 * SECOND;
const HOUR = MINUTE * 60;
const DAY = HOUR * 24;
const MONTH = DAY * 30;
const YEAR = DAY * 365;

function humanizeRelativeTime(relative) {
	const elapsed = Date.now() - relative;

	if (elapsed < MINUTE) {
		return `${Math.round(elapsed / 1_000)} seconds ago`;
	} else if (elapsed < HOUR) {
		return `${Math.round(elapsed / MINUTE)} minutes ago`;
	} else if (elapsed < DAY) {
		return `${Math.round(elapsed / HOUR)} hours ago`;
	} else if (elapsed < MONTH) {
		return `~${Math.round(elapsed / DAY)} days ago`;
	} else if (elapsed < YEAR) {
		return `~${Math.round(elapsed / MONTH)} months ago`;
	} else {
		return `~${Math.round(elapsed / YEAR)} years ago`;
	}
}

humanizeRelativeTime;
formatCurrency;
