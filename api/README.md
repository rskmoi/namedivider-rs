# NameDivider API 0.2.0-beta

Python以外の環境からNameDividerを使えるようにするためにREST APIをホストするDockerイメージを提供しています。

https://hub.docker.com/r/rskmoi/namedivider-api

0.1.0はPython版(FastAPI)で、BasicNameDividerのみの提供でした。

0.2.0はRust(actix-web)で実装しており、BasicNameDividerに加えてGBDTNameDividerも使えるようになっています。

## Installation

```
docker pull rskmoi/namedivider-api:0.2.0-beta
```

## Usage

- Run Docker Image

```
docker run -d --rm -p 8000:8000 rskmoi/namedivider-api:0.2.0-beta
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