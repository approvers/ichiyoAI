# Changelog

## [2.0.0](https://github.com/approvers/ichiyoAI/compare/v1.0.1...v2.0.0) (2023-08-11)


### ⚠ BREAKING CHANGES

* fix release-please-action
* fix env key
* remove limit

### Features

* action error handling ([1c6bee5](https://github.com/approvers/ichiyoAI/commit/1c6bee52bd4602202305eb853240af6298f7eeb4))
* add ChatGPT and Discord Action ([32b43d8](https://github.com/approvers/ichiyoAI/commit/32b43d8394377b4ff24c736b145d1a6e46cd6b10))
* add waiting mesage ([a5448ba](https://github.com/approvers/ichiyoAI/commit/a5448ba61e33e04c4debe539ee00894a2dfeb8af))
* added openai api logic ([7cbf94a](https://github.com/approvers/ichiyoAI/commit/7cbf94a1eec3f3a771e89142c6fa0d7e51aa647b))
* added reply mode ([0a56b23](https://github.com/approvers/ichiyoAI/commit/0a56b23df2a438d8ed55cc673ac3c8f9e7afdb25))
* added tokio timeout ([ce94bcf](https://github.com/approvers/ichiyoAI/commit/ce94bcfdd4e3fc2ec465ddc6f55185b3c6aaf193))
* block mention ([039a0f0](https://github.com/approvers/ichiyoAI/commit/039a0f018f4fc973867e417c957df261bcf1340b))
* change message detection logic ([86dc716](https://github.com/approvers/ichiyoAI/commit/86dc716072896710d385c4cbc5bc5a2c871eddc8))
* chat completion logic ([ecd2acf](https://github.com/approvers/ichiyoAI/commit/ecd2acf1172a8f19b7898df03ba3056caa94384c))
* create file utils ([a4809b2](https://github.com/approvers/ichiyoAI/commit/a4809b2f5f14c8dc138510479061ebe4fabd2839))
* create temp dir logic ([6e8121a](https://github.com/approvers/ichiyoAI/commit/6e8121a9f6edf0fdb78287b72f54c5a3686032ff))
* direct message display ([c31cbd8](https://github.com/approvers/ichiyoAI/commit/c31cbd80e00157f3abb272c50b9d588c8cb074a0))
* discord api login logic ([bfb1e27](https://github.com/approvers/ichiyoAI/commit/bfb1e27cc31fda9bcabc7462910cb11fe6715dbd))
* error handling ([2ec3899](https://github.com/approvers/ichiyoAI/commit/2ec389900f294065df84b4a779e6e6d95aec4f9e))
* error message display ([e481125](https://github.com/approvers/ichiyoAI/commit/e481125a8995f9507d9add095dfacc0b40cede62))
* fix release-please-action ([28175c2](https://github.com/approvers/ichiyoAI/commit/28175c252cf8ee2d23b098fb2c7719b15bd3f453))
* hibiki mode ([96827d1](https://github.com/approvers/ichiyoAI/commit/96827d15d6e5059a52b5433b6bc7268d6cc701d5))
* init logger ([d69c969](https://github.com/approvers/ichiyoAI/commit/d69c9696c9e81b24639699e2fbe34549375ec5e9))
* message limit bypass ([f5efd2e](https://github.com/approvers/ichiyoAI/commit/f5efd2ec09495a479d2eb7a1718389f1200a3d0e))
* remove limit ([b1c6b96](https://github.com/approvers/ichiyoAI/commit/b1c6b96594f42b9f87dbc69efb7de13218193085))
* replace logger ([4cf24c9](https://github.com/approvers/ichiyoAI/commit/4cf24c93bf8a5cca2312500b27c9fd9df087aef9))
* support chat directed ([f8ed746](https://github.com/approvers/ichiyoAI/commit/f8ed7463bc343b36dc3e50e272ffe40ce7f61e3f))
* support conversion history ([1bdd779](https://github.com/approvers/ichiyoAI/commit/1bdd7798ae534bfd8eafaca3e94e2ecc71a16adf))
* support direct command ([6ac5a9b](https://github.com/approvers/ichiyoAI/commit/6ac5a9b484645f21209792c1b81fb81d3b226f58))
* support GPT-4 model ([401cc01](https://github.com/approvers/ichiyoAI/commit/401cc0194a3e9fc6b35b62addbbd5ae339004af4))


### Bug Fixes

* added mention check ([e7ee3fb](https://github.com/approvers/ichiyoAI/commit/e7ee3fb046dca21a1ccf8a6f33cd4e059b14ab5b))
* dockerfile ([3d036d3](https://github.com/approvers/ichiyoAI/commit/3d036d3d65158b62ee5ae143e63f8763dd3f6d94))
* **docs:** command reference ([8f98a2d](https://github.com/approvers/ichiyoAI/commit/8f98a2dc56c3ef0808abe19af5cdb058e9a05913))
* fix release ci ([f7ece6d](https://github.com/approvers/ichiyoAI/commit/f7ece6db5bd45fea8f6e6bf6f9a90cc522066ab4))


### Miscellaneous Chores

* fix env key ([3b22c02](https://github.com/approvers/ichiyoAI/commit/3b22c02fd55462df99f2a9807de8673f723e727c))

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
