# NameDivider API TypeScript SDK Sample

TypeScript/JavaScript向けのNameDivider API SDK風サンプル実装です。

## ファイル

- `sdk-like.ts`: SDK風のクライアント実装
- `sample.ts`: SDKを使用するサンプルコード
- `package.json`: 依存関係とスクリプト定義
- `tsconfig.json`: TypeScript設定

## セットアップ

```bash
npm install
```

## 使用方法

### TypeScriptで直接実行

```bash
npm run dev
```

### JavaScriptにコンパイルして実行

```bash
npm run build
npm start
```

## SDK使用例

```typescript
import { NameDividerClient } from './sdk-like';

const client = new NameDividerClient('http://localhost:8000');

// BasicNameDividerを使用
const basicResults = await client.divideBasic(['原敬', '菅義偉']);

// GBDTNameDividerを使用
const gbdtResults = await client.divideGbdt(['原敬', '菅義偉']);

// または直接modeを指定
const results = await client.divide(['原敬', '菅義偉'], 'gbdt');
```

## 前提条件

NameDivider APIサーバーが起動している必要があります：

```bash
docker run -d --rm -p 8000:8000 namedivider-api:v0.3.0-simple
```