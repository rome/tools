<pre class="language-text homepage-example collapsed collapsable"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">src/App.jsx:6:5</span> <strong>lint/js/doubleEquals</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>==</strong></span><span style="color: Tomato;">.</span>

  <strong>  5</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">App</span><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">username</span><span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>  <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">username</span> <span class="token operator">==</span> <span class="token string">&quot;sebmck&quot;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">== is only allowed when comparing against null.</span>

  <strong>Suggested fix:</strong> Use ===

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">username</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">==</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">&quot;sebmck&quot;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">username</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">==</span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">&quot;sebmck&quot;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This may be unsafe if you are relying on type coercion</span>
 </code><button aria-hidden="true" class="expand">Click to Expand</button></pre>
