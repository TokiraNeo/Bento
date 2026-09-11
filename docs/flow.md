# 连接与调用

Bento 是中间的 Hub。宿主主动连上来注册工具；Agent 通过 MCP 先搜再调。**Agent 不直连宿主。**

默认地址（可在桌面 Config 改）：

| 对端 | 地址 |
|---|---|
| 宿主 | `ws://127.0.0.1:2483` |
| Agent | `http://127.0.0.1:3752/mcp` |

先在桌面里把引擎点到 **运行中**。宿主用的 token 必须和 Config 里一致。

## 宿主怎么连进来

插件加载后自己连 Hub，Hub 不会去找 DCC。

```
宿主                              Bento
  │                                 │
  │── WS + Bearer token ──────────► │
  │── host.hello ─────────────────► │  host_name + protocol_version
  │◄─ host.welcome ──────────────── │  分配 namespace（如 blender / blender#2）
  │── tools.register ─────────────► │  全量工具列表
  │◄─ tools.registered ──────────── │
  │── host.ready ─────────────────► │  之后才可被搜索 / 调用
  │                                 │
  │◄─ tool.call ─────────────────── │  Hub 下发
  │── tool.result ────────────────► │
```

1. WebSocket 连 `127.0.0.1:2483`，HTTP 头带 `Authorization: Bearer <token>`。
2. `host.hello`：`protocol_version`、`host_name`。
3. `host.welcome` 里记下 `namespace`，这是工具对外名前缀。
4. `tools.register` 一次交全量（name、description、input_schema、`risk`: `normal` | `high`）。
5. `host.ready` 之后，Agent 才能搜到这些工具。
6. 保持连接，等 `tool.call`（`tool_name` + `arguments`），用 JSON-RPC 回 `tool.result`。
7. 断开后工具立刻从黄页撤掉；重连再走一遍上面。

参考实现：[BentoTest](https://github.com/TokiraNeo/BentoTest)。字段表见 [宿主 JSON-RPC](jsonrpc.md)。

## Agent 怎么调工具

把 Bento 配成普通 MCP Server，不要把宿主工具列表整份塞进上下文。

```
Agent                             Bento                              宿主
  │                                 │                                 │
  │── bento.search_tools ─────────► │  索引里搜（精确名 / 关键词）     │
  │◄─ [{ qualified_name, desc }] ── │                                 │
  │── bento.get_tool_schema ──────► │                                 │
  │◄─ input_schema ──────────────── │                                 │
  │── bento.call_tool ────────────► │── tool.call ──────────────────► │
  │                                 │◄─ tool.result ───────────────── │
  │◄─ 结果 ──────────────────────── │                                 │
```

| MCP 工具 | 做什么 |
|---|---|
| `bento.search_tools` | 传入全名或关键词，拿到 `qualified_name`（`{namespace}.{tool_name}`） |
| `bento.get_tool_schema` | 按全名取参数 schema |
| `bento.call_tool` | 按全名调用；`timeout_ms` 必填。`risk = high` 时桌面会先弹审批 |

搜的时候可以用 `blender.create_cube` 这种全名，也可以用自然语言。调用时用搜索结果里那条 `qualified_name`。
