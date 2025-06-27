# NameDivider API Go SDK Sample

Go向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `sdk_like.go`: SDK風のクライアント実装
- `sample.go`: SDKを使用するサンプルコード
- `go.mod`: Goモジュール定義

## セットアップ

```bash
# 依存関係は標準ライブラリのみなので、特別なセットアップは不要
go mod tidy
```

## 使用方法

```bash
go run *.go
```

## SDK使用例

```go
package main

import "fmt"

func main() {
    client := NewNameDividerClient("http://localhost:8000")
    
    // BasicNameDividerを使用
    basicResults, err := client.DivideBasic([]string{"原敬", "菅義偉"})
    if err != nil {
        panic(err)
    }
    
    // GBDTNameDividerを使用
    gbdtResults, err := client.DivideGBDT([]string{"原敬", "菅義偉"})
    if err != nil {
        panic(err)
    }
    
    // または直接modeを指定
    results, err := client.Divide([]string{"原敬", "菅義偉"}, "gbdt")
    if err != nil {
        panic(err)
    }
    
    for _, result := range results {
        fmt.Printf("%s %s (score: %.4f)\n", result.Family, result.Given, result.Score)
    }
}
```

## 特徴

- **型安全**: Goの強力な型システムを活用
- **標準ライブラリ**: 外部依存なしで軽量
- **エラーハンドリング**: Goらしい明示的なエラー処理
- **構造体**: JSON自動マッピング対応

## 前提条件

- Go 1.23以上
- NameDivider APIサーバーが起動していること

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```