# スタックとヒープ
https://keens.github.io/blog/2017/04/30/memoritosutakkutohi_puto/
- text領域: プログラムを置く
- data領域: 初期化されたグローバル変数を置く
- bss領域: 初期化されていない(データ領域だけ確保された)グローバル変数を置く(bss: Block Started by Symbol)
- stack領域: 関数の引数やローカル変数を置く
- heap領域: プログラムのデータを置く

- text, data, bssは実行される前からサイズがわかっているので問題ないが、heapとstackはプログラムの実行中にサイズがわかるものなのでどこにどうおいたらうまく配分できるかわからない。そこで以下のようにstackとheapを両端に配置して使いたい分だけ使用領域を伸ばせるようになっている。

```
+-------+ 2^64
| stack |
|   |   |
|   V   |
|       |
|   ^   |
|   |   |
| heap  |
+-------+
| bss   |
+-------+
| data  |
+-------+
| text  |
+-------+ 0
```

## stackと関数
- stackは関数呼び出しのために使われる。ネストした関数の呼び出しの系譜を関数の「コールスタック」と呼んだりするように関数呼び出しはスタック構造になっている。なのでスタックを用いて関すると都合がいい。

```
+--------+
| func 1 |
+--------+
| func 2 |
+--------+
| func 3 |
+--------+
| func 4 |
```

- 実際には関数の呼び出し履歴だけでなく、関数ローカルな変数などのデータも格納されている。

```
+--------+
| func 1 |
|--------|
| data 1 |
|--------|
| data 2 |
+--------+
| func 2 |
|--------|
| data   |
| ...    |
+--------+
| func 3 |
|--------|
| data   |
| ...    |
+--------+
| func 4 |
|--------|
| data   |
| ...    |
```

- データの解放は簡単で、スタックを巻き戻せば自動的に消える

```
+--------+
| func 1 |
|--------|
| data 1 |
|--------|
| data 2 |
+--------+
| func 2 |
|--------|
| data   |
| ...    |
+--------+
|        |
|        |
|        |
|        |
|        |
|        |
|        |
```

- 逆に言うと関数から抜けたら消えてしまう。
- ということで「条件が限られるけど高速に扱えるデータ領域」がstackだよ
- ちなみにメモリは使った分だけしか確保されないが、スタックを伸ばしすぎると確保されていない領域に達してエラーがでる（スタックオーバーフロー）

## heapとデータ
- heapにはstackに置けないデータが置かれる。これの扱いは少し面倒である。なぜかというとデータの確保や解放の順番がバラバラなので歯抜けな状態になってしまうから。

```
|        |
| data 4 |
+--------+
| data 3 |
+--------+
|        |
+--------+
| data 1 |
+--------+
```

- そこで「どこが使われていてどこが空いているか」を管理するシステムを導入する。C言語ではmallocという関数をインターフェースとして管理しているので管理システム自体もmallocと呼ぶことが多いよう。
- 実装方法はいろいろあるが、大抵は「メモリがこのくらい欲しい」と言われたら今管理している中からそれっぽい空きを探してそこを渡してあげるような作りになっている。
- この領域管理には(mallocの場合)そこそこのコストがかかる。でもその代わりに自由に確保/解放できる他、サイズの変更もできるので自由度が高い。

- というわけで、「自由度が高いが少しコストがかかるデータ領域」がheapである。

## プログラミング言語とメモリ
- いくつかの言語がどのようにメモリを使っているかを紹介する。

- GCのある言語ではheapの上に構築した自前のメモリ管理システムのことをヒープと読んでいることもあるので両者をきちんと区別する必要がある。
- 同じく、stackの使い方も言語独自でコールスタックと引数のスタックを分けたりするので気を付ける必要がある。

### C言語
データ領域にheapを関数呼び出しや関数ローカルなデータにstackを使っている

## Ruby
heapに確保した自前のスタックを用意している。

## Rust
- C言語と同じく、データ領域としてにheapを関数や関数ローカルなデータ用の領域としてstackを使っている。
- メモリの領域管理はmalloc, freeなどをコンパイラが自動で発効してくれる。なのでプログラマが自分で管理する必要はない。

## 高速なメモリの使い方
- 一番早い方法は「メモリを確保しない」こと。余計に確保しないのは非常に重要。
- 次はstackを使うと高速。(stackを使える言語でないと無理だけど)
- 最後の手段としてheapを使う

## まとめ