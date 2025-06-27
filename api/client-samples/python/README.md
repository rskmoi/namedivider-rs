# NameDivider API Python SDK Sample

Python向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `sdk_like.py`: SDK風のクライアント実装
- `sample.py`: SDKを使用するサンプルコード
- `requirements.txt`: 依存関係

## セットアップ

```bash
pip install -r requirements.txt
```

## 使用方法

```bash
python sample.py
```

## SDK使用例

```python
from sdk_like import NameDividerClient

client = NameDividerClient("http://localhost:8000")

# BasicNameDividerを使用
basic_results = client.divide_basic(["原敬", "菅義偉"])

# GBDTNameDividerを使用
gbdt_results = client.divide_gbdt(["原敬", "菅義偉"])

# または直接modeを指定
results = client.divide(["原敬", "菅義偉"], "gbdt")

# 結果の表示
for result in results:
    print(f"{result.family} {result.given} (score: {result.score:.4f})")
```

## 前提条件

NameDivider APIサーバーが起動している必要があります：

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```