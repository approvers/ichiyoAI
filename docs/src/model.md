# モデル

ichiyoAI は現在 ChatGPT, Gemini, DALL-E に対応しています.

- [LLM (大規模言語モデル)](#llm-大規模言語モデル)
  - [ChatGPT](#chatgpt)
    - [GPT-4](#gpt-4)
    - [GPT-3.5](#gpt-35)
  - [Gemini](#gemini)
- [深層学習モデル](#深層学習モデル)
  - [DALL-E](#dall-e)
- [パフォーマンス比較](#パフォーマンス比較)
  - [Gemini Ultra vs. GPT-4 (V)](#gemini-ultra-vs-gpt-4-v)
  - [Gemini Pro vs. GPT-3.5](#gemini-pro-vs-gpt-35)

## LLM (大規模言語モデル)

> 大規模言語モデルは、信憑性、理解度、範囲の全てにおいて限界があり、人間の監視が必要です。
> — *マイケル・オズボーン、オックスフォード大学機械学習教授 2023年1月25日*

LLMとは、人工ニューラルネットワークを使って文章やソースコードを生成する生成的モデル.

多数のパラメータ (数千万〜) を持つ人工ニュートラルネットワークを訓練することで, 人間のような文章を生成することができます.

### ChatGPT

**OpenAI** が開発するチャットボット (非対話型人工知能.)

#### GPT-4

GPT-3.5 を改良し, 自然言語やコードを理解・生成できる言語モデル. GPT-3.5 の **完全上位互換** .

**アクセスには限界税への納税が必要です.**

| モデル名 | ichiyoAI の対応バージョン | Context Window | トレーニングデータ | Input | Output |
| --- | --- | --- | --- | --- | --- |
| gpt-4-1106-preview | `v1.16.0` 〜 | 128,000 Token | Apr 2023 | $0.01 / 1k | $0.03 / 1k |
| gpt-4-vision-preview | 未対応(`*`) | 128,000 Token | Apr 2023 | $0.01 / 1k | $0.03 / 1k |
| gpt-4 (Current: gpt-4-0613) | `v1.5.0` 〜 `v1.15.4` | 8.192 Token | Sep 2021 | $0.03 / 1k | $0.06 / 1k |
| gpt-4-32k | 未対応(`*`) | 32,768 Token | Sep 2021 | $0.03 / 1k | $0.06 / 1k |

`*`: 対応予定なしのモデルです.

#### GPT-3.5

GPT-3 を改良し, 自然言語やコードを理解・生成できる言語モデル.

| モデル名 | ichiyoAI の対応バージョン | Context Window | トレーニングデータ | Input | Output |
| --- | --- | --- | --- | --- | --- |
| gpt-3.5-turbo-1106 | `v1.16.0` 〜 | 16,385 Token | Sep 2021 | $0.0010 / 1k | $0.0020 / 1k |
| gpt-3.5-turbo (Current: gpt-3.5-turbo-0613) | 〜 `v1.15.4` | 4,096 Token | Sep 2021 | $0.0015 / 1k | $0.002 / 1k |

### Gemini

Google が2014年に買収したイギリスにある Alphabet の人工知能子会社 **DeepMind Technologies** が開発したマルチモーダル大規模言語モデル.

LaMDA, PaLM2 の後継として供し, Gemini Ultra, Gemini Pro, Gemini Nano からなり, [GPT-4](#gpt-4) のライバルとして位置づけられている.

- Gemini Ultra は GPT-4, Gemini Pro は GPT-3.5 がライバルとしてそれぞれ位置づけられています.

| モデル名 | ichiyoAI の対応バージョン | Input | Output | Price |
| --- | --- | --- | --- | --- |
| Gemini Pro | `v2.0.0` 〜 | Text | Text | Free |
| Gemini Pro Vision | 未対応(`*`) | Text and Image | Text | Free |

- Gemini Ultra, Gemini Nano は Preview access 状態のため, ichiyoAI は未対応です.
  - 利用できるようになり次第対応予定です.
- PaLM や Embedding, Retrieval への対応は予定されていません.
- `*`: 対応予定なしのモデルです.

## 深層学習モデル

> ディープラーニング（英: deep learning）または深層学習（しんそうがくしゅう）とは、対象の全体像から細部までの各々の粒度の概念を階層構造として関連させて学習する手法のことである

### DALL-E

**OpenAI** が開発する画像生成モデル.

`prompts` と呼ばれる自然言語の記述からデジタル画像を生成することができます.

**DALL-E 3 のアクセスには限界税への納税が必要です.**

| モデル名 | ichiyoAI の対応バージョン | Price (1024×1024) |
| --- | --- | --- |
| dall-e-3 | `v1.17.0` 〜 | $0.040 / image |
| dall-e-2 | `v1.17.0` 〜 | $0.020 / image |

## パフォーマンス比較

以下の Performance benchmarks は Google AI for Developers から引用しています.

### Gemini Ultra vs. GPT-4 (V)

| 能力 | ベンチマーク | Gemini Ultra | GPT-4 (V) |
| --- | --- | --- | --- |
| 全般 | **MMLU** 57科目 (STEM, 人文科学など) の代表質問 | **90.0 %** | 86.4 % |
| 推論 | **Big-Bench Hard** 多段階の推論を必要とする難易度の高い多様なタスク | **83.6 %** | 83.1 % |
| 推論 | **DROP** 読解力 | **82.4** | 80.9 |
| 推論 | **HellaSwag** 日常業務における常識的な推論 | 87.8 % | **95.3 %** |
| 数学 | **GSM8K** 基本的な算数 (小学生の算数問題) | **94.4 %** | 92.0 % |
| 数学 | **MATH** 難易度の高い数学問題 (代数, 幾何, 微積分) | **53.2 %** | 52.9 % |
| コード | **HumanEval** Python コードの生成 | **74.4 %** | 67.0 % |
| コード | **Natural2Code** Python コードの生成. HumanEval のような新しいデータセット. | **74.9 %** | 73.9 % |

### Gemini Pro vs. GPT-3.5

| 能力 | ベンチマーク | Gemini Ultra | GPT-4 (V) |
| --- | --- | --- | --- |
| 全般 | **MMLU** 57科目 (STEM, 人文科学など) の代表質問 | **79.1 %** | 70.0 % |
| 推論 | **Big-Bench Hard** 多段階の推論を必要とする難易度の高い多様なタスク | **75.0 %** | 66.6 % |
| 推論 | **DROP** 読解力 | **74.1** | 64.1 |
| 推論 | **HellaSwag** 日常業務における常識的な推論 | 84.7 % | **85.5 %** |
| 数学 | **GSM8K** 基本的な算数 (小学生の算数問題) | **86.5 %** | 57.1 % |
| 数学 | **MATH** 難易度の高い数学問題 (代数, 幾何, 微積分) | 32.6 % | **34.1 %** |
| コード | **HumanEval** Python コードの生成 | **67.7 %** | 48.1 % |
| コード | **Natural2Code** Python コードの生成. HumanEval のような新しいデータセット. | **69.6 %** | 62.3 % |
