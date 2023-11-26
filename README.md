# ichiyoAI

[![Build ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/build.yaml)
[![clippy](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/clippy.yaml)
[![rustfmt](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/fmt.yaml)
[![Release ichiyoAI](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml/badge.svg)](https://github.com/approvers/ichiyoAI/actions/workflows/release.yaml)

ichiyoAI は OpenAI の [ChatGPT](https://openai.com/chatgpt) ・ [DALL-E](https://openai.com/dall-e-3) が利用できる Discord Bot です.

- [Features](#features)
  - [Coming Soon...](#coming-soon)
- [Usage](#usage)
  - [ChatGPT (Text Generation)](#chatgpt-text-generation)
  - [DALL-E (Image Generation)](#dall-e-image-generation)
- [Supported Models](#supported-models)
  - [GPT-4](#gpt-4)
  - [GPT-3.5](#gpt-35)
  - [DALL-E](#dall-e)
- [Installation](#installation)
- [Environment Variables](#environment-variables)

## Features

- LLM である **GPT-4 Turbo** & **GPT-3.5 Turbo** を使用した [Text Generation](https://platform.openai.com/docs/guides/text-generation)
- 深層学習モデルである **DALL·E 3** & **DALL·E 2** を使用した [Image Generation](https://platform.openai.com/docs/guides/images)

### Coming Soon...

- [ ] **GPT-4V** ( `gpt-4-vision-preview` ) を使用した [Vision](https://platform.openai.com/docs/guides/vision) への対応

## Usage

### ChatGPT (Text Generation)

ichiyoAI に接続している Discord Bot に対してメンションを送信すると, そのメッセージに対して ChatGPT が応答します.

ichiyoAI のメッセージに対してメンション付きで返信すると, 返信元のリプライまで ChatGPT が理解した上で応答します.

### DALL-E (Image Generation)

`!image` / `!davinci` コマンドを使用すると, DALL-E が画像を生成します.

第一引数にプロンプトを指定します. DALL-E 独自の倫理コードが存在し不適切なプロンプトは拒否されます.

## Supported Models

### GPT-4

GPT-3.5 を改良し, 自然言語やコードを理解・生成できる言語モデル. GPT-3.5 の **完全上位互換** .

| モデル名 | ichiyoAI の対応バージョン | Context Window | トレーニングデータ | Input | Output |
| --- | --- | --- | --- | --- | --- |
| gpt-4-1106-preview | v1.16.0 〜 | 128,000 Token | Apr 2023 | $0.01 / 1k | $0.03 / 1k |
| gpt-4-vision-preview | 未対応 | 128,000 Token | Apr 2023 | $0.01 / 1k | $0.03 / 1k |
| gpt-4 (Current: gpt-4-0613) | v1.5.0 〜 v1.15.4 | 8.192 Token | Sep 2021 | $0.03 / 1k | $0.06 / 1k |
| gpt-4-32k | 未対応 | 32,768 Token | Sep 2021 | $0.03 / 1k | $0.06 / 1k |

### GPT-3.5

GPT-3 を改良し, 自然言語やコードを理解・生成できる言語モデル.

| モデル名 | ichiyoAI の対応バージョン | Context Window | トレーニングデータ | Input | Output |
| --- | --- | --- | --- | --- | --- |
| gpt-3.5-turbo-1106 | v1.16.0 〜 | 16,385 Token | Sep 2021 | $0.0010 / 1k | $0.0020 / 1k |
| gpt-3.5-turbo (Current: gpt-3.5-turbo-0613) | 〜 v1.15.4 | 4,096 Token | Sep 2021 | $0.0015 / 1k | $0.002 / 1k |

### DALL-E

自然言語による記述からリアルな画像やアートを生成できる深層学習モデル.

> [!NOTE]
> 深層学習モデルは大量データを元に AI がルールやパターンを理解して学習させる手法です. DALL-E に GPT-4, GPT-3.5 のようなトレーニングの概念が存在しません.

| モデル名 | ichiyoAI の対応バージョン | Price (1024×1024) |
| --- | --- | --- |
| dall-e-3 | v1.17.0 〜 | $0.040 / image |
| dall-e-2 | v1.17.0 〜 | $0.020 / image |

## Installation

> [!NOTE]
> 限界開発鯖のメンバーは限界開発鯖内で利用できます.

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

| Key                 | Description       | required |
|---------------------|-------------------|----------|
| `DISCORD_API_TOKEN` | Discord API のトークン | `Yes`    |
| `OPENAI_API_KEY`    | OpenAI API のトークン  | `Yes`    |
| `GUILD_ID`          | 限界開発鯖の ID         | `Yes`    |
| `SPONSOR_ROLE_ID`   | 購読者ロールの ID        | `Yes`     |
