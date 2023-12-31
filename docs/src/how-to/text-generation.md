# Text Generation

ichiyoAI は ChatGPT, Gemini を利用したテキスト生成に対応しています.

- [使用方法](#使用方法)
  - [Message Command](#message-command)
  - [メンション (Legacy)](#メンション-legacy)
- [返信モード](#返信モード)
- [注意事項](#注意事項)

## 使用方法

### Message Command

ichiyoAI に送信するプロンプトメッセージに対して Message Command を送信することで, テキスト生成を行うことができます.

Message Command は以下のように構成されています. 使用するモデルに合わせてコマンドを選んでください.

| モデル | コマンド |
| --- | --- |
| GPT-4 Turbo | `Text (GPT-4 Turbo)` |
| GPT-3.5 Turbo | `Text (GPT-3.5 Turbo)` |
| Gemini | `Text (Gemini)` |

### メンション (Legacy)

v1.19.0 以前の方法です. この方法は今後削除される可能性があります.

ichiyoAI にメンションを送信することで, テキスト生成を行うことができます. ただしこの方法は **GPT-4 Turbo, GPT-3.5 Turbo のみ対応しています**. Gemini は対応していません.

## 返信モード

ichiyoAI のメッセージに対してメンション付きで返信すると, 返信元のリプライまで ChatGPT, Gemini が理解した上で応答します.

## 注意事項

- OpenAI API 側からのレスポンスの文字数が 2000 文字を超えた場合, Discord API の仕様上 ichiyoAI は返信せずに終了します.
- OpenAI API へリクエストを送ってから3分以上かかった場合は tokio により自動的にタイムアウトになります。もう一度送るか、内容を見直してみてください。
