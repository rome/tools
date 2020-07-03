
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

## Lint rules documentation

The page with the list of rules is auto generated when `../script/assert-generated` is run. The script checks for rules inside `packages\@romejs\compiler\lint\rules` and generate a list with the rule names in kebab-case (eg. noFindDOMNode turns into no-find-dom-node).

The script also looks for [kebab-case-rule-name].md files inside `website\src\docs\check\rules`. If a file is found, a link for it will be created in the rules page (if there's not link, there's not documentation yet). The script also looks for a description key at the front matter that each .md file should have, if there's not description, it will warning you about that.

If a new rule was added and the script was not run, the Generated files check will fail.