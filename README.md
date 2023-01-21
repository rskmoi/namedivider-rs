# namedivider-rs

## About

姓名が連結している日本語の名前を姓と名に分割するライブラリ**NameDivider**のrust実装です。

[python実装](https://github.com/rskmoi/namedivider-python/blob/master/README.md)より最大300倍ほど高速化しています。

lightgbmのモデルをPythonからRustに移植した結果、完璧に同じ結果を再現できず若干こちらのほうが精度が低いです。

こちらの実装はスーパーアルファ版で、後方互換性・保守性など全く考えていません。

また、ビルドに必要なfamily_names.txtというファイルもこのgithubでは管理していません(権利を保有する企業に公開の確認を取っていないため)。

そのためこのリポジトリはビルドできない状態です。

## Python

Python版をnamedivider-rsのPythonラッパーにするという可能性があり、その練習としてPythonライブラリとして使えるようにwheelファイルを公開しています。

```
pip install namedivider-0.1.0-cp38-cp38-manylinux_2_31_x86_64.whl
```

ニーズや作者の気分次第でWindows/Mac用のwheelも公開するかもしれません。
