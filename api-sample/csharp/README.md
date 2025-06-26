# NameDivider API C# SDK Sample

C#/.NET向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `SdkLike.cs`: SDK風のクライアント実装
- `Sample.cs`: SDKを使用するサンプルコード
- `NameDividerSample.csproj`: プロジェクト設定

## セットアップ

```bash
# 依存関係の復元
dotnet restore

# ビルド
dotnet build
```

## 使用方法

```bash
dotnet run
```

## SDK使用例

```csharp
using NameDivider.Api.Client;

// using文でリソース自動管理
using var client = new NameDividerClient("http://localhost:8000");

var testNames = new List<string> { "原敬", "菅義偉" };

// BasicNameDividerを使用
var basicResults = await client.DivideBasicAsync(testNames);

// GBDTNameDividerを使用
var gbdtResults = await client.DivideGbdtAsync(testNames);

// または直接modeを指定
var results = await client.DivideAsync(testNames, "gbdt");

foreach (var result in results)
{
    Console.WriteLine($"{result.Family} {result.Given} (score: {result.Score:F4})");
}

// LINQ を使った関数型プログラミング
var highConfidence = results
    .Where(r => r.Score > 0.5)
    .Select(r => $"{r.Family} {r.Given}")
    .ToList();

// 並列処理
var tasks = new[]
{
    client.DivideAsync(testNames, "basic"),
    client.DivideAsync(testNames, "gbdt")
};
var allResults = await Task.WhenAll(tasks);
```

## 特徴

- **モダンC#**: レコード型、パターンマッチング、switch式
- **非同期処理**: async/await とTask.WhenAll
- **型安全**: null許容参照型とSystem.Text.Json
- **LINQ**: 関数型プログラミング的なデータ操作
- **リソース管理**: IDisposableとusing文

## 前提条件

- .NET 6.0以上
- NameDivider APIサーバーが起動していること

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```