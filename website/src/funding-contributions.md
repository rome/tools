---
title: Funding Contributions
permalink: /funding/all-contributions/
layout: layouts/page.liquid
---

# All Public Contributions

<ul class="recent-contributions rows">
	<li>Loading...</li>
</ul>

<script>
  {% include scripts/funding-utils.js %}
  {% include scripts/recent-contributions.js %}

  fetch("{{ env.API_DOMAIN }}/funding/all").then(res => res.json()).then(data => {
	  setRecentContributions(data);
  });
</script>