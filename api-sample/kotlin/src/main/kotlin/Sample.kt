/**
 * NameDivider API Kotlin SDK サンプルコード
 */

import kotlinx.coroutines.*

/**
 * NameDivider API SDK の使用例
 */
suspend fun main() {
    val client = NameDividerClient("http://localhost:8000")
    val testNames = listOf("原敬", "菅義偉", "安倍晋三", "中曽根康弘")

    try {
        // 専用メソッド divideBasic() を使用
        val basicResults = client.divideBasic(testNames)
        println("BasicNameDivider Results:")
        basicResults.forEachIndexed { index, result ->
            println("${index + 1}. ${testNames[index]} -> ${result.family} ${result.given} (score: ${String.format("%.4f", result.score)})")
        }

        // 専用メソッド divideGbdt() を使用
        println("\nGBDTNameDivider Results:")
        val gbdtResults = client.divideGbdt(testNames)
        gbdtResults.forEachIndexed { index, result ->
            println("${index + 1}. ${testNames[index]} -> ${result.family} ${result.given} (score: ${String.format("%.4f", result.score)})")
        }

        // 汎用メソッド divide() とオプション指定
        println("\n汎用メソッド使用例:")
        val basicOptionResults = client.divide(testNames, "basic")
        val gbdtOptionResults = client.divide(testNames, "gbdt")
        println("Basic結果例: ${basicOptionResults[0].family} ${basicOptionResults[0].given}")
        println("GBDT結果例: ${gbdtOptionResults[0].family} ${gbdtOptionResults[0].given}")

        // 単体の名前を処理
        val singleResult = client.divideBasic(listOf("小泉純一郎"))
        println("\n単体処理: 小泉純一郎 -> ${singleResult[0].family} ${singleResult[0].given}")

        // Coroutinesを使った並列処理
        println("\n並列処理例:")
        val parallelNames = listOf("田中角栄", "吉田茂")
        val (parallelBasic, parallelGbdt) = client.compareAlgorithms(parallelNames)
        println("${parallelNames[0]}: Basic=${parallelBasic[0].family} ${parallelBasic[0].given}, GBDT=${parallelGbdt[0].family} ${parallelGbdt[0].given}")

        // 関数型プログラミング例
        println("\nKotlin関数型処理例:")
        val authorNames = listOf("池田勇人", "佐藤栄作", "福田赳夫")
        val authorResults = client.divideGbdt(authorNames)
        val highConfidence = authorResults
            .filter { it.score > 0.4 }
            .map { "${it.family} ${it.given}" }

        if (highConfidence.isNotEmpty()) {
            println("高スコア結果: ${highConfidence.joinToString(", ")}")
        }

        println("\nサンプル実行完了!")

    } catch (e: Exception) {
        println("エラーが発生しました: ${e.message}")
        println("NameDivider APIサーバーが起動していることを確認してください。")
        println("起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple")
    } finally {
        client.close()
    }
}