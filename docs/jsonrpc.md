# 宿主 JSON-RPC

宿主和 Bento 之间每条 WebSocket 文本帧都是 **一个 JSON 对象**，JSON-RPC 2.0。`id` 只接受 **字符串**（不要用数字）。`jsonrpc` 必须是 `"2.0"`。
字段详情见 [jsonrpc.md]("../crates/protocol/docs/jsonrpc.md")。

三种信封靠字段区分：

| 信封 | 怎么认 | 要不要回包 |
|---|---|---|
| **Request** | 有 `id` 也有 `method` | 要，用同一个 `id` 回 Response |
| **Notification** | 有 `method`、没有 `id` | 不要回 |
| **Response** | 有 `id`、没有 `method` | — |

当前 `protocol_version` 是 `2026-07-28`。

## Request

```json
{
  "jsonrpc": "2.0",
  "id": "…",
  "method": "host.hello",
  "params": { }
}
```

| 字段 | 类型 | 说明 |
|---|---|---|
| `jsonrpc` | `"2.0"` | 固定 |
| `id` | string | 调用方生成，响应原样带回 |
| `method` | string | 见下表 |
| `params` | object / `null` | 可省略，缺省当 `null` |

不要塞未知键。

当前会作为 Request 出现的 method：

| `method` | 方向 | `params` | 成功时的 `result` |
|---|---|---|---|
| `host.hello` | 宿主 → Bento | `protocol_version`, `host_name` | `namespace`, `protocol_version`, `bento_version` |
| `tools.register` | 宿主 → Bento | `tools` | `count` |
| `tool.call` | Bento → 宿主 | `tool_name`, `arguments` | `content`, `is_error` |

### `host.hello`

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "host.hello",
  "params": {
    "protocol_version": "2026-07-28",
    "host_name": "BentoTest"
  }
}
```

| 字段 | 类型 |
|---|---|
| `protocol_version` | string |
| `host_name` | string |

成功响应：

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "namespace": "BentoTest",
    "protocol_version": "2026-07-28",
    "bento_version": "1.0.0"
  }
}
```

| 字段 | 类型 |
|---|---|
| `namespace` | string（如 `BentoTest` / `BentoTest#2`） |
| `protocol_version` | string |
| `bento_version` | string |

### `tools.register`

一次交全量，不是增量。

```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "method": "tools.register",
  "params": {
    "tools": [
      {
        "name": "echo",
        "description": "Echo text back.",
        "input_schema": {
          "type": "object",
          "properties": { "text": { "type": "string" } },
          "required": ["text"]
        },
        "risk": "normal",
        "tags": ["debug"]
      }
    ]
  }
}
```

`tools[]` 每一项：

| 字段 | 类型 | 缺省 |
|---|---|---|
| `name` | string | |
| `description` | string | |
| `input_schema` | object（JSON Schema） | |
| `risk` | `"normal"` \| `"high"` | `"normal"` |
| `tags` | string[] | `[]` |

成功响应：`{ "count": 1 }`（写入条数）。

### `tool.call`

Bento 在宿主 `host.ready` 之后下发。宿主用**同一条 `id`** 回 Response。工具业务失败请放在 `result.is_error`，不要改成 JSON-RPC `error`。

```json
{
  "jsonrpc": "2.0",
  "id": "a1b2",
  "method": "tool.call",
  "params": {
    "tool_name": "echo",
    "arguments": { "text": "hi" }
  }
}
```

| 字段 | 类型 |
|---|---|
| `tool_name` | string（局部名，不含 namespace） |
| `arguments` | 任意 JSON |

成功响应：

```json
{
  "jsonrpc": "2.0",
  "id": "a1b2",
  "result": {
    "content": [{ "type": "text", "text": "hi" }],
    "is_error": false
  }
}
```

| 字段 | 类型 | 缺省 |
|---|---|---|
| `content` | `{ "type": "text", "text": string }[]` | |
| `is_error` | bool | `false` |

## Notification

没有 `id`，发出去即可。

```json
{
  "jsonrpc": "2.0",
  "method": "host.ready",
  "params": {}
}
```

| 字段 | 类型 | 说明 |
|---|---|---|
| `jsonrpc` | `"2.0"` | 固定 |
| `method` | string | |
| `params` | object / `null` | 可省略 |

当前只有：

| `method` | 方向 | `params` |
|---|---|---|
| `host.ready` | 宿主 → Bento | `{}`（无字段） |

`host.ready` 之后，这些工具才会进搜索、才会收到 `tool.call`。

## Response

对应某条 Request 的 `id`。成功带 `result`，失败带 `error`，不要两个都带。

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": { }
}
```

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "error": {
    "code": -32602,
    "message": "Invalid params"
  }
}
```

| 字段 | 类型 |
|---|---|
| `jsonrpc` | `"2.0"` |
| `id` | string（与请求相同） |
| `result` | 任意 JSON（成功时） |
| `error.code` | number |
| `error.message` | string |
| `error.payload` | 任意 JSON，可选 |

上线、注册、调用的时序见 [连接与调用](flow.md)。
