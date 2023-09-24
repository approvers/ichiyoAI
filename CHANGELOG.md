# Changelog

## [1.12.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.12.0...ichiyo_ai-v1.12.1) (2023-09-24)


### Bug Fixes

* Docker Image のビルドに失敗する問題の修正 ([#109](https://github.com/approvers/ichiyoAI/issues/109)) ([5dd8a39](https://github.com/approvers/ichiyoAI/commit/5dd8a3917762330bddf9d3b7526f1f635a247bdb))

## [1.12.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.11.1...ichiyo_ai-v1.12.0) (2023-09-23)


### Features

* 5文字以上コンテキストがない会話は強制的に終了するように ([#106](https://github.com/approvers/ichiyoAI/issues/106)) ([e215af3](https://github.com/approvers/ichiyoAI/commit/e215af3f551a51670d71d1cdafca4a4ef66cf0f2))

## [1.11.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.11.0...ichiyo_ai-v1.11.1) (2023-09-17)


### Bug Fixes

* 利用料金表示の誤字を修正 ([#103](https://github.com/approvers/ichiyoAI/issues/103)) ([8c7964b](https://github.com/approvers/ichiyoAI/commit/8c7964bc2b175bdf43537c8a94264854ee1a3825))

## [1.11.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.10.0...ichiyo_ai-v1.11.0) (2023-09-16)


### Features

* 応答メッセージに合計トークン数を表示するように ([#102](https://github.com/approvers/ichiyoAI/issues/102)) ([19136a4](https://github.com/approvers/ichiyoAI/commit/19136a4a434c05d337c541b465926f54b9924d2d))


### Bug Fixes

* エラーメッセージがメンションされず送信される問題の修正 ([#100](https://github.com/approvers/ichiyoAI/issues/100)) ([e472107](https://github.com/approvers/ichiyoAI/commit/e472107043f2b6a79b190fc6ce8cdc0f21ab56ad))

## [1.10.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.9.2...ichiyo_ai-v1.10.0) (2023-08-27)


### Features

* 使用モデルの表示を追加 ([#89](https://github.com/approvers/ichiyoAI/issues/89)) ([96f8805](https://github.com/approvers/ichiyoAI/commit/96f8805c276ab3c83b631b3ac2dd463f000f1ab8))

## [1.9.2](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.9.1...ichiyo_ai-v1.9.2) (2023-08-26)


### Bug Fixes

* コンテキストが途中で欠如する問題の修正 ([#85](https://github.com/approvers/ichiyoAI/issues/85)) ([2fab02e](https://github.com/approvers/ichiyoAI/commit/2fab02e7897a3dc2c28f701f48faa1dd6c28c2ff))

## [1.9.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.9.0...ichiyo_ai-v1.9.1) (2023-08-26)


### Bug Fixes

* Docker コンテナ上のSSL認証エラーを修正 ([#83](https://github.com/approvers/ichiyoAI/issues/83)) ([f09d97d](https://github.com/approvers/ichiyoAI/commit/f09d97d2a9d64b8f47d4775598289cc233eeef17))

## [1.9.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.8.0...ichiyo_ai-v1.9.0) (2023-08-24)


### Features

* sentry のセットアップ ([#81](https://github.com/approvers/ichiyoAI/issues/81)) ([011c1bd](https://github.com/approvers/ichiyoAI/commit/011c1bd91e5b4caa5dc4472d4132510f83a01218))

## [1.8.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.7.0...ichiyo_ai-v1.8.0) (2023-08-20)


### Features

* レスポンスメッセージに利用料金の表示を追加 ([#72](https://github.com/approvers/ichiyoAI/issues/72)) ([2d2c0fc](https://github.com/approvers/ichiyoAI/commit/2d2c0fc5a81d6bf86ba794ae2c74f133df357c18))

## [1.7.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.6.0...ichiyo_ai-v1.7.0) (2023-08-17)


### Features

* 2000文字を超えないようにシステムコンテキストを設定する ([#65](https://github.com/approvers/ichiyoAI/issues/65)) ([e892315](https://github.com/approvers/ichiyoAI/commit/e892315991150d25fafcfc02c91415cfbcc5398d))

## [1.6.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.5.0...ichiyo_ai-v1.6.0) (2023-08-15)


### Features

* タイムアウト秒数を緩和 ([#63](https://github.com/approvers/ichiyoAI/issues/63)) ([225adc2](https://github.com/approvers/ichiyoAI/commit/225adc253e60e236a2ad5908b12bdeb47f0d1da6))

## [1.5.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.4.0...ichiyo_ai-v1.5.0) (2023-08-14)


### Features

* サブスクライバー限定で GPT-4 を解放 ([#59](https://github.com/approvers/ichiyoAI/issues/59)) ([f407fd7](https://github.com/approvers/ichiyoAI/commit/f407fd7c45ad38bed82e91553537b66badc226c0))


### Bug Fixes

* 返信時メンションしない不具合を修正 ([#61](https://github.com/approvers/ichiyoAI/issues/61)) ([92db0b1](https://github.com/approvers/ichiyoAI/commit/92db0b123cab54aef19acc15c87d3eaa946bd298))

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
