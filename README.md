# namedivider-rs

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

Python版に比べると(1)全てのassetsをバイナリに埋め込んでいる(2)Rustによる計算速度向上の2点で速度向上しています。

```
pip install namedivider-0.1.0-cp38-cp38-manylinux_2_31_x86_64.whl
```

ライセンスはpython実装と同じです。 ニーズや作者の気分次第でWindows/Mac用のwheelも公開するかもしれません。
