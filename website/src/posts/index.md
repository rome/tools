---
title: Blog posts
layout: layouts/blog.njk
---
<div class="post-list">

  {%- for post in collections.post reversed -%}
    <article>
      <h1><a href="{{ post.url }}">{{ post.data.title }}</a></h1>
      <div class="info">
        <div>by {{ post.data.author }}</div>
        <time datetime="{{ post.date }}"> {{ post.date | dateFormat }} </time>
      </div>
      <p>{{ post.data.description }}</p>
    </article>
  {%- endfor -%}

</div>