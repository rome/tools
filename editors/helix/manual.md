# Configuration
Currently, rome support `js`, `jsx`, `ts`, `tsx` four languages. Rome uses a file socket to connect the editor client, 
which may be different from other language servers using a binary e.g. `rust-analyzer`.
you could use `nc -U ${LANGUAGE_SERVER_SOCKET_PATH}` to connect your rome language server, `LANGUAGE_SERVER_SOCKET_PATH` could be got by
`rome __print_socket`, by default it is `/tmp/rome-socket`. More details why we need nc you could reference https://github.com/helix-editor/helix/wiki/How-to-install-the-default-language-servers

**languages.toml**
```toml
[[language]]
name = "javascript"
scope = "source.js"
file-types = ["js"]
language-server = { command = "nc", args = ["-U", "/tmp/rome-socket"] }
formatter = { command = "rome", args = ["format", "--stdin-file-path", "test.js"]}
auto-format = true

[[language]]
name = "jsx"
scope = "source.jsx"
file-types = ["jsx"]
language-server = { command = "nc", args = ["-U", "/tmp/rome-socket"] }
formatter = { command = "rome", args = ["format", "--stdin-file-path", "test.jsx"]}
auto-format = true
[[language]]
name = "typescript"
scope = "source.ts"
file-types = ["ts"]
language-server = { command = "nc", args = ["-U", "/tmp/rome-socket"] }
formatter = { command = "rome", args = ["format", "--stdin-file-path", "test.ts"]}
auto-format = true

[[language]]
name = "tsx"
scope = "source.tsx"
file-types = ["tsx"]
language-server = { command = "nc", args = ["-U", "/tmp/rome-socket"] }
formatter = { command = "rome", args = ["format", "--stdin-file-path", "test.tsx"]}
auto-format = true

```

# Limitation
1. The `rome-socket` will not automatically create when you reboot until you use the command `rome __print_socket`. As a workaround,
you could write a init startup bash script.

# Video record
## Code Action
https://user-images.githubusercontent.com/17974631/190205045-aeb86f87-1915-4d8b-8aad-2c046443ba83.mp4

## Formatting
https://user-images.githubusercontent.com/17974631/190205065-ddfde866-5f7c-4f53-8a62-b6cbb577982f.mp4