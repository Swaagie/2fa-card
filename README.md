# 2fa-card

CLI to digitize the Lynx broker 2fa card.

## Install

```sh
cargo install
```

## Configure

Create a `~/.2fa-card.yaml` file in the home directory, where each number on the card matches an (ordered) entry

```yaml
---
- "JSD"
- "OPQ"
- "QAB"
- "ZSM"
- etc.
```

## Usage

```sh
card [number] [number]
```
