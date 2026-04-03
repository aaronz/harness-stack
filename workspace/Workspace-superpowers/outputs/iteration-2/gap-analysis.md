**Intent → Routing: Research/analysis intent** — Need to understand codebase structure AND read PRD to compare gaps. My approach: parallel explore (codebase) + find/read PRD → synthesize into gap report.

Let me gather context in parallel:
Found PRD.md at root. Let me read it and explore the outputs directory structure in parallel:
Let me read the core implementation files to compare with PRD:
Let me continue gathering more implementation details:
Let me check remaining components and tests for complete picture:
Let me check a few more files for completeness:
Now I have a complete picture. Let me compile the gap analysis report.

# 差距分析报告

## 项目概述
- **项目名称**: AI-Ready Evaluator (AI需求可实现性评估系统)
- **PRD版本**: v1.0 (2026-03-31)
- **实现状态**: 基础功能完成，核心评估流程可用

---

## 差距列表

| 差距项 | 严重程度 | 模块 | 修复建议 |
|--------|----------|------|----------|
| 评估维度权重配置UI缺失 | P0 | 前端-Settings | 在Settings页面添加ScoreWeights管理界面 |
| 风险热图/高亮未实现 | P1 | 前端-报告页 | 在评估详情页原文中标记AI高风险段落 |
| LLM配置编辑功能缺失 | P1 | 前端-LLMConfigForm | 添加编辑模式支持 |
| 模型选择器UI缺失 | P1 | 前端-上传 | 上传时可选择非默认模型 |
| 评估报告详情页缺少"上下文缺口分析" | P1 | 前端-报告页 | 显示risks数组内容 |
| 评估标题编辑UI缺失 | P2 | 前端-评估详情 | 添加标题编辑功能 |
| weights管理API无UI | P2 | 前端 | 创建Weights管理组件 |
| LLM调用无重试机制 | P2 | 后端-evaluator | 添加失败重试逻辑 |
| evaluator.test.ts内容缺失 | P2 | 测试 | 补充评估逻辑测试 |
| 无流式输出支持 | P2 | 后端-LLM | PRD提到streaming但未实现 |

---

## P0/P1/P2问题分类

### P0 (阻塞性问题)

**1. 评估维度权重配置UI缺失**
- **现状**: ScoreWeights数据模型已存在，API已实现(`GET/POST /api/config/weights`)，但Settings页面完全没有UI入口
- **影响**: 用户无法自定义评估维度权重，产品无法满足不同业务场景的评估偏好
- **修复**: 在Settings页面添加"Weights Configuration"区块，支持创建/编辑/删除权重预设

### P1 (重要功能)

**2. 风险热图/高亮未实现**
- **现状**: PRD要求"高风险区域标记：在需求原文中高亮AI最可能误解的段落"，但`evaluations/[id]/page.tsx`仅显示原始文本
- **影响**: 用户无法快速定位需求中的高风险区域，评估报告价值降低
- **修复**: 解析LLM返回的`risks`字段，在原文中用颜色标记对应段落

**3. LLM配置编辑功能缺失**
- **现状**: `LLMConfigForm`只有Add和Delete，没有Edit模式
- **影响**: 用户修改模型配置必须先删后建，不够便捷
- **修复**: 在`LLMConfigForm`中添加编辑模式，填充现有数据

**4. 模型选择器UI缺失**
- **现状**: 上传文件时强制使用默认模型，无法选择其他已配置模型
- **影响**: 用户想对比不同模型评估结果时需频繁切换默认模型
- **修复**: 在`FileUpload`组件添加模型下拉选择器

**5. 上下文缺口分析未展示**
- **现状**: `risks`数组被存储但未在报告页展示
- **影响**: PRD强调的"上下文缺口分析：列出AI生成代码时必定缺失的外部知识"未实现
- **修复**: 在评估报告页添加"Risks & Context Gaps"区块

### P2 (优化项)

| 问题 | 现状 | 建议 |
|------|------|------|
| 评估标题编辑UI | PUT API存在但无UI | 添加编辑按钮 |
| Weights管理 | API存在无UI | 创建管理组件 |
| LLM重试机制 | 调用失败直接报错 | 添加3次重试逻辑 |
| evaluator.test.ts | 存在但未验证核心逻辑 | 补充测试用例 |
| 流式输出 | 未实现 | 按需支持 |

---

## 技术债务清单

1. **类型定义不一致**
   - `EvaluationResult`中`overall: number`，但`EvaluatedResult`使用`overallScore: number`
   - 建议统一命名

2. **Error Handling不完整**
   - LLM调用失败无重试
   - 缺少请求超时处理

3. **硬编码值**
   - `llm.ts`中模型列表hardcoded在组件内
   - 建议移到配置文件或API

4. **安全性**
   - API Key加密存储已实现✅
   - 但缺乏输入验证(如model名称白名单校验)

5. **数据库**
   - 缺少索引：Evaluation.createdAt, LLMConfig.isDefault
   - 缺少数据迁移脚本

6. **前端状态管理**
   - 使用useState + prop drilling，无全局状态管理
   - 大型应用可能需要Context或Zustand

---

## 功能完整性检查

| PRD功能 | 状态 | 说明 |
|---------|------|------|
| 五大评估维度 | ✅ | 已实现 |
| 维度权重配置 | ⚠️ | API有，UI无 |
| 复杂度惩罚 | ✅ | simple/medium/complex |
| S/A/B/C等级 | ✅ | 已实现 |
| 多LLM支持 | ✅ | OpenAI/Anthropic/Gemini/Ollama |
| 运行时切换LLM | ⚠️ | 可切换但无UI选择器 |
| 优化建议生成 | ✅ | suggestions数组 |
| 风险识别 | ⚠️ | 有risks字段但未展示 |
| 风险热图 | ❌ | 未实现高亮 |
| 文件上传评估 | ✅ | TXT/Markdown |
| 评估历史 | ✅ | 列表+详情 |
| API Key加密 | ✅ | AES加密 |
| SQLite存储 | ✅ | Prisma+SQLite |

---

## 总结

**完成度: ~75%**

核心评估流程已完整实现，主要差距集中在:
1. 配置管理UI不完整(Weights)
2. 报告可视化不够丰富(风险热图、上下文缺口)
3. 部分交互功能缺失(模型选择、编辑功能)
The background exploration tasks completed, but I've already synthesized all the findings and delivered the complete gap analysis report above.

**Summary of key findings:**
- **Project completeness: ~75%**
- **P0 (blocking)**: Weights configuration UI missing
- **P1 (important)**: Risk heatmap, LLM config editing, model selector, risks display
- **P2 (optimization)**: Retry mechanism, edit UI, tests, streaming

Is there a specific gap you'd like me to prioritize fixing, or would you like me to create a detailed implementation plan for any of these items?
