/*
NameDivider API Go SDK サンプルコード
*/

package main

import (
	"fmt"
	"log"
)

/*
NameDivider API SDK の使用例
*/
func main() {
	client := NewNameDividerClient("http://localhost:8000")
	testNames := []string{"原敬", "菅義偉", "安倍晋三", "中曽根康弘"}

	// 専用メソッド DivideBasic() を使用
	basicResults, err := client.DivideBasic(testNames)
	if err != nil {
		log.Printf("BasicNameDivider エラー: %v", err)
		return
	}

	fmt.Println("BasicNameDivider Results:")
	for i, result := range basicResults {
		fmt.Printf("%d. %s -> %s %s (score: %.4f)\n", i+1, testNames[i], result.Family, result.Given, result.Score)
	}

	// 専用メソッド DivideGBDT() を使用
	fmt.Println("\nGBDTNameDivider Results:")
	gbdtResults, err := client.DivideGBDT(testNames)
	if err != nil {
		log.Printf("GBDTNameDivider エラー: %v", err)
		return
	}

	for i, result := range gbdtResults {
		fmt.Printf("%d. %s -> %s %s (score: %.4f)\n", i+1, testNames[i], result.Family, result.Given, result.Score)
	}

	// 汎用メソッド Divide() とオプション指定
	fmt.Println("\n汎用メソッド使用例:")
	basicOptionResults, err := client.Divide(testNames, "basic")
	if err != nil {
		log.Printf("基本モード エラー: %v", err)
		return
	}

	gbdtOptionResults, err := client.Divide(testNames, "gbdt")
	if err != nil {
		log.Printf("GBDTモード エラー: %v", err)
		return
	}

	fmt.Printf("Basic結果例: %s %s\n", basicOptionResults[0].Family, basicOptionResults[0].Given)
	fmt.Printf("GBDT結果例: %s %s\n", gbdtOptionResults[0].Family, gbdtOptionResults[0].Given)

	// 単体の名前を処理
	singleResult, err := client.DivideBasic([]string{"小泉純一郎"})
	if err != nil {
		log.Printf("単体処理 エラー: %v", err)
		return
	}

	fmt.Printf("\n単体処理: 小泉純一郎 -> %s %s\n", singleResult[0].Family, singleResult[0].Given)

	fmt.Println("\nサンプル実行完了!")
}