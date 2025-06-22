# GitHub Actions ワークフロー

このディレクトリには、日本人姓名分割ツールのCI/CDワークフローが含まれています。

## 📋 ワークフロー一覧

### 1. `validate-config.yml` - 設定検証（軽量・高速）
**実行時間**: 2-3分  
**目的**: 基本的な設定とファイル整合性の検証

- family_names.txt の再構築テスト
- 依存関係の確認
- プラットフォーム固有の環境変数テスト
- Linux libclang検出テスト
- Windows GBDT処理テスト

**使用タイミング**: 
- PR開発中の迅速フィードバック
- 設定変更時の初期検証

### 2. `test-build.yml` - 単一環境テスト（デバッグ用）
**実行時間**: 10-15分  
**目的**: Ubuntu + Python 3.11 環境での実ビルドテスト

- 実際のPythonホイール作成
- 機能テスト（BasicNameDivider, GBDTNameDivider）
- manylinux環境での動作確認

**使用タイミング**:
- 新機能開発時のデバッグ
- ビルド問題の調査
- 修正内容の迅速検証

### 3. `build-all-wheels.yml` - 本番用統合ビルド
**実行時間**: 60-90分  
**目的**: 全30環境でのPythonホイール作成

**対象環境**:
- **Linux**: manylinux + musllinux（各5ホイール）
- **Windows**: Python 3.9-3.13（5ホイール）  
- **macOS Intel**: Python 3.9-3.13（5ホイール）
- **macOS Apple Silicon**: Python 3.9-3.13（5ホイール）
- **合計**: 30ホイール

**使用タイミング**:
- PR マージ前の最終検証
- リリース用ビルド
- 本番環境での動作確認

## 🔄 推奨ワークフロー

開発プロセスに応じて段階的に実行することを推奨します：

```
1. 開発中 → validate-config.yml (2-3分)
   ↓ 設定OK
2. デバッグ → test-build.yml (10-15分) 
   ↓ ビルドOK
3. 本番確認 → build-all-wheels.yml (60-90分)
   ↓ 全環境OK
4. マージ・リリース
```

## 🛠️ 技術的詳細

### Linux環境の特殊対応
- **manylinux**: CentOS 7ベース、llvm-toolset-7使用
- **musllinux**: Alpine Linuxベース、標準clang使用
- **LIBCLANG_PATH**: プラットフォーム自動検出で設定

### Windows環境の特殊対応
- **UTF-8エンコーディング**: GBDT modelファイルの文字化け対策
- **PowerShell処理**: バッククォートエスケープ対応
- **32bit除外**: `*-win32 *_i686` はスキップ

### macOS環境の特殊対応
- **OpenMP**: LightGBM用にlibomp自動インストール
- **環境変数**: LIBRARY_PATH, CPATH, LDFLAGS設定

## 🔧 保守・更新

### 定期的な見直し項目
- Rustバージョン（現在: 1.75.0）
- Pythonバージョン範囲（現在: 3.9-3.13）
- cibuildwheelバージョン（現在: v3.0.0）
- コンテナイメージ（manylinux2014, musllinux_1_2）

### 新しいプラットフォーム追加時
1. 新環境用のCIBW_ENVIRONMENT設定追加
2. 依存関係インストール手順の確認
3. テスト実行による動作確認
4. ドキュメント更新

---

**注意**: これらのワークフローは段階的に開発・最適化されたものです。変更時は必ず段階的テスト（validate → test-build → build-all）を実行してください。