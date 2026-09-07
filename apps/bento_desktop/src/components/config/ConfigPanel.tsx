/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
import { useEffect, useState } from "react";
import { getConfig, saveConfig } from "@bridge/commands/config_commands";
import type { CoreConfig } from "@bridge/types/core_config";
import { Field } from "@components/ui/Field";
import styles from "./ConfigPanel.module.css";

export function ConfigPanel() {
  const [config, setConfig] = useState<CoreConfig | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    getConfig().then(setConfig);
  }, []);

  if (!config) {
    return <div className={styles.empty}>加载配置中…</div>;
  }

  const update = (patch: Partial<CoreConfig>) =>
    setConfig((prev) => (prev ? { ...prev, ...patch } : prev));

  const host = config.host_server;
  const agent = config.agent_server;
  const { exact, lexical, semantic, fusion } = config.tool_rag;

  const save = async () => {
    await saveConfig(config);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className={styles.panel}>
      <div className={styles.header}>
        <h2 className={styles.title}>配置</h2>
        <button className={styles.save} onClick={save}>
          {saved ? "已保存" : "保存"}
        </button>
      </div>

      <section className={styles.section}>
        <h3 className={styles.sectionTitle}>宿主服务（WS）</h3>
        <div className={styles.grid}>
          <Field
            label="Host"
            value={host.host}
            onChange={(v) => update({ host_server: { ...host, host: v } })}
          />
          <Field
            label="Port"
            type="number"
            value={host.port}
            onChange={(v) => update({ host_server: { ...host, port: v } })}
          />
          <Field
            label="Token"
            value={host.token}
            onChange={(v) => update({ host_server: { ...host, token: v } })}
          />
        </div>
      </section>

      <section className={styles.section}>
        <h3 className={styles.sectionTitle}>Agent 服务（MCP）</h3>
        <div className={styles.grid}>
          <Field
            label="Host"
            value={agent.host}
            onChange={(v) => update({ agent_server: { ...agent, host: v } })}
          />
          <Field
            label="Port"
            type="number"
            value={agent.port}
            onChange={(v) => update({ agent_server: { ...agent, port: v } })}
          />
        </div>
      </section>

      <section className={styles.section}>
        <h3 className={styles.sectionTitle}>检索召回</h3>
        <div className={styles.grid}>
          <Field
            label="精确通道截断"
            type="number"
            value={exact.candidate}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, exact: { ...exact, candidate: v } } })
            }
          />
          <Field
            label="词法通道截断"
            type="number"
            value={lexical.candidate}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, lexical: { ...lexical, candidate: v } } })
            }
          />
          <Field
            label="语义通道截断"
            type="number"
            value={semantic.candidate}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, semantic: { ...semantic, candidate: v } } })
            }
          />
        </div>
      </section>

      <section className={styles.section}>
        <h3 className={styles.sectionTitle}>融合权重（RRF）</h3>
        <div className={styles.grid}>
          <Field
            label="RRF 平滑常数 k"
            type="number"
            value={fusion.rrf_k}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, fusion: { ...fusion, rrf_k: v } } })
            }
          />
          <Field
            label="精确通道权重"
            type="number"
            value={fusion.exact}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, fusion: { ...fusion, exact: v } } })
            }
          />
          <Field
            label="词法通道权重"
            type="number"
            value={fusion.lexical}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, fusion: { ...fusion, lexical: v } } })
            }
          />
          <Field
            label="语义通道权重"
            type="number"
            value={fusion.semantic}
            onChange={(v) =>
              update({ tool_rag: { ...config.tool_rag, fusion: { ...fusion, semantic: v } } })
            }
          />
        </div>
      </section>
    </div>
  );
}
