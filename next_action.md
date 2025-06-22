# 次にお願いしたい作業

.github/workflows以下を改善し、以下の状態を達成する

- 1. [Python3.9-3.13]の5バージョンx[manylinux/musllinux/windows/mac]の4環境の合計20whlをビルドできる状態
- 2. できれば、それが1つのworkflowファイルで実現できている状態

## 現状

- Python3.11 に限れば[manylinux/musllinux/windows/mac]の4環境のビルドができる
- linuxの2つ
  - .github/workflows/build-multiplatform-wheels.whl
- windows
  - .github/workflows/test-windows-wheel.whl
- mac
  - .github/workflows/test-macos-wheel.whl

ただし、それらの書き方は必ずしも統一されていない

## 参考

https://github.com/WorksApplications/sudachi.rs/blob/develop/.github/workflows/build-python-wheels.yml のようなスマートなファイルが理想

## 実装について

方針・進め方は一緒に考えましょう。

タスクを行うときは以下を気をつけて下さい
1. ブランチを切って作業する
2. 変更を加えたファイルのみをgit addする(雑にrootからgit addしない)

## [Deep Think] 方針案

### 現状分析
- 現在3つのworkflowファイルが存在し、それぞれ異なるアプローチでビルドを実行
- `build-multiplatform-wheels.yml`: Linux（manylinux/musllinux）に対応、行数263行と複雑
- `test-windows-wheel.yml`: Windows専用、PowerShell中心の実装、行数321行
- `test-macos-wheel.yml`: macOS専用、OpenMP設定が複雑、行数244行

### 参考ワークフローの分析
sudachi.rsの実装では以下のアプローチを採用：
- `pypa/cibuildwheel@v3.0.0`を使用してクロスプラットフォーム対応
- matrixで`os`と`python-version`を組み合わせて20環境を実現
- 各プラットフォーム固有の複雑な処理をcibuildwheelに委譲

### 統合戦略案

#### 方針1: cibuildwheelベースの統合 (推奨)
**メリット:**
- 1つのワークフローファイルで20環境対応可能
- プラットフォーム固有の複雑な処理をcibuildwheelが自動処理
- 行数を大幅削減（現在828行 → 推定150行程度）
- メンテナンス性向上

**実装ステップ:**
1. 新しい統合ワークフローファイル作成
2. 現在の3ファイルからの移行
   - Linux: cibuildwheelの`CIBW_ENVIRONMENT`設定でmanylinux/musllinux対応
   - Windows: cibuildwheelの`CIBW_BEFORE_ALL_WINDOWS`でUTF-8設定
   - macOS: cibuildwheelの`CIBW_BEFORE_ALL_MACOS`でOpenMP設定
3. family_names.txtの復元処理を統一
4. テスト実行の統一

#### 方針2: 現在のファイル統合
**メリット:**
- 既存の動作確認済みロジックを活用
- 段階的な移行が可能

**デメリット:**
- 複雑性が残る（800行超のワークフローファイル）
- プラットフォーム固有の処理が分散

### 推奨実装プラン

**Phase 1: 新統合ワークフロー作成**
- `.github/workflows/build-wheels-unified.yml`を作成
- cibuildwheelベースで5バージョン×4プラットフォーム対応
- 既存ワークフローのテスト済み設定を移植

**Phase 2: 検証・調整**
- 新ワークフローでテストビルド実行
- 既存ワークフローとの結果比較
- 問題があれば個別調整

**Phase 3: 移行完了**
- 新ワークフローの動作確認後、既存3ファイルを削除
- READMEやドキュメントの更新

### 技術的検討事項

1. **family_names.txt復元処理**: 全プラットフォームで共通化
2. **UTF-8エンコーディング**: Windows固有の複雑な処理をcibuildwheelで簡素化
3. **OpenMP設定**: macOS固有のlibomp設定をcibuildwheelで処理
4. **テスト実行**: 統一されたテストスクリプトで全環境検証

この方針でいかがでしょうか？

## ビルド時間短縮のための段階的ワークフロー戦略

### 基本方針: 段階的ワークフローアプローチ
ビルド待ち時間を最小化するため、以下の3段階でワークフローを分離：

#### Stage 1: `validate-config.yml` - 軽量検証（2-3分）
**目的**: 設定ミスを早期発見、ビルドなし
**実行タイミング**: 全PR、Push時に常時実行
**検証項目**:
- 環境変数の存在確認（FAMILY_NAMES_PART_1-8）
- family_names.txt復元テスト（ファイルサイズ・行数確認）
- Python/Rustバージョン設定確認
- 依存関係インストール確認
- cibuildwheel設定構文チェック

#### Stage 2: `test-build.yml` - 単一環境ビルド（10-15分）
**目的**: 実際のビルド・動作確認を1環境で検証
**実行タイミング**: PR作成時、手動実行
**対象環境**: ubuntu-latest + Python 3.11のみ
**検証項目**:
- Stage 1の全確認項目
- 1環境でのフルビルド・テスト
- wheel生成・インストール・動作確認

#### Stage 3: `build-all-wheels.yml` - 全環境ビルド（60-90分）
**目的**: 最終的な20wheels生成
**実行タイミング**: メインブランチマージ時、リリース時、手動実行
**対象環境**: 5 Python バージョン × 4 プラットフォーム = 20環境

### 実装計画

#### Phase 1: 軽量検証ワークフロー作成
1. `validate-config.yml`作成
2. 環境変数・設定の事前チェック機能実装
3. モックデータでのfamily_names.txt復元テスト

#### Phase 2: 単一環境ビルドワークフロー作成
1. `test-build.yml`作成
2. ubuntu-latest + Python 3.11での完全ビルドテスト
3. 既存ワークフローとの結果比較

#### Phase 3: 全環境統合ワークフロー作成
1. `build-all-wheels.yml`作成
2. cibuildwheelベースで20環境対応
3. 既存3ワークフローからの設定移植

#### Phase 4: 移行完了
1. 既存ワークフロー削除
2. ドキュメント更新

この段階的アプローチにより、設定ミスは数分で、ビルド問題も15分程度で検出可能になります。