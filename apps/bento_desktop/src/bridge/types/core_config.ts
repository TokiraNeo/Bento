/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

export interface HostServerConfig {
  host: string;
  port: number;
  token: string;
}

export interface AgentServerConfig {
  host: string;
  port: number;
}

export interface ExactRetrieveConfig {
  candidate: number;
}

export type ToolDocField = "name" | "tags" | "description";

export interface LexicalRetrieveConfig {
  fields: Record<ToolDocField, number>;
  k1: number;
  b: number;
  candidate: number;
}

export interface SemanticRetrieveConfig {
  candidate: number;
}

export interface FusionConfig {
  rrf_k: number;
  exact: number;
  lexical: number;
  semantic: number;
}

export interface ToolRagConfig {
  exact: ExactRetrieveConfig;
  lexical: LexicalRetrieveConfig;
  semantic: SemanticRetrieveConfig;
  fusion: FusionConfig;
}

export interface CoreConfig {
  protocol_version: string;
  host_server: HostServerConfig;
  agent_server: AgentServerConfig;
  tool_rag: ToolRagConfig;
}
