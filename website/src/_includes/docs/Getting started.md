# Getting started

While Rome seeks to fill the role of many tools in the JavaScript ecosystem, it can be integrated into existing projects and used as much or as little as you like.

First, navigate into your project folder:

```bash
cd my_existing_project
```

Now, create a Rome configuration for your project. When prompted, it is advised to use the recommended settings:

```bash
rome init
```

This command creates a Rome configuration file, `rome.json`, which looks like this:

```json
{
  "version": "^0.0.52",
  "lint": {
    "enabled": true
  }
}
```

You're all set to get started with Rome. Continue reading to explore the variety of commands that Rome supports.