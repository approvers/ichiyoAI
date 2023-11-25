# ichiyoAI

[![Build ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml)
[![clippy](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml)
[![Release ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml)

ichiyoAI は限界開発鯖で使用できるチャットAIです.

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

設定の例は [.env.example](./.env.example) で確認できます.

| Key                 | Description       | required | default  |
|---------------------|-------------------|----------| -------- |
| `DISCORD_API_TOKEN` | Discord API のトークン | `Yes`    | ---     |
| `OPENAI_API_KEY`    | OpenAI API のトークン  | `Yes`    | ---     |
| `GUILD_ID`          | 限界開発鯖の ID         | `Yes`    | ---     |
| `TAXPAYER_ROLE_ID`  | 購読者ロールの ID        | `Yes`     | ---     |
| `LOG_ENVIRONMENT` | ログの出力レベル切り替え. | `No` | `production` |

### `LOG_ENVIRONMENT`

環境変数 `LOG_ENVIRONMENT` はログの出力レベルを切り替えます.

- `production`: 本番環境向け. ログレベル `INFO` 以上のログを出力します.
- `development`: デバック環境向け. ログレベル `DEBUG` 以上のログを出力します.

デフォルトでは `production` が設定されています. `development` に切り替えると, Serenity や ichiyoAI 本体のデバッグログが出力されます.
