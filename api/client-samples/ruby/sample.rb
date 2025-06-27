#!/usr/bin/env ruby
# frozen_string_literal: true

##
# NameDivider API Ruby SDK サンプルコード

require_relative 'sdk_like'

##
# NameDivider API SDK の使用例
def main
  client = NameDividerClient.new('http://localhost:8000')
  test_names = %w[原敬 菅義偉 安倍晋三 中曽根康弘]

  begin
    # 専用メソッド divide_basic() を使用
    basic_results = client.divide_basic(test_names)
    puts 'BasicNameDivider Results:'
    basic_results.each_with_index do |result, i|
      puts "#{i + 1}. #{test_names[i]} -> #{result.family} #{result.given} (score: #{result.score.round(4)})"
    end

    # 専用メソッド divide_gbdt() を使用
    puts "\nGBDTNameDivider Results:"
    gbdt_results = client.divide_gbdt(test_names)
    gbdt_results.each_with_index do |result, i|
      puts "#{i + 1}. #{test_names[i]} -> #{result.family} #{result.given} (score: #{result.score.round(4)})"
    end

    # 汎用メソッド divide() とオプション指定
    puts "\n汎用メソッド使用例:"
    basic_option_results = client.divide(test_names, 'basic')
    gbdt_option_results = client.divide(test_names, 'gbdt')
    puts "Basic結果例: #{basic_option_results[0].family} #{basic_option_results[0].given}"
    puts "GBDT結果例: #{gbdt_option_results[0].family} #{gbdt_option_results[0].given}"

    # 単体の名前を処理
    single_result = client.divide_basic(['小泉純一郎'])
    puts "\n単体処理: 小泉純一郎 -> #{single_result[0].family} #{single_result[0].given}"

    # Rubyらしい関数型プログラミング例
    puts "\nRubyらしい処理例:"
    author_names = %w[田中角栄 吉田茂 池田勇人]
    author_results = client.divide_gbdt(author_names)
    
    # 高スコアの結果のみをフィルタリング
    high_confidence = author_results
                        .select { |result| result.score > 0.4 }
                        .map { |result| "#{result.family} #{result.given}" }
    
    puts "高スコア結果: #{high_confidence.join(', ')}" unless high_confidence.empty?

    puts "\nサンプル実行完了!"

  rescue StandardError => e
    puts "エラーが発生しました: #{e.message}"
    puts 'NameDivider APIサーバーが起動していることを確認してください。'
    puts '起動コマンド: docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple'
  end
end

# スクリプトが直接実行された場合のみmainを呼び出し
main if __FILE__ == $PROGRAM_NAME