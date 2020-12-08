// global formatCurrency, setRecentContributions, humanizeRelativeTime
const recentContributions = document.querySelector("ul.recent-contributions");

function setRecentContributions(data) {
	recentContributions.textContent = "";

	if (data.length === 0) {
		const item = document.createElement("li");
		item.textContent = "No public contributions yet! Will you be the first?";
		recentContributions.appendChild(item);
	}

	for (const obj of data) {
		const item = document.createElement("li");
		recentContributions.appendChild(item);

		const details = document.createElement("div");
		details.classList.add("details");
		item.append(details);

		if (obj.github !== undefined) {
			const img = document.createElement("img");
			img.src = `https://avatars.githubusercontent.com/${obj.github}`;
			details.append(img);
		}

		const name = document.createElement("span");
		name.classList.add("name");
		name.textContent = obj.name === "" ? "Anonymous" : obj.name;
		details.append(name);

		if (obj.comment !== "") {
			const comment = document.createElement("div");
			comment.classList.add("quote");
			comment.textContent = `“${obj.comment}”`;
			item.append(comment);
		}

		const amount = document.createElement("div");
		amount.classList.add("amount");
		amount.textContent = formatCurrency(obj.amount);
		item.append(amount);

		const time = document.createElement("span");
		time.classList.add("time");
		time.textContent = humanizeRelativeTime(obj.time);
		item.append(time);
	}
}

setRecentContributions;
