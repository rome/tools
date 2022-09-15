# Configuration
Currently, rome supports the following file extensions: `js`, `jsx`, `ts`, `tsx` and `d.ts`. 

Rome uses a file socket to connect the editor client, which may be different from other language servers using a binary e.g. `rust-analyzer`.

You can use `nc -U ${LANGUAGE_SERVER_SOCKET_PATH}` to connect to the Rome language server. `LANGUAGE_SERVER_SOCKET_PATH` is path to where the Rome's socket is created. Rome creates that socket inside the temporary folder of the operative system, inside a folder called `rome-socket`. 

To know the path of your OS, run the command:
```shell
rome __print_socket
```
More details why we need `nc`, please read the [wiki page of helix](https://github.com/helix-editor/helix/wiki/How-to-install-the-default-language-servers)

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
`/tmp/rome-socket` is the default socket file path in Linux. Use the command `rome __print_socket` and use the correct value.

# Limitation
1. The `rome-socket` will not automatically create when you reboot until you use the command `rome __print_socket`. As a workaround,
you could write a init startup bash script.

# Video record
## Code Action
https://user-images.githubusercontent.com/17974631/190205045-aeb86f87-1915-4d8b-8aad-2c046443ba83.mp4

## Formatting
https://user-images.githubusercontent.com/17974631/190205065-ddfde866-5f7c-4f53-8a62-b6cbb577982f.mp4