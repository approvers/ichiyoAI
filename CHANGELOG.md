# Changelog

## [1.0.0](https://github.com/approvers/ichiyoAI/compare/v0.5.2...v1.0.0) (2023-08-11)


### ⚠ BREAKING CHANGES

- 環境変数の Key が変更されました.
  - `CHATGPT_API_TOKEN` → `OPENAI_API_KEY`
- 以下の機能は廃止しました.
  - `!direct` (指示モード)
  - `!hibiki` (響モード)

### Features

全コードを書き直し. 仕様を見直しました.

- 思考中メッセージ (`waiting_message`) は廃止され、代わりに入力中(Typing)を使用するようになりました。
- 返信モードを追加しました。限定的ですが、会話のコンテキストを維持できます。
- OpenAI API からのレスポンスが15秒以上かかった場合は、返信せずエラーになるようになりました。
