# NameDivider API PHP SDK Sample

PHP向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `SdkLike.php`: SDK風のクライアント実装
- `sample.php`: SDKを使用するサンプルコード
- `composer.json`: 依存関係とプロジェクト設定

## セットアップ

```bash
# Composerを使用（オプション）
composer install

# または直接実行（外部依存なし）
php sample.php
```

## 使用方法

```bash
php sample.php
```

## SDK使用例

```php
<?php
require_once 'SdkLike.php';

$client = new NameDividerClient('http://localhost:8000');

// BasicNameDividerを使用
$basicResults = $client->divideBasic(['原敬', '菅義偉']);

// GBDTNameDividerを使用  
$gbdtResults = $client->divideGbdt(['原敬', '菅義偉']);

// または直接modeを指定
$results = $client->divide(['原敬', '菅義偉'], 'gbdt');

foreach ($results as $result) {
    echo "{$result->family} {$result->given} (score: " . sprintf('%.4f', $result->score) . ")\n";
}
?>
```

## 特徴

- **PHP 8.0+**: モダンPHPの機能を活用
- **型安全**: プロパティ型宣言とDocBlockコメント
- **クラス設計**: オブジェクト指向的なAPI設計
- **エラーハンドリング**: 例外ベースのエラー処理

## 必要な拡張

- `curl`: HTTP通信用
- `json`: JSONシリアライゼーション用

## 前提条件

- PHP 8.0以上
- NameDivider APIサーバーが起動していること

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```