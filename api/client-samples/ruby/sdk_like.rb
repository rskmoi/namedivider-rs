# frozen_string_literal: true

require 'net/http'
require 'json'
require 'uri'

##
# 分割された名前の結果を表すクラス
class DividedName
  attr_reader :family, :given, :separator, :score, :algorithm

  def initialize(data)
    @family = data['family']
    @given = data['given']
    @separator = data['separator']
    @score = data['score']
    @algorithm = data['algorithm']
  end

  def to_s
    "DividedName(family='#{@family}', given='#{@given}', score=#{@score.round(4)}, algorithm='#{@algorithm}')"
  end

  def inspect
    to_s
  end
end

##
# NameDivider API Ruby SDK
# 
# このクラスは、NameDivider APIのRuby SDKの実装です。
# 基本的な使用方法から、異なるアルゴリズムモードの使い分けまで対応しています。
class NameDividerClient
  DEFAULT_BASE_URL = 'http://localhost:8000'
  DEFAULT_TIMEOUT = 30

  attr_reader :base_url, :timeout

  ##
  # クライアントを初期化
  # 
  # @param [String] base_url APIのベースURL
  # @param [Integer] timeout タイムアウト秒数
  def initialize(base_url = DEFAULT_BASE_URL, timeout: DEFAULT_TIMEOUT)
    @base_url = base_url
    @timeout = timeout
  end

  ##
  # 名前を分割する
  # 
  # @param [Array<String>] names 分割する名前の配列
  # @param [String] mode 分割モード ('basic' または 'gbdt')
  # @return [Array<DividedName>] 分割結果の配列
  # @raise [StandardError] APIエラーまたはネットワークエラーの場合
  def divide(names, mode = 'basic')
    raise ArgumentError, 'names must be an array' unless names.is_a?(Array)
    raise ArgumentError, 'names cannot be empty' if names.empty?
    raise ArgumentError, 'mode must be basic or gbdt' unless %w[basic gbdt].include?(mode)

    request_body = {
      names: names,
      mode: mode
    }.to_json

    uri = URI("#{@base_url}/divide")
    
    Net::HTTP.start(uri.hostname, uri.port, use_ssl: uri.scheme == 'https',
                    open_timeout: @timeout, read_timeout: @timeout) do |http|
      request = Net::HTTP::Post.new(uri)
      request['Content-Type'] = 'application/json'
      request.body = request_body

      response = http.request(request)

      unless response.is_a?(Net::HTTPSuccess)
        raise StandardError, "HTTP error: #{response.code} #{response.message}"
      end

      begin
        data = JSON.parse(response.body)
      rescue JSON::ParserError => e
        raise StandardError, "Invalid JSON response: #{e.message}"
      end

      unless data.key?('divided_names')
        raise StandardError, 'Invalid response format: missing divided_names'
      end

      data['divided_names'].map { |item| DividedName.new(item) }
    end
  end

  ##
  # BasicNameDividerを使用して名前を分割
  # 
  # @param [Array<String>] names 分割する名前の配列
  # @return [Array<DividedName>] 分割結果の配列
  def divide_basic(names)
    divide(names, 'basic')
  end

  ##
  # GBDTNameDividerを使用して名前を分割
  # 
  # @param [Array<String>] names 分割する名前の配列
  # @return [Array<DividedName>] 分割結果の配列
  def divide_gbdt(names)
    divide(names, 'gbdt')
  end
end