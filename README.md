# ichiyoAI

[![Build ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml)
[![clippy](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml)
[![Release ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml)

ichiyoAI は Discord 上で ChatGPI と会話ができる Discord Bot です。

2023/03/11に公開した DiscordGPT を Rust で再実装したものになります。

- [使い方](./docs/README.md)

----

- [x] 2000文字以上のレスポンスに対応する
- [ ] 会話のコンテキスト維持機能に対応する

## Installation

```shell
# 最新版
docker pull ghcr.io/approvers/ichiyo_ai:latest

# バージョン指定
docker pull ghcr.io/approvers/ichiyo_ai:vX.Y.Z

# ----
docker run --env-file=.env -d ichiyo_ai
```

## Environment Variables

設定の例は [.env.example](./.env.example) で確認できます。

| Key                 | Description       | Default |
|---------------------|-------------------|---------|
| `DISCORD_API_TOKEN` | Discord API のトークン | -       |
| `OPENAI_API_KEY` | OpenAI API のトークン  | -       |
