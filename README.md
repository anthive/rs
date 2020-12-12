# AntHive.IO sample bot in Rust

## [Import](https://github.com/new/import) this sample bot.

## Requirements
- Rust 
- Set your username in [ANTHIVE](ANTHIVE) file.
- Register your bot at https://profile.anthive.io/bots

## Debug
- git push origin master
- Register a new version of your bot from latest commits
- Start new game at https://profile.anthive.io/new-game
- Replay game step by step:
  - View logs
  - Download step payload

ANTHIVE

### Run locally (not required)
```
cargo run --release
```
It will start localhost server on port :7070

```
curl -X 'POST' -d @payload.json http://localhost:7070/run.php
```

### [Rules](https://anthive.io/rules/) and [Leaderboard](https://anthive.io/leaderboard/)
