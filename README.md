# rust-practice-axum
Rust 공부 프로젝트 axum 프레임워크

Axum + MongoDB CRUD (Study)

Rust **Axum** + **MongoDB 8**로 `items`에 대한 **목록/상세/생성/수정/삭제** CRUD API를 제공합니다.

## Stack

* Rust (axum 0.8, tokio 1.x)
* MongoDB 8 (Docker Compose)
* tower-http (CORS), tracing (로그)

## 폴더 구조

```
src/
  main.rs      # 부팅/라우터 연결
  db.rs        # Mongo 클라이언트 초기화(AppState)
  error.rs     # 에러 타입 & HTTP 응답 매핑
  model.rs     # Item, CreateItem, UpdateItem
  route.rs     # /items CRUD 라우트
docker-compose.yml  # MongoDB 8
.env                # 포트/DB 연결 정보
```

## 실행 방법

1. **MongoDB 기동**

```bash
docker compose up -d
```

2. **서버 실행**

```bash
cargo run
# -> listening on http://0.0.0.0:8080
```

> `.env`에 기본 설정(포트/Mongo 접속 정보)이 들어있습니다. 필요 시 값만 바꾸시면 됩니다.

## API

### 1) 생성 (POST /items)

```bash
curl -s -X POST http://localhost:8080/items \
  -H 'Content-Type: application/json' \
  -d '{"name":"apple","price":1000}' | jq
```

### 2) 목록 (GET /items)

```bash
curl -s http://localhost:8080/items | jq
```

### 3) 상세 (GET /items/{id})

```bash
curl -s http://localhost:8080/items/{ID_치환} | jq
```

### 4) 수정 (PUT /items/{id})  — 부분 수정 가능

```bash
curl -s -X PUT http://localhost:8080/items/{ID_치환} \
  -H 'Content-Type: application/json' \
  -d '{"price":1500}' | jq
```

### 5) 삭제 (DELETE /items/{id})

```bash
curl -i -X DELETE http://localhost:8080/items/{ID_치환}
```

## 데이터 모델

* `Item`

    * `_id(ObjectId)`, `name(String)`, `price(i32)`, `created_at`, `updated_at`
