# Rust SNS Cross-Post Tool

This is a command line tool designed for cross-posting predefined message content to multiple SNS. It offers the following functionalities:

- Configuration of the post's content
- Configuration and storage of login credentials
- Execution of post to specific SNS (currently supports only BlueSky)

## Installation Instructions

Use the following command to build this tool:

```shell
cargo build
```

## Usage

1. Environment Variable Configuration
    - Set the account information of the SNS where you will post your message in environment variables.
    - `BLUESKY_LOGIN_NAME` : Login name on BlueSky
    - `BLUESKY_APP_PASSWORD` : App Password for BlueSky
    - We recommend placing an `.env` file in the project root directory and setting the above environment variables in it.

2. Message Configuration
    - Set the content of the message you will post in the `message.json` file. This file should have the following format:

        ```json
        {
            "content": "Message content",
            "sender": "Sender",
            "receivers": ["BlueSky"]
        }
        ```

3. Executing the Tool
    - Execute the following commands from the command line tool you built:

        ```sh
        sns-cross-post-tool send
        ```

    - By specifying the `send` argument, the configured message will be posted to the specified SNS.

4. Running Tests
    - Execute the following command from the command line:

        ```sh
        cargo test
        ```

That summarizes the usage of this tool.


# Rust SNS Cross-Post Tool

このツールは、投稿内容を設定したメッセージを複数のSNSにクロスポストするためのコマンドラインツールです。以下のような機能が提供されています。

- 投稿するメッセージ内容の設定
- ログイン情報の設定と保存
- 特定のSNS（現在はBlueSkyのみ対応）への投稿実行

## インストール方法

以下のコマンドを実行してこのツールをビルドしてください：

```shell
cargo build
```

## 使い方

1. 環境変数の設定
    - メッセージを投稿する先のアカウント情報を環境変数に設定します。
    - `BLUESKY_LOGIN_NAME` : BlueSkyのログイン名
    - `BLUESKY_APP_PASSWORD` : BlueSkyのアプリパスワード
    - `.env` ファイルをプロジェクトのルートディレクトリに配置して、その中に上記の環境変数を設定することを推奨します。

2. メッセージの設定
    - 投稿するメッセージの内容を `message.json` ファイルに設定します。このファイルは以下の形式である必要があります。

        ```json
        {
            "content": "Message content",
            "sender": "Sender",
            "receivers": ["BlueSky"]
        }
        ```

3. ツールの実行
    - ビルドしたコマンドラインツールから以下のコマンドを実行します：

        ```sh
        sns-cross-post-tool send
        ```

    - `send` 引数を指定することで、設定されたメッセージを指定されたSNSに投稿します。

4. テストの実行
    - コマンドラインから以下のコマンドを実行します：

        ```sh
        cargo test
        ```

以上がこのツールの使用方法になります。
