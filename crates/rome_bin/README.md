# `rome_bin`

Rome's main binary distribution, exposes the command line interface defined in
`rome_cli`, and the language server interface defined in `rome_lsp` and used by
the `rome` VSCode extension

# Logs

When the server is run in daemon mode, it will output logs to a file created in
a `rome-logs` directory inside the system temporary directory. The log file
will be rotated on a hourly basis.
