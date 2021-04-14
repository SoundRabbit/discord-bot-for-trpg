# UECTRPG同好会 ダイスボット

## BOTサーバーへの導入方法

※すでにDiscordのBOTを作成して`CLIENT SECRET`を取得しており、AWS等のサーバーも存在することが前提です

### 1. Rustの実行環境を作成

[https://www.rust-lang.org/ja/tools/install](https://www.rust-lang.org/ja/tools/install)を参考に、Rustの実行環境を作ります。概ね、以下のコマンドを実行することになると思います。

```txt
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. リポジトリのクローン

```txt
git clone "https://github.com/SoundRabbit/discord-bot-for-trpg.git"
```

により、リポジトリをクローンします。

### 3. CLIENT SECRETの登録

`discord-bot-for-trpg/src`ディレクトリ（`main.rs`が入っているディレクトリ）内に`token`という名前のテキストファイルを作ります。このファイルに拡張子はいりません。そして、`token`ファイルに、`CLIENT SECRET`を保存してください。この時、末尾に改行は必要ありません。

```txt
echo [CLIENT SECRECT] > ./src/token
```

### 4. 追加メッセージの登録

`discord-bot-for-trpg/src`ディレクトリ（`main.rs`が入っているディレクトリ）内に`msg`という名前のテキストファイルを作ります。このファイルに拡張子はいりません。そして、`msg`ファイルには、ボットから送りたいテキストを入れておきます。例えば `この返信はBOTによって行われました` と入れておくと、BOTの返信には `この返信はBOTによって行われました` というテキストが追加されます。通常は空で構わないと思います。

```txt
touch ./src/msg
```

### 5. BOTの実行

`cargo run --release`コマンドを`discord-bot-for-trpg`ディレクトリで実行してください。
