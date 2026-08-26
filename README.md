<div align="center">

# Bento

> A search engine for tools — type a name or a short description; open the one you need.

**A tool-relay hub for AI agents and tool hosts.**

![Rust](https://img.shields.io/badge/Rust-1.85+-e44c26?style=flat-square&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-GPL--3.0-333850?style=flat-square&logo=gnu&logoColor=white)
![Status](https://img.shields.io/badge/status-early--stage-e8a33d?style=flat-square)
![Arch](https://img.shields.io/badge/platform-Windows%20%2F%20macOS%20%2F%20Linux-6c7293?style=flat-square)

把 **N×M 的不可复用集成**，变成 **N+M 的标准化适配**。

</div>

---

## 这是什么

**Bento 是给 Agent 用的工具搜索引擎**，同时是宿主与 Agent 之间的中继 Hub。

AI 正在进入游戏开发流程（建模、摆放、材质、动画），但集成形态高度碎片化：

- **宿主侧**：Blender（bpy）、UE（Python/C++）、Maya…… 每家暴露工具的方式完全不同；
- **Agent 侧**：Codex、OpenCode、Cursor…… 各家已有成熟的 Agent 循环（工具调用、上下文压缩、审批）。

每对接一个 Agent × 一个宿主，就要重写一遍连接、协议、工具注册——这就是 **N×M 的组合爆炸**。把全部工具 schema 灌进上下文，也和把整个网页索引塞进用户脑子一样不现实。

Bento 做两件事：

1. **搜索**：当前在线宿主的工具构成实时语料。Agent 可以猜完整 `qualified_name`（如 `blender.create_cube`，精确命中优先），也可以用短描述/关键词检索。结果页只给摘要，点进 `get_tool_schema` 才有全文。
2. **中继**：宿主只写一个薄插件把工具注册上来，Agent 通过标准 MCP **零适配**即插即用。搜到之后 Hub 按全名路由到对应宿主；危险操作先审批。Agent 永不直连宿主。

名字仍是便当盒：工具按格装好，按需取用，而不是整桌端上来。对 Agent 而言，操作方式就是一台搜索引擎——先搜再打开，而不是面对一份无穷长的书签。

---

## 核心设计

整个项目围绕三个核心理念展开。

### 1. 一个适配中间层：让一切能暴露工具的进程接入

```
Agent 层（Codex / OpenCode / Cursor ...）
          │  MCP（零适配，即插即用）
          ▼
┌──────────────────────────────┐
│        Bento Hub             │
│  检索 · 路由 · 命名空间 · 审批│
└──────┬───────────────┬───────┘
       │ WS+JSON-RPC   │ MCP Client
       ▼               ▼
原生宿主（Blender/UE/Maya）   已有 MCP Server（fs/git/browser...）
```

- **原生宿主**：写一个薄插件，主动连入 Hub 的 WS 端口（`host.hello` 握手 → `tools.register` 注册 → `host.ready` 就绪）；
- **Agent**：把 Hub 当成一个普通的 MCP Server 配置；检索入口是 `bento.search_tools`，不要假设工具列表会一次性灌进上下文；
- **唯一通路**：Agent 永不直连宿主，一切消息经 Hub 中转，审计 / 安全 / 路由单点完成。

### 2. 工具搜索引擎：渐进式加载，而不是灌目录

一个 UE5 这样的宿主，工具集可能有数百上千个。问题结构和 Web 搜索一样：**语料大到塞不进消费者的脑子**。

**让 LLM 自己搜工具。全名命中分最高；否则用关键词 / 语义。**

```
bento.search_tools(query)     → 混合检索 Top-K（精确全名 / 词法 / 语义）
bento.get_tool_schema(name)   → 调用前取完整 schema
bento.list_domains            → 可选：先看有哪些域
```

`query` 两种写法都合法：

- 已从上下文知道全名 → 原样传入 `namespace.tool_name`（精确通道，优先于描述里碰巧出现的词）；
- 不知道名字 → 短句或关键词（词法 BM25，后续加语义）。

工具对外名称是不透明的 `qualified_name`。Agent 不必拆 namespace；猜名时遵守卡片上见过的范式。搜不到就换说法，不要发明未出现过的全名去调用。

Hub 只把少量候选暴露给 Agent，上下文自然瘦身。索引纯内存、不落盘：宿主断连即从黄页撤下，Bento 重启后等宿主重连再收录。

### 3. 过程式工具：与其封装一百个工具，不如给一个 `execute_code()`

长尾能力不应「一能力一 schema」——那会让搜索空间爆炸、维护成本失控、LLM 注意力被稀释。

```python
# 与其封装 create_cube / set_material / rotate_object / ...
# 不如给 agent 一把「钥匙」：
blender.execute_script("""
    bpy.ops.mesh.primitive_cube_add(size=2.0)
    mat = bpy.data.materials.new("Red")
    ...
""")
```

- 常驻、可检索的高层工具保持精简；
- 长尾交给脚本执行；
- `execute_script` 类工具 **默认 `risk = high`**，走 Hub 侧强制审批。

---

## 工程布局

```
bento_core                         ← 组装根：创建各组件、注入端口
  ├── bento_host_server            ← 宿主 WS：握手 / 注册 / 调用分发 / 断连清理
  ├── bento_agent_server           ← Agent MCP：search_tools / get_schema / list_domains
  └── bento_tool_rag               ← 工具搜索引擎：内存目录 + 精确 / 词法 / 语义
          └── bento_protocol       ← 传输契约（JSON-RPC / ToolDefinition / 检索 DTO）
```

| Crate | 包名 | 职责 |
|---|---|---|
| `crates/protocol` | `bento_protocol` | 跨边界传输类型：宿主 JSON-RPC 命令、`ToolDefinition`、`ToolSearchQuery` / `ToolSearchHit`。 |
| `crates/utility` | `bento_utility` | 进程内小工具（如 UUID）。 |
| `crates/tool_rag` | `bento_tool_rag` | 纯内存工具索引与混合检索。不落盘；宿主断连即清、进程重启即空。 |
| `crates/host_server` | `bento_host_server` | 被动监听宿主 WS。通过 `ToolIndexSink` 端口登记 / 就绪 / 移除工具。 |
| `crates/agent_server` | `bento_agent_server` | 面向 Agent 的 MCP 服务端。通过 `ToolQuerySink` 端口检索。 |
| `crates/core` | `bento_core` | 组装根：持有 `ToolRagEngine`，用薄适配器接到两个 server 的端口。运行时调用直达引擎。 |

## License

[GPL-3.0-or-later](LICENSE) · © 2026 TokiraNeo
