"""
NameDivider API Python SDK サンプルコード

このスクリプトは、NameDivider APIのPython SDKの使用例を示します。
基本的な使用方法から、異なるアルゴリズムモードの使い分けまで、
実際の実装で参考になる例を含んでいます。
"""

from sdk_like import NameDividerClient


def main():
    """NameDivider API SDK の使用例"""
    
    # NameDividerクライアントを初期化
    # デフォルトではlocalhost:8000に接続します
    client = NameDividerClient("http://localhost:8000")
    
    # テスト用の日本語名前データ
    # 歴代総理大臣の名前を使用
    test_names = ["原敬", "菅義偉", "安倍晋三", "中曽根康弘"]
    
    try:
        # 専用メソッド divide_basic() を使用
        basic_results = client.divide_basic(test_names)
        print("BasicNameDivider Results:")
        for i, (name, result) in enumerate(zip(test_names, basic_results), 1):
            print(f"{i}. {name} -> {result.family} {result.given} (score: {result.score:.4f})")
        
        # 専用メソッド divide_gbdt() を使用
        print("\nGBDTNameDivider Results:")
        gbdt_results = client.divide_gbdt(test_names)
        for i, (name, result) in enumerate(zip(test_names, gbdt_results), 1):
            print(f"{i}. {name} -> {result.family} {result.given} (score: {result.score:.4f})")
        
        # 汎用メソッド divide() とオプション指定
        print("\n汎用メソッド使用例:")
        basic_option_results = client.divide(test_names, mode="basic")
        gbdt_option_results = client.divide(test_names, mode="gbdt")
        print(f"Basic結果例: {basic_option_results[0].family} {basic_option_results[0].given}")
        print(f"GBDT結果例: {gbdt_option_results[0].family} {gbdt_option_results[0].given}")
        
        # 単体の名前を処理
        single_result = client.divide_basic(["小泉純一郎"])
        print(f"\n単体処理: 小泉純一郎 -> {single_result[0].family} {single_result[0].given}")
        
        
    except Exception as error:
        print(f"エラーが発生しました: {error}")
        print("NameDivider APIサーバーが起動していることを確認してください。")
        print("起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple")


if __name__ == "__main__":
    main()