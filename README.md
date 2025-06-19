# namedivider-rs

## 2025-06-19

現在作業中のためREADMEと実態が合っていない状態が数日続く予定です。作業が終わると以下の状態になる予定です。
- namedivider-rs v0.2.0がリリースされる
- namedivider-api v0.3.0がリリースされる
- PythonラッパーがPython3.9-3.13、linux/mac/windowsで動作する

## About

姓名が連結している日本語の名前を姓と名に分割するライブラリ**NameDivider**のrust実装です。

[python実装](https://github.com/rskmoi/namedivider-python/blob/master/README.md)より最大200倍ほど高速化しています。

cliで名前1つを分割するときBasicNameDividerで3ms,GBDTNameDividerで15ms程度です。 

lightgbmのモデルをPythonからRustに移植した結果、完璧に同じ結果を再現できず若干こちらのほうが精度が低いです。

こちらの実装はスーパーアルファ版で、後方互換性・保守性など全く考えていません。

また、ビルドに必要なfamily_names.txtというファイルもこのgithubでは管理していません(権利を保有する企業に公開の確認を取っていないため)。

そのためこのリポジトリはビルドできない状態です。

(このfamily_names.txtはGBDTNameDividerにのみ必要なファイルなので、ソースコードを少しいじってBasicNameDividerのみビルドすることは可能です)

## Python

Python版をnamedivider-rsのPythonラッパーにするという可能性があり、その練習としてPythonライブラリとして使えるようにwheelファイルを公開しています。

Python版より数十倍オーダーで速いです。

### v0.2.0での改善点

- lightgbm-rsの改善により、自動的なLGBM関数bindingが可能になりました
- バッチ処理機能（`divide_names`メソッド）を追加しました
- エラーハンドリングとパッケージ名の一貫性を改善しました
- namedivider-pythonとの統合がより安定しました

### インストール

現在はLinux (x64) Python 3.11用のwheelのみ提供しています：

```bash
pip install https://github.com/rskmoi/namedivider-rs/releases/download/v0.2.0/namedivider_rust-0.1.0-cp311-cp311-linux_x86_64.whl
```

ライセンスはpython実装と同じです。他のプラットフォームやPythonバージョン用のwheelは順次提供予定です。
