# NameDivider API

Python以外の環境からNameDividerを使えるようにするためにREST APIをホストするDockerイメージを提供しています。

https://hub.docker.com/r/rskmoi/namedivider-api

## Version History

- **0.1.0**: Python版(FastAPI)で、BasicNameDividerのみの提供
- **0.2.0-beta**: Rust(actix-web)で実装、BasicNameDivider + GBDTNameDivider対応
- **0.3.0**: lightgbm-rs改善版統合、Rustが安定したと判断しlatestにする

## Installation

```
docker pull rskmoi/namedivider-api:0.3.0
```

## Usage

- Run Docker Image

```
docker run -d --rm -p 8000:8000 rskmoi/namedivider-api:0.3.0
```

- Send HTTP request

### BasicNameDividerを使う場合

```
curl -X POST -H "Content-Type: application/json" -d '{"names":["竈門炭治郎", "竈門禰豆子"]}' localhost:8000/divide
```

or

```
curl -X POST -H "Content-Type: application/json" -d '{"names":["竈門炭治郎", "竈門禰豆子"], "mode": "basic"}' localhost:8000/divide
```

### GBDTNameDividerを使う場合

```
curl -X POST -H "Content-Type: application/json" -d '{"names":["竈門炭治郎", "竈門禰豆子"], "mode": "gbdt"}' localhost:8000/divide
```

- Response

```
{
    "divided_names":
        [
            {"family":"竈門","given":"炭治郎","separator":" ","score":0.3004587452426102,"algorithm":"kanji_feature"},
            {"family":"竈門","given":"禰豆子","separator":" ","score":0.30480429696983175,"algorithm":"kanji_feature"}
        ]
}
```

## Notice

- `names` は姓名が分割されていない名前のリストで、1リクエストで受け付ける上限は1000個です

## Client Samples

各プログラミング言語向けのSDK風サンプル実装を[client-samples/](./client-samples/)ディレクトリで提供しています。

### 対応言語

- [TypeScript/JavaScript](./client-samples/typescript/)
- [Python](./client-samples/python/)
- [Go](./client-samples/go/)
- [PHP](./client-samples/php/)
- [Ruby](./client-samples/ruby/)
- [C#](./client-samples/csharp/)
- [Kotlin](./client-samples/kotlin/)

各言語で使いやすいクライアントライブラリとサンプルコードを提供しており、専用メソッド（`divideBasic()`, `divideGbdt()`）と汎用メソッド（`divide(mode)`）の両方をサポートしています。詳細な使用方法は各言語のREADMEを参照してください。