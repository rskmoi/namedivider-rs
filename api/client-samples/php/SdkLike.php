<?php

/**
 * NameDivider API PHP SDK
 * 
 * このクラスは、NameDivider APIのPHP SDKの実装です。
 * 基本的な使用方法から、異なるアルゴリズムモードの使い分けまで対応しています。
 */

/**
 * 分割された名前の結果を表すクラス
 */
class DividedName
{
    public string $family;
    public string $given;
    public string $separator;
    public float $score;
    public string $algorithm;

    public function __construct(array $data)
    {
        $this->family = $data['family'];
        $this->given = $data['given'];
        $this->separator = $data['separator'];
        $this->score = $data['score'];
        $this->algorithm = $data['algorithm'];
    }

    public function __toString(): string
    {
        return sprintf(
            "DividedName(family='%s', given='%s', score=%.4f, algorithm='%s')",
            $this->family,
            $this->given,
            $this->score,
            $this->algorithm
        );
    }
}

/**
 * NameDivider API クライアント
 */
class NameDividerClient
{
    private string $baseUrl;
    private array $curlOptions;

    /**
     * コンストラクタ
     * 
     * @param string $baseUrl APIのベースURL
     */
    public function __construct(string $baseUrl = 'http://localhost:8000')
    {
        $this->baseUrl = $baseUrl;
        $this->curlOptions = [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_HTTPHEADER => ['Content-Type: application/json'],
            CURLOPT_TIMEOUT => 30,
            CURLOPT_CONNECTTIMEOUT => 10,
        ];
    }

    /**
     * 名前を分割する
     * 
     * @param array<string> $names 分割する名前の配列
     * @param string $mode 分割モード ('basic' または 'gbdt')
     * @return array<DividedName> 分割結果の配列
     * @throws Exception APIエラーまたはネットワークエラーの場合
     */
    public function divide(array $names, string $mode = 'basic'): array
    {
        $request = [
            'names' => $names,
            'mode' => $mode
        ];

        $ch = curl_init();
        curl_setopt_array($ch, $this->curlOptions + [
            CURLOPT_URL => $this->baseUrl . '/divide',
            CURLOPT_POST => true,
            CURLOPT_POSTFIELDS => json_encode($request),
        ]);

        $response = curl_exec($ch);
        $httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        $error = curl_error($ch);
        curl_close($ch);

        if ($response === false) {
            throw new Exception("cURL error: $error");
        }

        if ($httpCode !== 200) {
            throw new Exception("HTTP error: $httpCode");
        }

        $data = json_decode($response, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new Exception('Invalid JSON response: ' . json_last_error_msg());
        }

        if (!isset($data['divided_names'])) {
            throw new Exception('Invalid response format');
        }

        return array_map(
            fn($item) => new DividedName($item),
            $data['divided_names']
        );
    }

    /**
     * BasicNameDividerを使用して名前を分割
     * 
     * @param array<string> $names 分割する名前の配列
     * @return array<DividedName> 分割結果の配列
     */
    public function divideBasic(array $names): array
    {
        return $this->divide($names, 'basic');
    }

    /**
     * GBDTNameDividerを使用して名前を分割
     * 
     * @param array<string> $names 分割する名前の配列
     * @return array<DividedName> 分割結果の配列
     */
    public function divideGbdt(array $names): array
    {
        return $this->divide($names, 'gbdt');
    }
}