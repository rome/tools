const lintRulesFolder = '../packages/@romejs/compiler/lint/rules';
const fs = require('fs');
const path = require("path");

const map = {};
let file = `---
title: Rome - A JavaScript toolchain
layout: layouts/base.njk
showHero: false
---

# Rules\n
`;

const categoryAlias = {
  js: 'Javascript',
  'jsx-a11y': 'JSX a11y',
  react: 'React',
  ts: 'TypeScript'
}

function nameFormat(fileName){

  fileName = fileName.replace('.ts', '');

  fileName = camelToKebab(fileName);

  return fileName;

}

function camelToKebab(string) {
  return string
      .replace(/([A-Z]{1})(?=[a-z])/g, '-$1')
      .replace(/([a-z]{1})(?=[A-Z]{2})/g, '$1-')
      .toLowerCase();
}

for (const category of fs.readdirSync(lintRulesFolder)) {
  const loc = path.join(lintRulesFolder, category);

	if (fs.statSync(loc).isFile()) {
		continue;
  }

  if(!map[category]) map[category] = [];

  for (const rule of fs.readdirSync(loc)) {
    if(rule.endsWith(".ts") && !rule.endsWith("test.ts")){
      map[category].push(nameFormat(rule));
    }
  }

}

for (const [category] of Object.entries(map)) {
  map[category] = map[category].sort((a, b) => {
    return a.localeCompare(b);
  });
}

function getDescription(content){
  const description = content.match(/description:(.*)/);
  if(description){
    return description[1];
  }

  return null;

}


for (const [category, value] of Object.entries(map)) {
let table = `
| Rule  | Description |
| ------------- | ------------- |
`;
  file += `## ${categoryAlias[category]}\n\n`;

  for (const rule of value) {

    const file = `./src/lint/rules/${rule}.md`;

    if(fs.existsSync(file)){

      const content = fs.readFileSync(file).toString();
      const description = getDescription(content);
      if(!description){
        console.log(`${file} is missing a description`);
      }

      table += `| [${rule}](/lint/rules/${rule}) | ${description || ''} |\n`;
    } else {
      table += `| ${rule} |  |\n`;
    }

  }

  file += table;

  file += `\n`;

}

fs.writeFile('./src/lint/rules/index.md', file, function (err,data) {
  if (err) {
    return console.log(err);
  }
});