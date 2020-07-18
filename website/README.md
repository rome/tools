# [`romefrontend.dev`](https://romefrontend.dev/)

## Installation

```
$ npm install
```

## Local Development

```
$ npm start
```

This command starts a local development server. Most changes are reflected live without having to restart the server.

## Build

```
$ npm build
```

This command generates static content into the build directory and can be served using any static contents hosting service.

## Blog

All posts should be inside `website/src/blog/posts` in markdown format. The post url slug will be the same as the file name. All posts should have these keys in the front matter: `title`, `descritpion`, `author` and `tags`. The post will only be listed if it have the tag `post`. Others tags can be added and pages for those tags will be auto generated. `date` on the front matter is optional, if not set, the date of the file creation will be used.
