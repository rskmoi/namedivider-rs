# NameDivider API Kotlin SDK Sample

Kotlin向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `SdkLike.kt`: SDK風のクライアント実装
- `Sample.kt`: SDKを使用するサンプルコード
- `build.gradle.kts`: Gradle設定（Kotlin DSL）
- `gradle.properties`: Gradle設定

## セットアップ

```bash
# 依存関係の解決とビルド
./gradlew build

# または直接実行
./gradlew run
```

## 使用方法

```bash
./gradlew run
```

## SDK使用例

```kotlin
import kotlinx.coroutines.*

suspend fun main() {
    val client = NameDividerClient("http://localhost:8000")
    
    val testNames = listOf("原敬", "菅義偉")
    
    try {
        // BasicNameDividerを使用
        val basicResults = client.divideBasic(testNames)
        
        // GBDTNameDividerを使用
        val gbdtResults = client.divideGbdt(testNames)
        
        // または直接modeを指定
        val results = client.divide(testNames, "gbdt")
        
        results.forEach { result ->
            println("${result.family} ${result.given} (score: ${String.format("%.4f", result.score)})")
        }
        
        // 並列処理
        val (basic, gbdt) = client.compareAlgorithms(testNames)
        
        // 関数型プログラミング
        val highConfidence = results
            .filter { it.score > 0.5 }
            .map { "${it.family} ${it.given}" }
            
        // 拡張関数の活用
        fun DividedName.analyze() = when {
            score > 0.8 -> "高精度"
            score > 0.5 -> "中精度"
            else -> "低精度"
        }
        
        results.forEach { result ->
            println("${result.family} ${result.given}: ${result.analyze()}")
        }
        
    } finally {
        client.close()
    }
}
```

## 特徴

- **Coroutines**: 非同期プログラミングとsuspend関数
- **Serialization**: kotlinx.serializationによる型安全なJSON処理
- **関数型**: filter, map, forEachなどの高階関数
- **拡張関数**: 既存クラスへのメソッド追加
- **データクラス**: 簡潔なデータ表現
- **When式**: パターンマッチング

## 依存関係

- Kotlin Coroutines: 非同期処理
- Kotlin Serialization: JSON シリアライゼーション

## 前提条件

- JDK 11以上
- NameDivider APIサーバーが起動していること

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```