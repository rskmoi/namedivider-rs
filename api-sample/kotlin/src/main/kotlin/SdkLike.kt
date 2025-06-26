import kotlinx.coroutines.*
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse
import java.time.Duration

/**
 * 分割された名前の結果を表すデータクラス
 */
@Serializable
data class DividedName(
    val family: String,
    val given: String,
    val separator: String,
    val score: Double,
    val algorithm: String
) {
    override fun toString(): String {
        return "DividedName(family='$family', given='$given', score=${String.format("%.4f", score)}, algorithm='$algorithm')"
    }
}

/**
 * API リクエストのペイロード
 */
@Serializable
data class DivideRequest(
    val names: List<String>,
    val mode: String = "basic"
)

/**
 * API レスポンスの構造
 */
@Serializable
data class DivideResponse(
    @SerialName("divided_names")
    val dividedNames: List<DividedName>
)

/**
 * NameDivider API Kotlin SDK
 * 
 * このクラスは、NameDivider APIのKotlin SDKの実装です。
 * 基本的な使用方法から、異なるアルゴリズムモードの使い分けまで対応しています。
 * Kotlin Coroutinesを使用した非同期処理をサポートしています。
 */
class NameDividerClient(
    private val baseUrl: String = "http://localhost:8000",
    private val timeout: Duration = Duration.ofSeconds(30)
) {
    private val httpClient = HttpClient.newBuilder()
        .connectTimeout(timeout)
        .build()

    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
    }

    /**
     * 名前を分割する
     * 
     * @param names 分割する名前のリスト
     * @param mode 分割モード ('basic' または 'gbdt')
     * @return 分割結果のリスト
     * @throws IllegalArgumentException 引数が無効な場合
     * @throws Exception API通信エラーの場合
     */
    suspend fun divide(names: List<String>, mode: String = "basic"): List<DividedName> = withContext(Dispatchers.IO) {
        require(names.isNotEmpty()) { "Names cannot be empty" }
        require(mode in listOf("basic", "gbdt")) { "Mode must be 'basic' or 'gbdt'" }

        val request = DivideRequest(names, mode)
        val requestBody = json.encodeToString(request)

        val httpRequest = HttpRequest.newBuilder()
            .uri(URI.create("$baseUrl/divide"))
            .header("Content-Type", "application/json")
            .timeout(timeout)
            .POST(HttpRequest.BodyPublishers.ofString(requestBody))
            .build()

        val response = httpClient.send(httpRequest, HttpResponse.BodyHandlers.ofString())

        if (response.statusCode() !in 200..299) {
            throw Exception("HTTP error: ${response.statusCode()}")
        }

        try {
            val divideResponse = json.decodeFromString<DivideResponse>(response.body())
            divideResponse.dividedNames
        } catch (e: SerializationException) {
            throw Exception("Failed to parse response: ${e.message}", e)
        }
    }

    /**
     * BasicNameDividerを使用して名前を分割
     * 
     * @param names 分割する名前のリスト
     * @return 分割結果のリスト
     */
    suspend fun divideBasic(names: List<String>): List<DividedName> = divide(names, "basic")

    /**
     * GBDTNameDividerを使用して名前を分割
     * 
     * @param names 分割する名前のリスト
     * @return 分割結果のリスト
     */
    suspend fun divideGbdt(names: List<String>): List<DividedName> = divide(names, "gbdt")

    /**
     * 複数のモードを並列実行してアルゴリズムを比較
     * 
     * @param names 分割する名前のリスト
     * @return BasicとGBDTの結果をPairで返す
     */
    suspend fun compareAlgorithms(names: List<String>): Pair<List<DividedName>, List<DividedName>> = coroutineScope {
        val basicDeferred = async { divideBasic(names) }
        val gbdtDeferred = async { divideGbdt(names) }
        
        Pair(basicDeferred.await(), gbdtDeferred.await())
    }

    /**
     * バッチ処理用の高レベルAPI
     * 
     * @param namesList 複数の名前リスト
     * @param mode 分割モード
     * @param concurrency 並列実行数
     * @return 各リストの分割結果
     */
    suspend fun batchDivide(
        namesList: List<List<String>>, 
        mode: String = "basic",
        concurrency: Int = 3
    ): List<List<DividedName>> = coroutineScope {
        namesList.chunked(concurrency).flatMap { chunk ->
            chunk.map { names ->
                async { divide(names, mode) }
            }.awaitAll()
        }
    }

    /**
     * リソースを解放
     */
    fun close() {
        // HttpClient は自動でリソース管理されるため、特別な処理は不要
    }
}