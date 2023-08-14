# Changelog

## [1.4.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.3.0...ichiyo_ai-v1.4.0) (2023-08-14)


### Features

* アクティビティ欄にバージョンを表示するように ([#58](https://github.com/approvers/ichiyoAI/issues/58)) ([f81d055](https://github.com/approvers/ichiyoAI/commit/f81d0555a600eadfb67d63a4ed4a33b71084252e))


### Bug Fixes

* **ci:** higuchi-ichiyo の制御用 AppID を指定する ([#55](https://github.com/approvers/ichiyoAI/issues/55)) ([7a49d4d](https://github.com/approvers/ichiyoAI/commit/7a49d4d50ecef7358cf712ae11dd5efe3d8cb5c5))

## [1.3.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.2.0...ichiyo_ai-v1.3.0) (2023-08-13)


### Features

* 返信のコンテキストを維持するように変更 ([#49](https://github.com/approvers/ichiyoAI/issues/49)) ([d97a047](https://github.com/approvers/ichiyoAI/commit/d97a04711bc24c3071d05fa1c4db797c48ac4762))

## [1.2.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.1.0...ichiyo_ai-v1.2.0) (2023-08-12)


### Features

* mitigation openai timeout ([a0cc9bc](https://github.com/approvers/ichiyoAI/commit/a0cc9bcf4a0ca766e6653d335e7ab2532120c29c))
* support sender name ([4a7e7e1](https://github.com/approvers/ichiyoAI/commit/4a7e7e110b6eea2269b328dd06ed1c00502224c0))

## [1.1.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.0.1...ichiyo_ai-v1.1.0) (2023-08-12)


### Features

* chat_mode and reply_mode using gpt-4 ([2027f1d](https://github.com/approvers/ichiyoAI/commit/2027f1df67c86f67003764e07b1efb9e66f6ae7b))
* ease timeout ([ee8a751](https://github.com/approvers/ichiyoAI/commit/ee8a7512f1325f051bdea58d5ed9cae9ed2c9e01))
* support multi model ([2b9c21c](https://github.com/approvers/ichiyoAI/commit/2b9c21cee93db89e4a879fdfeb5db122bd9724a7))

## [1.0.1](https://github.com/approvers/ichiyoAI/compare/v1.0.0...v1.0.1) (2023-08-11)


### Bug Fixes

* dockerfile ([3d036d3](https://github.com/approvers/ichiyoAI/commit/3d036d3d65158b62ee5ae143e63f8763dd3f6d94))
* fix release ci ([f7ece6d](https://github.com/approvers/ichiyoAI/commit/f7ece6db5bd45fea8f6e6bf6f9a90cc522066ab4))

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
