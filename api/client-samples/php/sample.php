<?php

/**
 * NameDivider API PHP SDK サンプルコード
 */

require_once 'SdkLike.php';

/**
 * NameDivider API SDK の使用例
 */
function main(): void
{
    $client = new NameDividerClient('http://localhost:8000');
    $testNames = ['原敬', '菅義偉', '安倍晋三', '中曽根康弘'];

    try {
        // 専用メソッド divideBasic() を使用
        $basicResults = $client->divideBasic($testNames);
        echo "BasicNameDivider Results:\n";
        foreach ($basicResults as $i => $result) {
            $index = $i + 1;
            echo "{$index}. {$testNames[$i]} -> {$result->family} {$result->given} (score: " . sprintf('%.4f', $result->score) . ")\n";
        }

        // 専用メソッド divideGbdt() を使用
        echo "\nGBDTNameDivider Results:\n";
        $gbdtResults = $client->divideGbdt($testNames);
        foreach ($gbdtResults as $i => $result) {
            $index = $i + 1;
            echo "{$index}. {$testNames[$i]} -> {$result->family} {$result->given} (score: " . sprintf('%.4f', $result->score) . ")\n";
        }

        // 汎用メソッド divide() とオプション指定
        echo "\n汎用メソッド使用例:\n";
        $basicOptionResults = $client->divide($testNames, 'basic');
        $gbdtOptionResults = $client->divide($testNames, 'gbdt');
        echo "Basic結果例: {$basicOptionResults[0]->family} {$basicOptionResults[0]->given}\n";
        echo "GBDT結果例: {$gbdtOptionResults[0]->family} {$gbdtOptionResults[0]->given}\n";

        // 単体の名前を処理
        $singleResult = $client->divideBasic(['小泉純一郎']);
        echo "\n単体処理: 小泉純一郎 -> {$singleResult[0]->family} {$singleResult[0]->given}\n";

        echo "\nサンプル実行完了!\n";

    } catch (Exception $error) {
        echo "エラーが発生しました: " . $error->getMessage() . "\n";
        echo "NameDivider APIサーバーが起動していることを確認してください。\n";
        echo "起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple\n";
    }
}

// メイン関数を実行
if (basename(__FILE__) === basename($_SERVER['SCRIPT_NAME'])) {
    main();
}