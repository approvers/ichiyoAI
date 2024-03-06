# Changelog

## [2.4.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.4.0...ichiyo_ai-v2.4.1) (2024-03-06)


### Bug Fixes

* **deps:** bump mio from 0.8.9 to 0.8.11 ([#215](https://github.com/approvers/ichiyoAI/issues/215)) ([42d9987](https://github.com/approvers/ichiyoAI/commit/42d9987c0014e1dac56a42f5fd8e88eab8f314b8))

## [2.4.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.3.1...ichiyo_ai-v2.4.0) (2024-02-11)


### Features

* update model `gpt-3.5-turbo-0125` ([#204](https://github.com/approvers/ichiyoAI/issues/204)) ([176d95d](https://github.com/approvers/ichiyoAI/commit/176d95d979a2588af503a9db0bbdbde1076349c5))


### Bug Fixes

* **docker:** release-please config `package-name` ([#207](https://github.com/approvers/ichiyoAI/issues/207)) ([3bf04e9](https://github.com/approvers/ichiyoAI/commit/3bf04e98506055c68799692c3d1bc481487f985f))

## [2.3.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.3.0...ichiyo_ai-v2.3.1) (2024-01-31)


### Bug Fixes

* fix some typos and grammar errors ([#199](https://github.com/approvers/ichiyoAI/issues/199)) ([72b18c5](https://github.com/approvers/ichiyoAI/commit/72b18c550740b31ed7f66f156aeda7bea929967a))

## [2.3.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.2.0...ichiyo_ai-v2.3.0) (2024-01-27)


### Features

* GPT-4 Turbo `gpt-4-0125-preview` のサポート ([#195](https://github.com/approvers/ichiyoAI/issues/195)) ([87dc83e](https://github.com/approvers/ichiyoAI/commit/87dc83e2f0e5b9a3428cd87e3666faeec52c9c31))

## [2.2.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.1.1...ichiyo_ai-v2.2.0) (2024-01-21)


### Features

* **bin:** DALL-E, ChatGPT のリクエストタイムアウトを緩和 ([#193](https://github.com/approvers/ichiyoAI/issues/193)) ([3486e64](https://github.com/approvers/ichiyoAI/commit/3486e649fc8eaa9bc084ecb1a8990816e612af3f))

## [2.1.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.1.0...ichiyo_ai-v2.1.1) (2024-01-19)


### Bug Fixes

* **deps:** bump h2 from 0.3.21 to 0.3.24 ([#191](https://github.com/approvers/ichiyoAI/issues/191)) ([6714886](https://github.com/approvers/ichiyoAI/commit/671488608064a19693d5e0cf19ef0e80901aa4be))

## [2.1.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v2.0.0...ichiyo_ai-v2.1.0) (2024-01-11)


### Features

* **lib:** Google AI 関係のエラー報告機能を強化 ([#189](https://github.com/approvers/ichiyoAI/issues/189)) ([791addf](https://github.com/approvers/ichiyoAI/commit/791addfcf1744944bfdda53a903139cad8cb4dfd))

## [2.0.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.21.0...ichiyo_ai-v2.0.0) (2024-01-08)


### ⚠ BREAKING CHANGES

ichiyoAI v2.0.0 では Gemini への対応等や Application Command への対応が行われました.

* 環境変数に `GOOGLE_AI_API_KEY`, `SENTRY_DSN` が追加されました.
  * `GOOGLE_AI_API_KEY` は指定しないと起動できません. [Google AI Studio](https://makersuite.google.com) で発行できます.
  * `SENTRY_DSN` は指定しなかった場合 Sentry によるエラー監視が行われません.
* ichiyoAI は起動時にギルドコマンドで Application Command を設定します.
  * 自分の環境で起動する場合は Bot に登録されたコマンドが上書きされても大丈夫か確認してからにしてください.
  * 限界開発鯖版はそのまま利用できます.

### Features

* v2 の実装 ([#185](https://github.com/approvers/ichiyoAI/issues/185)) ([4157da0](https://github.com/approvers/ichiyoAI/commit/4157da06c415caca2bb89b4c0aa91a37f84cc5e1))
* Text Generation が Message Command で利用できるようになりました.
  * これにより, 限界税に納税しているメンバーでも GPT-3.5 Turbo が利用できます.
  * メンションによる生成は廃止されました. 新しい利用方法については [ドキュメント](https://ichiyoai.approvers.dev/how-to/text-generation.html) を参照してください.
  * **重要:** 限界税を納税していない場合は `Text (GPT-4 Turbo)` コマンドは表示されません. (限界開発鯖版のみ)
* Image Generation が Application Command で利用できるようになりました.
  * これにより, 限界税に納税しているメンバーでも DALL-E 2 が利用できます.
  * `!image` による生成は廃止されました. 新しい利用方法については [ドキュメント](https://ichiyoai.approvers.dev/how-to/image-generation.html) を参照してください.
* Gemini Pro が利用できるようになりました.
  * Google が開発した GPT-3.5 Turbo を凌駕するマルチモーダル大規模言語モデルです.
  * Gemini Pro は GPT-3.5 Turbo のライバルとして位置づけられており, DeepMind Technologies によるベンチマークでは多くの測定において GPT-3.5 Turbo を上回っています.
  * 詳しいベンチマーク結果は [こちら](https://deepmind.google/technologies/gemini/#capabilities)

## [1.21.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.20.1...ichiyo_ai-v1.21.0) (2024-01-03)


### Features

* Sentry の有効化 ([#182](https://github.com/approvers/ichiyoAI/issues/182)) ([3b647f3](https://github.com/approvers/ichiyoAI/commit/3b647f32c7402f609a3adee1b472b643b69ad223))

## [1.20.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.20.0...ichiyo_ai-v1.20.1) (2024-01-02)


### Bug Fixes

* `serde_json` の "expected a borrowd string" のエラー ([#179](https://github.com/approvers/ichiyoAI/issues/179)) ([5cc69ce](https://github.com/approvers/ichiyoAI/commit/5cc69ce5496617e5b3200080242e597a067d262a))

## [1.20.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.19.1...ichiyo_ai-v1.20.0) (2024-01-02)


 [!NOTE]
 ichiyoAI v1 最後のリリースになります

### Features

* 内部で使う生成系 AI の API の wrapper を実装する ([#165](https://github.com/approvers/ichiyoAI/issues/165)) ([794c59f](https://github.com/approvers/ichiyoAI/commit/794c59f934f83f10ccab786b38b81c16fabac56e))
* 内部で使う画像生成系の AI の API の wrapper を実装する ([#177](https://github.com/approvers/ichiyoAI/issues/177)) ([5775763](https://github.com/approvers/ichiyoAI/commit/577576383f63f5bc5e594492a3e6eb432d71705d))
  * Text Generation, Image Generation は ichiyoAI:lib (内部Wrapper) に置き換わりました.
  * これにより, `async_openai` に依存しなくなりました.
  * 2つの変更は [@Nanai10a](https://github.com/Nanai10a) が実装しました. Thanks :heart:

## [1.19.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.19.0...ichiyo_ai-v1.19.1) (2023-12-09)


### Bug Fixes

* メンションを正しく取り除かれない不具合の修正 ([#158](https://github.com/approvers/ichiyoAI/issues/158)) ([63b844c](https://github.com/approvers/ichiyoAI/commit/63b844c88627200192275fbd730d8d7da001ac35))

## [1.19.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.18.0...ichiyo_ai-v1.19.0) (2023-12-02)


### Features

* support serenity v0.12 ([#155](https://github.com/approvers/ichiyoAI/issues/155)) ([257c375](https://github.com/approvers/ichiyoAI/commit/257c375e2059cfff44638825a5a540c7426b6cd4))

## [1.18.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.17.0...ichiyo_ai-v1.18.0) (2023-11-26)


### Features

* 埋め込みの DALL-E のモデル名表示 ([#152](https://github.com/approvers/ichiyoAI/issues/152)) ([08bfe87](https://github.com/approvers/ichiyoAI/commit/08bfe87ab9e17b7cfc8b27b8f58175fb3e7ce985))

## [1.17.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.16.0...ichiyo_ai-v1.17.0) (2023-11-25)


### Features

* async_openai v0.17.0 のサポート ([#148](https://github.com/approvers/ichiyoAI/issues/148)) ([b1b0b09](https://github.com/approvers/ichiyoAI/commit/b1b0b090993c3b99c0f7ddc4bb9b6a90d8e82f51))
* ImageGeneration 機能のサポート ([#151](https://github.com/approvers/ichiyoAI/issues/151)) ([5f88fb9](https://github.com/approvers/ichiyoAI/commit/5f88fb9ef15ae26cfda3464d9aabfb478d2cd857))

## [1.16.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.15.4...ichiyo_ai-v1.16.0) (2023-11-07)


### Features

* GPT-4 Turbo (gpt-4-1106-preview) のサポート ([#142](https://github.com/approvers/ichiyoAI/issues/142)) ([0cc7f5e](https://github.com/approvers/ichiyoAI/commit/0cc7f5ea23b5962cf01f75d179fb8ff71ad3dded))
* Updated GPT 3.5 Turbo (gpt-3.5-turbo-1106) のサポート ([#143](https://github.com/approvers/ichiyoAI/issues/143)) ([40c5300](https://github.com/approvers/ichiyoAI/commit/40c53000e4c378ea935cfab229042cd44af88083))
* デバックログの切り替えロジックを作成 ([#140](https://github.com/approvers/ichiyoAI/issues/140)) ([cfdecd1](https://github.com/approvers/ichiyoAI/commit/cfdecd19fdfaa226cc4ee563be081bcbc5afbc0f))
* メッセージフォーマットの改善 ([#144](https://github.com/approvers/ichiyoAI/issues/144)) ([8567760](https://github.com/approvers/ichiyoAI/commit/8567760efb44496cd4b9ac2094cd511f56e8f797))

## [1.15.4](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.15.3...ichiyo_ai-v1.15.4) (2023-10-28)


### Bug Fixes

* コンテキストが正しく維持されない問題を修正 ([#135](https://github.com/approvers/ichiyoAI/issues/135)) ([98c1a62](https://github.com/approvers/ichiyoAI/commit/98c1a62f8169ce1e79fa6b2d59f72bc541963185))

## [1.15.3](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.15.2...ichiyo_ai-v1.15.3) (2023-10-28)


### Bug Fixes

* 使用するトークン長が短くレスポンスが不完全になる問題の修正 ([#133](https://github.com/approvers/ichiyoAI/issues/133)) ([72c134f](https://github.com/approvers/ichiyoAI/commit/72c134f8979c3a4ac0ce8a5e17b302143f58d9a8))

## [1.15.2](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.15.1...ichiyo_ai-v1.15.2) (2023-10-28)


### Bug Fixes

* Docker Container 上で動作しない問題の修正 ([#131](https://github.com/approvers/ichiyoAI/issues/131)) ([458db6c](https://github.com/approvers/ichiyoAI/commit/458db6c3b2dd0d1282942b7d29ef7392c7615833))

## [1.15.1](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.15.0...ichiyo_ai-v1.15.1) (2023-10-28)


### Bug Fixes

* バージョン取得の方法を修正 ([#129](https://github.com/approvers/ichiyoAI/issues/129)) ([1531a4d](https://github.com/approvers/ichiyoAI/commit/1531a4dd0fb0d8d26a3e29582b808aa1786cc56b))

## [1.15.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.14.0...ichiyo_ai-v1.15.0) (2023-10-25)


### Features

* [rollback] release v1.13.0 (v1.14.0) ([#126](https://github.com/approvers/ichiyoAI/issues/126)) ([7c824ce](https://github.com/approvers/ichiyoAI/commit/7c824ce8bceee3a7ba662b2ffd01bde2f8b35562))
* v1.15.0 の強制リリース ([#128](https://github.com/approvers/ichiyoAI/issues/128)) ([9f0003d](https://github.com/approvers/ichiyoAI/commit/9f0003de4cf799f940042e04a58394762be39bc2))

## [1.14.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.13.0...ichiyo_ai-v1.14.0) (2023-10-23)


### Features

* ichiyoAI v1.13.0 ([#120](https://github.com/approvers/ichiyoAI/issues/120)) ([6c714a3](https://github.com/approvers/ichiyoAI/commit/6c714a3c08afa47c688a5927419e3a73b855b389))

## [1.13.0](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.12.2...ichiyo_ai-v1.13.0) (2023-10-23)


### Features

* recreate ChatGPT logic ([#122](https://github.com/approvers/ichiyoAI/issues/122)) ([699810b](https://github.com/approvers/ichiyoAI/commit/699810b0219b8febaee0063cba4ad982d4eca750))

## [1.12.2](https://github.com/approvers/ichiyoAI/compare/ichiyo_ai-v1.12.1...ichiyo_ai-v1.12.2) (2023-10-03)


### Bug Fixes

* 返信モードのコンテキストが5文字以上なのにエラーになる問題の修正 ([#112](https://github.com/approvers/ichiyoAI/issues/112)) ([152613e](https://github.com/approvers/ichiyoAI/commit/152613e6555b2fa86df16e3c8deaac0e0bcbaefb))

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
