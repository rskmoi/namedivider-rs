# NameDivider API Ruby SDK Sample

Ruby向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `sdk_like.rb`: SDK風のクライアント実装
- `sample.rb`: SDKを使用するサンプルコード
- `Gemfile`: 依存関係定義（オプション）

## セットアップ

```bash
# Bundlerを使用（オプション）
bundle install

# または直接実行（外部gemなし）
ruby sample.rb
```

## 使用方法

```bash
ruby sample.rb
# または
chmod +x sample.rb
./sample.rb
```

## SDK使用例

```ruby
require_relative 'sdk_like'

client = NameDividerClient.new('http://localhost:8000')

# BasicNameDividerを使用
basic_results = client.divide_basic(['原敬', '菅義偉'])

# GBDTNameDividerを使用
gbdt_results = client.divide_gbdt(['原敬', '菅義偉'])

# または直接modeを指定
results = client.divide(['原敬', '菅義偉'], 'gbdt')

results.each do |result|
  puts "#{result.family} #{result.given} (score: #{result.score.round(4)})"
end

# Rubyらしい関数型プログラミング
high_confidence = results
  .select { |r| r.score > 0.5 }
  .map { |r| "#{r.family} #{r.given}" }
```

## 特徴

- **Rubyらしい記法**: メソッドチェイニングと関数型プログラミング
- **標準ライブラリ**: 外部gemなしで軽量
- **並列処理**: Threadを使った並列API呼び出し
- **エラーハンドリング**: 例外ベースの明確なエラー処理
- **ドキュメント**: RDocスタイルのコメント

## 前提条件

- Ruby 2.7以上
- NameDivider APIサーバーが起動していること

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```