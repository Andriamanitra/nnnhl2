# NNNHL 2

No-Nonsense NHL schedule & highlights, now with 98% less JS/TS, 100% more backend, and 100% more Rust.

Unlike [its predecessor](https://github.com/Andriamanitra/nnnhl) which was just a static website, nnnhl2 is a small web *server* because the new NHL API uses CORS to prevent other websites from using it.
In the new system the API requests are made on the server side, and the HTML is fully server-side rendered as well (using [maud](https://github.com/lambda-fairy/maud) templating language which I recommend checking out, it's really sweet).


## Developing

The project is built using `cargo` (you will need to [install Rust](https://www.rust-lang.org/tools/install) to build it yourself):
```
cargo run
```
By default the server will listen on port 8002, but that can be configured by setting the `$HOST` environment variable, eg. `HOST=127.0.0.1:8080 cargo run` to have it listen on localhost port 8080.

I recommend installing [`browser-sync`](https://github.com/Browsersync/browser-sync), [`entr`](https://github.com/eradman/entr), and [`just`](https://github.com/casey/just) so you can automatically build the project & auto-refresh the browser while developing with `just`:
```
just watch
```


## Running inside Docker

If you don't want to bother with installing Rust and building `nnnhl2` yourself, you may use a pre-built docker container (you may also use `docker` in place of `podman`):
```sh
podman run --rm --init --name nnnhl2 -p 8002:8002 ghcr.io/andriamanitra/nnnhl2:latest
```
This starts the server at http://127.0.0.1:8002/


## dev notes

API types still missing for:
* [Stats](https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md#stats) for players, skaters, goalies, teams
* [Rosters](https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md#rosters)
* [Game details](https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md#game-details)
* [Player search](https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md#player-search)

(it would be nice to get them all generated & cleaned up and publish it as a separate crate)
