/**
 * NameDivider API C# SDK サンプルコード
 */

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NameDivider.Api.Client;

namespace NameDivider.Api.Sample
{
    /// <summary>
    /// NameDivider API SDK の使用例
    /// </summary>
    public class Program
    {
        public static async Task Main(string[] args)
        {
            using var client = new NameDividerClient("http://localhost:8000");
            var testNames = new List<string> { "原敬", "菅義偉", "安倍晋三", "中曽根康弘" };

            try
            {
                // 専用メソッド DivideBasicAsync() を使用
                var basicResults = await client.DivideBasicAsync(testNames);
                Console.WriteLine("BasicNameDivider Results:");
                for (int i = 0; i < basicResults.Count; i++)
                {
                    var result = basicResults[i];
                    Console.WriteLine($"{i + 1}. {testNames[i]} -> {result.Family} {result.Given} (score: {result.Score:F4})");
                }

                // 専用メソッド DivideGbdtAsync() を使用
                Console.WriteLine("\nGBDTNameDivider Results:");
                var gbdtResults = await client.DivideGbdtAsync(testNames);
                for (int i = 0; i < gbdtResults.Count; i++)
                {
                    var result = gbdtResults[i];
                    Console.WriteLine($"{i + 1}. {testNames[i]} -> {result.Family} {result.Given} (score: {result.Score:F4})");
                }

                // 汎用メソッド DivideAsync() とオプション指定
                Console.WriteLine("\n汎用メソッド使用例:");
                var basicOptionResults = await client.DivideAsync(testNames, "basic");
                var gbdtOptionResults = await client.DivideAsync(testNames, "gbdt");
                Console.WriteLine($"Basic結果例: {basicOptionResults[0].Family} {basicOptionResults[0].Given}");
                Console.WriteLine($"GBDT結果例: {gbdtOptionResults[0].Family} {gbdtOptionResults[0].Given}");

                // 単体の名前を処理
                var singleResult = await client.DivideBasicAsync(new List<string> { "小泉純一郎" });
                Console.WriteLine($"\n単体処理: 小泉純一郎 -> {singleResult[0].Family} {singleResult[0].Given}");

                // 並列処理
                Console.WriteLine("\n並列処理例:");
                var parallelNames = new List<string> { "田中角栄", "吉田茂" };
                var tasks = new[]
                {
                    client.DivideAsync(parallelNames, "basic"),
                    client.DivideAsync(parallelNames, "gbdt")
                };
                var results = await Task.WhenAll(tasks);
                var parallelBasic = results[0];
                var parallelGbdt = results[1];
                Console.WriteLine($"{parallelNames[0]}: Basic={parallelBasic[0].Family} {parallelBasic[0].Given}, GBDT={parallelGbdt[0].Family} {parallelGbdt[0].Given}");

                // LINQ を使った処理例
                Console.WriteLine("\nLINQ処理例:");
                var authorNames = new List<string> { "池田勇人", "佐藤栄作", "福田赳夫" };
                var authorResults = await client.DivideGbdtAsync(authorNames);
                var highConfidence = authorResults
                    .Where(r => r.Score > 0.4)
                    .Select(r => $"{r.Family} {r.Given}")
                    .ToList();
                
                if (highConfidence.Any())
                {
                    Console.WriteLine($"高スコア結果: {string.Join(", ", highConfidence)}");
                }

                Console.WriteLine("\nサンプル実行完了!");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"エラーが発生しました: {ex.Message}");
                Console.WriteLine("NameDivider APIサーバーが起動していることを確認してください。");
                Console.WriteLine("起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple");
            }
        }
    }
}