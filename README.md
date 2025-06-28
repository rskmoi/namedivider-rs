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

Python版をnamedivider-rsのPythonラッパーにするという可能性があり、namedivider-pythonのv0.4からOptionalではありますがこのRust実装をバックエンドとした高速化ができるようになっています。

`namedivider_core`という名前でパッケージ化・PyPIで配布しています。

- install
```
pip install namedivider-core
```

- usage
```
from namedivider_core import BasicNameDivider, GBDTNameDivider

# あとは基本機能はnamedivider-pythonと同様の挙動

from pprint import pprint

basic_divider = BasicNameDivider() # BasicNameDivider is fast but accuracy is 99.2%
divided_name = basic_divider.divide_name("菅義偉")

gbdt_divider = GBDTNameDivider() # GBDTNameDivider is slow but accuracy is 99.9%
divided_name = gbdt_divider.divide_name("菅義偉")

print(divided_name)
# 菅 義偉

pprint(divided_name.to_dict())
# {'algorithm': 'kanji_feature',
# 'family': '菅',
# 'given': '義偉',
# 'score': 0.7300634880343344,
# 'separator': ' '}
```


ライセンスはpython実装と同じです。


## CI/CD

このプロジェクトでは、GitHub Actionsを使用して全プラットフォーム対応のPythonホイールを自動ビルドしています。

### 対応環境
- **Python**: 3.9, 3.10, 3.11, 3.12, 3.13
- **プラットフォーム**: 
  - Linux: manylinux2014, musllinux_1_2 (x86_64)
  - Windows: win_amd64
  - macOS Intel: x86_64 (macOS 13.0以降)
  - macOS Apple Silicon: arm64 (macOS 14.0以降)
- **総計**: 25種類のホイール環境をサポート

詳細な技術情報については [ワークフローのドキュメント](.github/workflows/README.md) を参照してください。
