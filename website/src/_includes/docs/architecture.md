## Architecture (in progress)

Rome uses a server-client architecture to run its tasks.

### Daemon

A [daemon](https://en.wikipedia.org/wiki/Daemon_(computing)) is a long-running server 
that Rome will spawn in the background, and will use it to process a series tasks as it sees fit.