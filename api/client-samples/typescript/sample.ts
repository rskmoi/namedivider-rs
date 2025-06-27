/**
 * NameDivider API TypeScript SDK サンプルコード
 */

import { NameDividerClient, DividedName } from './sdk-like';

/**
 * NameDivider API SDK の使用例
 */
async function main(): Promise<void> {
  const client = new NameDividerClient('http://localhost:8000');
  const testNames: string[] = ['原敬', '菅義偉', '安倍晋三', '中曽根康弘'];

  try {
    // 専用メソッド divideBasic() を使用
    const basicResults: DividedName[] = await client.divideBasic(testNames);
    console.log('BasicNameDivider Results:');
    basicResults.forEach((result: DividedName, index: number) => {
      console.log(`${index + 1}. ${testNames[index]} -> ${result.family} ${result.given} (score: ${result.score.toFixed(4)})`);
    });

    // 専用メソッド divideGbdt() を使用
    console.log('\nGBDTNameDivider Results:');
    const gbdtResults: DividedName[] = await client.divideGbdt(testNames);
    gbdtResults.forEach((result: DividedName, index: number) => {
      console.log(`${index + 1}. ${testNames[index]} -> ${result.family} ${result.given} (score: ${result.score.toFixed(4)})`);
    });

    // 汎用メソッド divide() とオプション指定
    console.log('\n汎用メソッド使用例:');
    const basicOptionResults: DividedName[] = await client.divide(testNames, 'basic');
    const gbdtOptionResults: DividedName[] = await client.divide(testNames, 'gbdt');
    console.log(`Basic結果例: ${basicOptionResults[0].family} ${basicOptionResults[0].given}`);
    console.log(`GBDT結果例: ${gbdtOptionResults[0].family} ${gbdtOptionResults[0].given}`);

    // 単体の名前を処理
    const singleResult: DividedName[] = await client.divideBasic(['小泉純一郎']);
    console.log(`\n単体処理: 小泉純一郎 -> ${singleResult[0].family} ${singleResult[0].given}`);

    // Promise.allを使った並列処理
    console.log('\n並列処理例:');
    const parallelNames: string[] = ['田中角栄', '吉田茂'];
    const [parallelBasic, parallelGbdt] = await Promise.all([
      client.divide(parallelNames, 'basic'),
      client.divide(parallelNames, 'gbdt')
    ]);
    console.log(`${parallelNames[0]}: Basic=${parallelBasic[0].family} ${parallelBasic[0].given}, GBDT=${parallelGbdt[0].family} ${parallelGbdt[0].given}`);

  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error('エラーが発生しました:', error.message);
    } else {
      console.error('不明なエラーが発生しました:', error);
    }
    console.error('NameDivider APIサーバーが起動していることを確認してください。');
    console.error('起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple');
  }
}

// メイン関数を実行（Node.js環境での実行チェック）
if (require.main === module) {
  main().catch((error: unknown) => {
    console.error('Unhandled error:', error);
    process.exit(1);
  });
}