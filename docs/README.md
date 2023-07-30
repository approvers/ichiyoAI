# How to use ichiyoAI 

## 使い方

### 会話モード

```
@ichiyoAI <メッセージ>
```

**例:**
> @ichiyoAI Rustのサンプルコードを提示してください。

- `@ichiyoAI` とメンションをつけてメッセージを送信すると ChatGPT と会話出来ます。
- メンションの位置は任意で、どこにつけても反応します。
- ChatGPT は Markdown 記法を一部理解します。
- 2000文字を超えたレスポンスは表示できません。
- 会話のコンテキストは維持されません。

### 指示モード

```
!direct <指示内容> <メッセージ>
```

**例:**
> !direct あなたは樋口一葉です。 
> 
> あなたの名前はなんですか？  
> あなたの好きな食べ物はなんですか？

- `!direct` or `!roleplay` では指示モードで ChatGPT と会話出来ます。
- 第1引数には指示内容を指定します。
- 第2引数にはメッセージを指定します。
  - 第2引数以降では改行等が使用できます。

## モデル

- 使用モデルは [`gpt-3.5`](https://platform.openai.com/docs/models/gpt-3-5) または [`gpt-4`](https://platform.openai.com/docs/models/gpt-4) です。
  - トレーニングデータは2021年9月です。
  - 詳細は [Models - OpenAI API](https://platform.openai.com/docs/models/overview) を確認してください。

## 
