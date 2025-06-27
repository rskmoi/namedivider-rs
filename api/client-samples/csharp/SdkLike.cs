using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace NameDivider.Api.Client
{
    /// <summary>
    /// 分割された名前の結果を表すレコード
    /// </summary>
    public record DividedName(
        string Family,
        string Given,
        string Separator,
        double Score,
        string Algorithm)
    {
        public override string ToString()
        {
            return $"DividedName(Family='{Family}', Given='{Given}', Score={Score:F4}, Algorithm='{Algorithm}')";
        }
    }

    /// <summary>
    /// API リクエストのペイロード
    /// </summary>
    public record DivideRequest(List<string> Names, string Mode = "basic");

    /// <summary>
    /// API レスポンスの構造
    /// </summary>
    public record DivideResponse([property: JsonPropertyName("divided_names")] List<DividedName> DividedNames);

    /// <summary>
    /// NameDivider API C# SDK
    /// 
    /// このクラスは、NameDivider APIのC# SDKの実装です。
    /// 基本的な使用方法から、異なるアルゴリズムモードの使い分けまで対応しています。
    /// </summary>
    public class NameDividerClient : IDisposable
    {
        private readonly HttpClient _httpClient;
        private readonly string _baseUrl;
        private readonly JsonSerializerOptions _jsonOptions;

        /// <summary>
        /// クライアントを初期化
        /// </summary>
        /// <param name="baseUrl">APIのベースURL</param>
        /// <param name="httpClient">HTTPクライアント（オプション）</param>
        public NameDividerClient(string baseUrl = "http://localhost:8000", HttpClient? httpClient = null)
        {
            _baseUrl = baseUrl;
            _httpClient = httpClient ?? new HttpClient();
            _jsonOptions = new JsonSerializerOptions
            {
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                PropertyNameCaseInsensitive = true
            };
        }

        /// <summary>
        /// 名前を分割する
        /// </summary>
        /// <param name="names">分割する名前のリスト</param>
        /// <param name="mode">分割モード ('basic' または 'gbdt')</param>
        /// <returns>分割結果のリスト</returns>
        /// <exception cref="ArgumentException">引数が無効な場合</exception>
        /// <exception cref="HttpRequestException">HTTP通信エラーの場合</exception>
        /// <exception cref="JsonException">JSONパースエラーの場合</exception>
        public async Task<List<DividedName>> DivideAsync(List<string> names, string mode = "basic")
        {
            if (names == null || names.Count == 0)
                throw new ArgumentException("Names cannot be null or empty", nameof(names));

            if (mode != "basic" && mode != "gbdt")
                throw new ArgumentException("Mode must be 'basic' or 'gbdt'", nameof(mode));

            var request = new DivideRequest(names, mode);
            var jsonRequest = JsonSerializer.Serialize(request, _jsonOptions);
            var content = new StringContent(jsonRequest, Encoding.UTF8, "application/json");

            try
            {
                var response = await _httpClient.PostAsync($"{_baseUrl}/divide", content);
                response.EnsureSuccessStatusCode();

                var jsonResponse = await response.Content.ReadAsStringAsync();
                var divideResponse = JsonSerializer.Deserialize<DivideResponse>(jsonResponse, _jsonOptions);

                return divideResponse?.DividedNames ?? throw new JsonException("Invalid response format");
            }
            catch (HttpRequestException ex)
            {
                throw new HttpRequestException($"API request failed: {ex.Message}", ex);
            }
            catch (JsonException ex)
            {
                throw new JsonException($"Failed to parse response: {ex.Message}", ex);
            }
        }

        /// <summary>
        /// BasicNameDividerを使用して名前を分割
        /// </summary>
        /// <param name="names">分割する名前のリスト</param>
        /// <returns>分割結果のリスト</returns>
        public async Task<List<DividedName>> DivideBasicAsync(List<string> names)
        {
            return await DivideAsync(names, "basic");
        }

        /// <summary>
        /// GBDTNameDividerを使用して名前を分割
        /// </summary>
        /// <param name="names">分割する名前のリスト</param>
        /// <returns>分割結果のリスト</returns>
        public async Task<List<DividedName>> DivideGbdtAsync(List<string> names)
        {
            return await DivideAsync(names, "gbdt");
        }

        /// <summary>
        /// リソースを解放
        /// </summary>
        public void Dispose()
        {
            _httpClient?.Dispose();
        }
    }
}