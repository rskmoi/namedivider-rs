# Next Action: プラットフォーム拡張と cibuildwheel 最新化

## 🎯 概要

現在の20環境対応ワークフローを30環境に拡張し、Python 3.13サポートとIntel Macサポートを追加する。

## 📊 現状分析

### 現在のビルド結果 (18ホイール)
```
✅ Linux (10ホイール):
- manylinux: Python 3.9, 3.10, 3.11, 3.12, 3.13 (5ホイール)
- musllinux:  Python 3.9, 3.10, 3.11, 3.12, 3.13 (5ホイール)

❌ Windows (4/5ホイール):
- Python 3.9, 3.10, 3.11, 3.12 (Python 3.13 未対応)

❌ macOS (4/5ホイール):  
- Apple Silicon (arm64): Python 3.9, 3.10, 3.11, 3.12 (Python 3.13 未対応)
- Intel (x86_64): 未対応
```

### 課題特定
1. **Python 3.13 未対応**: cibuildwheel v2.17 制限
2. **Intel Mac 未対応**: `macos-latest` = Apple Silicon のみ

## 🔧 解決策

### Task 1: cibuildwheel 最新化
**現在**: `cibuildwheel@v2.17.0` (Python 3.13 未サポート)
**更新**: `cibuildwheel@v3.0.0` (Python 3.13 正式サポート)

**変更箇所**:
- `.github/workflows/build-all-wheels.yml`: Line 122
- `.github/workflows/test-build.yml`: cibuildwheelバージョン更新

### Task 2: Intel Mac サポート追加
**現在**: `macos-latest` (Apple Silicon arm64 のみ)
**更新**: `macos-13` (Intel x86_64) + `macos-latest` (Apple Silicon arm64)

**変更箇所**:
- Matrix strategy の修正
- macOS環境の並列ビルド対応

## 🎯 目標構成 (30ホイール)

```
Linux (10ホイール):
├── manylinux x86_64: Python 3.9-3.13 (5ホイール)
└── musllinux x86_64:  Python 3.9-3.13 (5ホイール)

Windows (5ホイール):
└── win_amd64: Python 3.9-3.13 (5ホイール)

macOS Intel (5ホイール):
└── macosx x86_64: Python 3.9-3.13 (5ホイール)

macOS Apple Silicon (5ホイール):  
└── macosx arm64: Python 3.9-3.13 (5ホイール)

ARM64サポート (将来課題):
├── Linux aarch64: 現在未対応
└── Windows arm64: 現在未対応
```

## 📋 実装計画

### Phase 1: ブランチ準備
- [x] 新ブランチ作成: `feature/upgrade-cibuildwheel-and-intel-mac`
- [ ] main ブランチから最新取得

### Phase 2: cibuildwheel 最新化
- [ ] `build-all-wheels.yml`: v2.17.0 → v3.0.0
- [ ] `test-build.yml`: cibuildwheel更新（必要に応じて）
- [ ] Python 3.13 ビルド確認

### Phase 3: Intel Mac サポート追加
- [ ] Matrix strategy 修正
- [ ] macOS環境変数の確認・調整
- [ ] 並列ビルド時間の最適化

### Phase 4: テスト・検証
- [ ] validate-config.yml での設定確認
- [ ] test-build.yml での単一環境テスト
- [ ] build-all-wheels.yml での全環境ビルド確認

### Phase 5: ドキュメント更新
- [ ] `.github/workflows/README.md` 更新
- [ ] root `README.md` 環境情報更新
- [ ] 30ホイール対応の記載

### Phase 6: PR作成
- [ ] コミット・プッシュ
- [ ] PR作成（別セッション/タスクとして分離）
- [ ] レビュー・マージ

## ⚠️ 注意事項

### 検証ポイント
- **ビルド時間**: macOS環境追加による実行時間増加
- **matrix制限**: GitHub Actions の同時実行数制限
- **cibuildwheel v3.0**: 新バージョンでの安定性確認

### リスク軽減
- 段階的実装（cibuildwheel更新 → Intel Mac追加）
- 既存機能への影響確認
- 失敗時のロールバック計画

## 🔗 関連情報

- **cibuildwheel リリース**: [v3.0.0 Release Notes](https://github.com/pypa/cibuildwheel/releases/tag/v3.0.0)
- **GitHub Actions macOS runners**: [Available images](https://github.com/actions/runner-images)
- **Python 3.13 サポート状況**: cibuildwheel v2.20+ で正式サポート

---

**作成日**: 2025-06-22
**前回完了**: 20環境対応ワークフロー基盤構築
**今回目標**: 30環境対応への拡張と Python 3.13 サポート