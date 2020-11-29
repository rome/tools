---
title: Success!
permalink: /funding/checkout-complete/
layout: layouts/page.liquid
---

# Thank you!

Thank you for the support! I wouldn't be able to do this without you.

If you would like to update your shipping details, need a refund for any reason, or have any other questions, please feel free to get in touch with me at [sebastian@rome.tools](mailto:sebastian@rome.tools).

Interested in supporting us with a recurring donation? Subscribe on [OpenCollective](https://opencollective.com/rometools/).

<a class="button primary" id="tweet" href="#">Tweet your contribution</a>

&mdash; Sebastian McKenzie

<script>
  const tweetButton = document.querySelector("#tweet");
  const tier = localStorage.getItem("checkout-tier");
  const leading = tier == null ? "I just sponsored @rometools" : `I just sponsored @rometools as a ${tier}`;
  const tweetTemplate = `${leading}! Rome is a new open source JavaScript and web toolchain. Find out more and contribute at https://rome.tools/funding/`;
  tweetButton.href = `https://twitter.com/intent/tweet?text=${encodeURI(tweetTemplate)}`;
</script>