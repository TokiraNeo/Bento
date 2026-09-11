# JSON-RPC `Value` 字段

三种信封里，`params` / `result` 的类型是 `serde_json::Value`。按 `method` 解析成下面这些具体类型。

类型定义见 `src/jsonrpc/params.rs`、`src/jsonrpc/results.rs`、`src/tool.rs`。

## `JsonRpcRequest.params`

| `method` | 具体类型 |
|---|---|
| `host.hello` | `HostHelloParam` |
| `tools.register` | `ToolRegisterParam` |
| `tool.call` | `ToolCallParam` |

### `HostHelloParam`

| 字段 | 类型 |
|---|---|
| `protocol_version` | string |
| `host_name` | string |

### `ToolRegisterParam`

| 字段 | 类型 |
|---|---|
| `tools` | `ToolDefinition[]` |

`ToolDefinition`：

| 字段 | 类型 | 缺省 |
|---|---|---|
| `name` | string | |
| `description` | string | |
| `input_schema` | object（JSON Schema） | |
| `risk` | `"normal"` \| `"high"` | `"normal"` |
| `tags` | string[] | `[]` |

### `ToolCallParam`

| 字段 | 类型 |
|---|---|
| `tool_name` | string |
| `arguments` | 任意 JSON |

## `JsonRpcNotification.params`

| `method` | 具体类型 |
|---|---|
| `host.ready` | `HostReadyParam` |

### `HostReadyParam`

无字段。值为 `{}`。

## `JsonRpcResponse.result`

按所回应的那个 request 的 `method`：

| request `method` | 具体类型 |
|---|---|
| `host.hello` | `HostWelcomeResult` |
| `tools.register` | `ToolRegisterResult` |
| `tool.call` | `ToolCallResult` |

### `HostWelcomeResult`

| 字段 | 类型 |
|---|---|
| `namespace` | string |
| `protocol_version` | string |
| `bento_version` | string |

### `ToolRegisterResult`

| 字段 | 类型 |
|---|---|
| `count` | number |

### `ToolCallResult`

| 字段 | 类型 | 缺省 |
|---|---|---|
| `content` | `{ "type": "text", "text": string }[]` | |
| `is_error` | bool | `false` |
