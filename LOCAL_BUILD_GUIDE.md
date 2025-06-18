# namedivider-rust ローカルビルドガイド

namedivider-rustを複数プラットフォーム・複数Python版向けにローカルでビルドする手順を説明します。

## 前提条件

### 必須ファイル
- `namedivider-rs/` ディレクトリに `family_names.txt` が配置されている必要があります
- このファイルには姓名データが含まれており、公開配布はできません

### システム要件
- Rust 1.75+ （本番環境との互換性のため）
- Python 3.9, 3.10, 3.11, 3.12, 3.13 （必要な分だけインストール）
- maturin （Rust拡張ビルド用Pythonパッケージ）

### 対応プラットフォーム
- Linux (Ubuntu 20.04+ で動作確認済み)
- macOS (macOS 11+ で動作確認済み)  
- Windows (Visual Studio Build Tools が必要)

## インストール手順

### 1. Rustのインストール
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup install 1.75.0
rustup default 1.75.0
```

### 2. 複数Python版のインストール
#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install python3.9 python3.10 python3.11 python3.12 python3.13
sudo apt install python3.9-dev python3.10-dev python3.11-dev python3.12-dev python3.13-dev
```

#### macOS (Homebrewを使用):
```bash
brew install python@3.9 python@3.10 python@3.11 python@3.12 python@3.13
```

#### Windows:
python.orgから各バージョンをダウンロード・インストール

### 3. maturinのインストール
```bash
# 各Python版に対して:
python3.9 -m pip install maturin
python3.10 -m pip install maturin
python3.11 -m pip install maturin
python3.12 -m pip install maturin
python3.13 -m pip install maturin
```

## Wheelのビルド

### オプション1: 自動マルチプラットフォームビルド
```bash
# 自動ビルドスクリプトの実行
./scripts/build_multiplatform_wheels.sh
```

このスクリプトは以下を実行します：
- `family_names.txt` の存在確認
- 利用可能な全Python版向けにwheelをビルド
- `dist/` ディレクトリにwheelを出力

### オプション2: 特定Python版向け手動ビルド
```bash
cd python/
python3.11 -m maturin build --release --out ../dist/
```

### オプション3: 開発ビルド（テスト用）
```bash
cd python/
python3.11 -m maturin develop
```

## ビルドしたWheelのテスト

### インストールとテスト
```bash
# wheelのインストール
python -m pip install --force-reinstall dist/namedivider_rust-0.2.0-*.whl

# 基本機能テスト
python -c "
import namedivider_rust
divider = namedivider_rust.BasicNameDivider()
result = divider.divide_name('田中太郎')
print(f'結果: {result}')
assert result == '田中 太郎'
print('✓ 基本テストが通りました')
"

# GBDT機能テスト
python -c "
import namedivider_rust
divider = namedivider_rust.GBDTNameDivider()
result = divider.divide_name('田中太郎')
print(f'GBDT結果: {result}')
assert result == '田中 太郎'
print('✓ GBDTテストが通りました')
"
```

### パフォーマンステスト
```bash
python -c "
import time
import namedivider_rust

# Basic dividerのパフォーマンス
divider = namedivider_rust.BasicNameDivider()
start = time.time()
for _ in range(1000):
    divider.divide_name('田中太郎')
basic_time = time.time() - start

print(f'Basic divider: 1000件で{basic_time:.3f}秒')
print(f'処理速度: {1000/basic_time:.0f} 件/秒')
"
```

## 期待される出力

### ビルド成功時
```
Building wheels for namedivider-rust v0.2.0
=== Building for Python 3.9 ===
✓ Successfully built wheel for Python 3.9
=== Building for Python 3.10 ===  
✓ Successfully built wheel for Python 3.10
...

=== Built wheels ===
namedivider_rust-0.2.0-cp39-cp39-linux_x86_64.whl
namedivider_rust-0.2.0-cp310-cp310-linux_x86_64.whl
...
```

### プラットフォーム別Wheel名
- **Linux**: `namedivider_rust-0.2.0-cp311-cp311-linux_x86_64.whl`
- **macOS**: `namedivider_rust-0.2.0-cp311-cp311-macosx_11_0_x86_64.whl`
- **Windows**: `namedivider_rust-0.2.0-cp311-cp311-win_amd64.whl`

## トラブルシューティング

### よくある問題

#### 1. `family_names.txt` が見つからない
```
Error: family_names.txt not found in namedivider-rs/ directory
```
**解決方法**: `namedivider-rs/` ディレクトリに family_names.txt ファイルを配置してください

#### 2. Python版が見つからない
```
Warning: Python 3.X not found, skipping...
```
**解決方法**: システムパッケージマネージャーで不足しているPython版をインストールしてください

#### 3. maturinが見つからない
```
✗ Failed to install maturin for Python 3.X
```
**解決方法**: そのPython版にmaturinをインストールしてください：
```bash
python3.X -m pip install maturin
```

#### 4. Rustコンパイルエラー
**解決方法**: Rust 1.75+を使用し、必要なシステム依存関係があることを確認してください：
```bash
# Ubuntu/Debian
sudo apt install build-essential cmake

# macOS  
xcode-select --install

# Windows
# Visual Studio Build Tools をインストール
```

## パフォーマンス期待値

改善されたlightgbm-rs統合によるベンチマーク結果：

- **単一名前処理**: 従来版より5%高速化
- **バッチ処理**: 従来版より14%高速化
- **GBDTモード**: モデル読み込みと推論で大幅改善

## 配布方法

ビルドされたwheelは以下のように配布できます：
- ユーザーへの手動配布
- PyPIへのアップロード（PyPIアカウントが必要）
- GitHubリリースでの共有
- ローカルでのテスト・開発使用

## セキュリティ注意事項

`family_names.txt` ファイルには機密データが含まれているため、以下は厳禁です：
- 公開リポジトリへのコミット
- 公開配布での同梱
- 適切な許可なしでの共有

このファイルは常に適切にセキュリティを確保し、認可されたビルドでのみ使用してください。