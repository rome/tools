// @ts-check

const originalTitle = document.title;
const headerMobile = document.querySelector(".header-mobile");
const sidebar = document.querySelector(".sidebar");
const tocList = document.querySelector(".toc");

/** @type {HTMLElement} */
const tocContainer = document.querySelector(".toc-menu");

/** @type {Array<HTMLAnchorElement>} */
const tocLinks = Array.from(document.querySelectorAll(".toc a"));

/** @type {Array<{
 * heading: HTMLElement,
 * link: HTMLAnchorElement,
 * }>} */
const headingElements = tocLinks.map((link) => {
	return {
		heading: document.querySelector(`[id="${link.getAttribute("href").slice(1)}"]`),
		link,
	};
});


/**
 * @typedef {Object} CalculatedHeading
 * @property {string} id
 * @property {HTMLAnchorElement} link
 * @property {Array<string>} titles
 * @property {number} level
 * @property {number} start
 * @property {number} end
 */

class TableOfContents {
	constructor() {
		/** @type {Array<CalculatedHeading>} */
		this.headingsCalculated = [];

		/** @type {undefined | number} */
		this.lastActiveHeading = undefined;
	}

	/**
	 * @param {MouseEvent} event
	 */
	handleTOCClick(event) {
		const target = event.target;
		event.preventDefault();
		if (!(target instanceof HTMLElement)) {
			return;
		}

		if (target.hasAttribute("href")) {
			const hash = target.getAttribute("href");
			window.location.hash = hash;
			this.scrollToHeading(hash);

			if (isMobile) {
				toggleMobileSidebar();
			}
		}
	}

	/**
	 * @returns {number}
	 */
	getScrollOffset() {
		let offset = 0;

		// Account for mobile header
		if (isMobile) {
			offset += headerMobile.clientHeight;
		}

		// Give everything a tiny bit of margin so it's not up against the edges
		offset += 20;

		return offset;
	}

	/**
	 * @param {HTMLElement} heading
	 * @returns {number}
	 */
	getHeadingTop(heading) {
		return heading.offsetTop -  parseFloat(window.getComputedStyle(heading).marginTop) - this.getScrollOffset();
	}

	/**
	 * @param {number} i
	 * @param {Array<CalculatedHeading>} stack
	 * @returns {CalculatedHeading}
	 */
	calculateHeading(i, stack) {
		const {heading, link} = headingElements[i];
		const id = `#${heading.getAttribute("id")}`;

		// Extract the level from the H tag
		const level = Number(heading.tagName[1]);

		// Get the headings above this one for us in document.title
		/** @type {Array<string>} */
		let titles = [heading.textContent.trim()];
		for (let i = stack.length - 1; i >= 0; i--) {
			const heading = stack[i];
			if (heading.level < level) {
				titles = heading.titles.concat(titles);
				break;
			}
		}

		// Calculate when this heading ends. It's either at the beginning of the next heading, or page bottom.
		let start = this.getHeadingTop(heading);
		let end;

		const nextHeading = headingElements[i + 1];
		if (nextHeading) {
			end = this.getHeadingTop(nextHeading.heading);
		} else {
			end = document.body.clientHeight;
		}

		return {
			level,
			id,
			titles,
			link,
			start,
			end,
		};
	}

	calculateHeadingsPositions() {
		console.log("Recalculating TOC headings positions");

		this.headingsCalculated = [];
		for (let i = 0; i < headingElements.length; i++) {
			this.headingsCalculated.push(this.calculateHeading(i, this.headingsCalculated));
		}
	}

	/**
	 * @param {number} i
	 * @returns {boolean}
	 */
	isActive(i) {
		const {start, end} = this.headingsCalculated[i];
		const top = scrollY + this.getScrollOffset();
		return top >= start && top <= end;
	}

	/**
	 * @param {number} i
	 * @param {boolean} activating
	 */
	toggleActive(i, activating) {
		const {link, titles} = this.headingsCalculated[i];

		if (activating) {
			document.title = `${titles.join(": ")} — ${originalTitle}`;
		} else {
			document.title = originalTitle;
		}

		/** @type {null | Element} */
		let target = link;
		while (target != null && target.tagName !== "DIV") {
			if (target.tagName === "LI") {
				target.classList.toggle("active");
			}
			target = target.parentElement;
		}
	}

	checkActive() {
		if (this.lastActiveHeading !== undefined) {
			if (this.isActive(this.lastActiveHeading)) {
				return;
			} else {
				this.toggleActive(this.lastActiveHeading, false);
				this.lastActiveHeading = undefined;
			}
		}

		for (let i = 0; i < this.headingsCalculated.length; i++) {
			if (this.isActive(i)) {
				// Set the heading as active
				this.lastActiveHeading = i;
				this.toggleActive(i, true);

				// Make sure TOC link is visible
				let linkTop = this.headingsCalculated[i].link.offsetTop - tocContainer.offsetTop;
				if (i === 0) {
					linkTop = 0;
				}
				const visibleStart = tocContainer.scrollTop;
				const visibleEnd = tocContainer.scrollTop + tocContainer.clientHeight;
				const isVisible = linkTop > visibleStart && linkTop < visibleEnd;
				if (!isVisible) {
					tocContainer.scrollTop =  linkTop;
				}

				break;
			}
		}
	}

	/**
	 * @param {string} hash
	 */
	scrollToHeading(hash) {
		const heading = document.getElementById(hash.replace(/^(#)/, ""));
		if (!heading) {
			return;
		}

		heading.setAttribute("tabindex", "-1");
		heading.focus();

		window.scrollTo(0, this.getHeadingTop(heading));
		this.checkActive();
	}

	onResize() {
		// In order to decide if we need to recalculate all the headings, we only need to compare the last one
		const i = headingElements.length - 1;
		const existing = this.headingsCalculated[i];
		const recalculated = this.calculateHeading(i, []);
		if (recalculated.start !== existing.start || recalculated.end !== existing.end) {
			this.calculateHeadingsPositions();
		}
	}

	attach() {
		this.calculateHeadingsPositions();
		this.checkActive();

		if (window.location.hash !== "") {
			this.scrollToHeading(window.location.hash);
		}

		tocList.addEventListener("click", this.handleTOCClick.bind(this), false);
		window.addEventListener("resize", this.onResize.bind(this), {capture: false, passive: true});
		window.addEventListener("scroll", this.checkActive.bind(this), {capture: false, passive: true});
		window.addEventListener("resize", this.checkActive.bind(this), {capture: false, passive: true});

		document.addEventListener(
			"click",
			(event) => {
				if (!(event.target instanceof HTMLElement)) {
					return;
				}

				if (!event.target.matches(".header-anchor")) {
					return;
				}

				event.preventDefault();

				const hash = event.target.getAttribute("href");
				window.location.hash = hash;
				this.scrollToHeading(hash);
			},
			false,
		);
	}
}

const toc = new TableOfContents();
toc.attach();
