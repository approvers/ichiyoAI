# ichiyoAI

[![Build ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml)
[![clippy](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml)
[![Release ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml)

ichiyoAI は限界開発鯖で使用できるチャットAIです。

## Usage

使い方等は [ichiyoAI Document](./docs/README.md) を参照してください。

## Installation

```shell
# 最新版
docker pull ghcr.io/approvers/ichiyo_ai:latest

# メジャーバージョン指定
docker pull ghcr.io/approvers/ichiyo_ai:v1

# バージョン指定
docker pull ghcr.io/approvers/ichiyo_ai:vX.Y.Z
```

## Environment Variables

設定の例は [.env.example](./.env.example) で確認できます。

| Key                 | Description       | Default |
|---------------------|-------------------|---------|
| `DISCORD_API_TOKEN` | Discord API のトークン | -       |
| `OPENAI_API_KEY` | OpenAI API のトークン  | -       |
| `GUILD_ID` | 限界開発鯖の ID | -       |
| `SUBSCRIBER_ROLE_ID` | 購読者ロールの ID | -       |
