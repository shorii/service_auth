# service_auth

単純な認証サービス。

## 認証フロー

```mermaid
sequenceDiagram
  autonumber
  Client->>+Envoy: GET /
  Envoy->>+UI Server: GET /
  UI Server-->> Envoy: response
  Envoy-->> Client: response
  Client->>+Envoy: POST /api/user/login
  Envoy->>+API Server: POST /user/login
  API Server-->> Envoy: Set-Cookie: cookie=eyJ...
  Envoy-->> Client: Set-Cookie: cookie=eyJ...
  Client->>+Envoy: GET 事前に設定したパス
  Envoy->>+API Server: GET /user/jwks
  API Server-->> Envoy: { keys: [ ... ] }
  Envoy-->Envoy: JWT認証
  Envoy-->Service Server: GET 事前に設定したパス
  Service Server-->>Envoy: response
  Envoy-->>Client: repsonse
```

## 実行

コンテナを起動する。

```sh
$ cd development
$ docker-compose up -d
```

APIサーバを起動する。
```sh
$ cd api
$ cargo run -- --bind 0.0.0.0:8080
```

UIサーバを起動する。
```sh
$ cd ui
$ yarn install
$ yarn dev
```

http://localhost:10000/ を開く。
