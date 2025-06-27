# NameDivider API SDK Samples

NameDivider APIを各言語で利用するためのSDK風サンプル実装です。

## 概要

NameDivider APIは日本語の人名を姓と名に分割するREST APIです。各言語で使いやすいSDK風のクライアントライブラリとサンプルコードを提供しています。

## 対応言語

- [TypeScript/JavaScript](./typescript/) - Node.js環境向け
- [Python](./python/) - Python 3.6+向け
- [Go](./go/) - Go 1.20+向け
- [PHP](./php/) - PHP 8.0+向け
- [Ruby](./ruby/) - Ruby 2.7+向け
- [C#](./csharp/) - .NET 6.0+向け
- [Kotlin](./kotlin/) - JDK 11+向け

## API仕様

### エンドポイント

- `POST /divide` - 名前分割API

### リクエスト

```json
{
  "names": ["原敬", "菅義偉"],
  "mode": "basic"  // "basic" or "gbdt"
}
```

### レスポンス

```json
{
  "divided_names": [
    {
      "family": "原",
      "given": "敬", 
      "separator": " ",
      "score": 0.3004587452426102,
      "algorithm": "kanji_feature"
    }
  ]
}
```

## 使用方法

1. NameDivider APIサーバーを起動
```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```

2. 各言語のディレクトリに移動してサンプルを実行
```bash
# TypeScript
cd typescript && npm install && npm run dev

# Python  
cd python && pip install -r requirements.txt && python sample.py

# Go
cd go && go run *.go

# PHP
cd php && php sample.php

# Ruby
cd ruby && ruby sample.rb

# C#
cd csharp && dotnet run

# Kotlin
cd kotlin && ./gradlew run
```

## 分割モード

- **basic**: BasicNameDivider - 基本的な分割アルゴリズム
- **gbdt**: GBDTNameDivider - 機械学習ベースの高精度分割