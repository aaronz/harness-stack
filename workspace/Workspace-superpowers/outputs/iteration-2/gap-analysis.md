I detect analysis intent — need to gather context by exploring codebase structure and finding PRD.

**My approach**: Parallel exploration of directory structure + locating PRD → synthesize gap analysis.
# 差距分析报告

## 一、PRD核心需求回顾

| PRD模块 | 核心功能 |
|---------|----------|
| 五大评估维度 | 上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束 |
| 多LLM支持 | OpenAI/Anthropic/Google/Ollama可配置切换 |
| 评估流程 | 文件上传 → AI评估 → 评分/等级/建议 |
| 结果展示 | 评分卡、等级(S/A/B/C)、优化建议、风险热力图 |
| 配置管理 | 运行时LLM配置 CRUD |

---

## 二、差距列表

| 差距项 | 严重程度 | 模块 | 修复建议 |
|--------|----------|------|----------|
| 风险热力图功能缺失 | **P0** | 前端/后端 | 实现需求原文高亮，标记AI高误解风险段落 |
| 输入验证缺失 | **P0** | API层 | 使用Zod验证所有API输入 |
| 单元测试缺失 | **P0** | 测试 | 添加评估引擎、API路由的单元测试 |
| LLM配置编辑/删除缺失 | **P1** | API/前端 | 实现 `/api/config/llm/[id]` PUT和DELETE |
| 评估统计API缺失 | **P1** | 后端 | 添加统计endpoint（采纳率、返工率等） |
| JSON解析错误处理不完善 | **P1** | lib/llm.ts | 增强parseResponse的容错能力 |
| 优化建议分类不精确 | **P2** | lib/llm.ts | 建议严格按context/boundary/atomicity/verifiability分类 |
| 文件大小限制未实现 | **P2** | API | 添加file size limit (如10MB) |
| 权重配置硬编码 | **P2** | 配置 | 权重可配置化，存入数据库或.env |
| 评估历史筛选功能缺失 | **P2** | 前端 | 添加按日期/分数/等级筛选 |

---

## 三、P0/P1/P2问题分类

### P0 - 阻断性问题

| 问题 | 影响 | 位置 |
|------|------|------|
| 风险热力图缺失 | 核心PRD功能未实现，用户无法识别高风险段落 | 前端evaluations/[id] |
| 输入验证缺失 | 安全性问题，非法输入可能导致崩溃 | 所有API路由 |
| 测试覆盖缺失 | 无法保证代码质量，线上风险高 | 全项目 |

### P1 - 重要功能缺失

| 问题 | 影响 | 位置 |
|------|------|------|
| LLM配置编辑/删除 | 配置管理不完整，无法修改已添加的LLM | api/config/llm/[id] |
| 评估统计功能 | 无法追踪PRD中的成功指标（采纳率、返工率） | 新增API |
| JSON解析容错 | LLM响应格式异常时会直接报错 | lib/llm.ts |

### P2 - 体验优化

| 问题 | 影响 | 位置 |
|------|------|------|
| 建议分类不精确 | 建议与维度不对应 | lib/llm.ts prompt |
| 文件大小限制 | 可能导致大文件DDOS | api/evaluations |
| 权重硬编码 | 无法根据业务调整评估策略 | lib/evaluator.ts |

---

## 四、技术债务清单

| 债务项 | 描述 | 修复成本 |
|--------|------|----------|
| T1 | 未使用的Zod依赖 (已安装未使用) | 低 |
| T2 | API路由缺少输入校验 (无schema validation) | 中 |
| T3 | 提示词硬编码在llm.ts中 | 低 |
| T4 | 评估权重 DEFAULT_WEIGHTS 硬编码 | 低 |
| T5 | 缺少错误边界处理 | 中 |
| T6 | 无loading states / toast notifications | 低 |
| T7 | 无响应式布局完整适配 | 低 |
| T8 | 无国际化支持 | 高 |

---

## 五、功能完整性矩阵

| PRD功能 | 状态 | 说明 |
|---------|------|------|
| 五大维度评分 | ✅ 已实现 | context/atomicity/boundary/verifiability/tech |
| 复杂度惩罚 | ✅ 已实现 | simple:1.0, medium:0.9, complex:0.7 |
| 等级评定 | ✅ 已实现 | S(90+)/A(75+)/B(60+)/C(<60) |
| 多LLM支持 | ✅ 已实现 | openai/anthropic/google/ollama |
| 文件上传评估 | ✅ 已实现 | 支持.md/.txt |
| 优化建议生成 | ⚠️ 部分 | 有输出但分类不精确 |
| 风险热力图 | ❌ 未实现 | **缺失** |
| 配置CRUD | ⚠️ 部分 | 缺编辑/删除 |

---

## 六、接口完整性

| API端点 | GET | POST | PUT | DELETE |
|---------|-----|------|-----|--------|
| /api/evaluations | ✅ 列表 | ✅ 创建 | - | - |
| /api/evaluations/[id] | ✅ 详情 | - | - | ✅ 删除 |
| /api/config/llm | ✅ 列表 | ✅ 创建 | - | - |
| /api/config/llm/[id] | ✅ 详情 | - | ❌ **未实现** | ❌ **未实现** |

---

## 七、总结

**核心功能完成度: ~75%**

- ✅ 基础评估流程完整
- ✅ 多LLM集成完成
- ⚠️ 结果展示不完整（缺风险热力图）
- ⚠️ 配置管理不完整
- ❌ 输入安全无保障
- ❌ 无测试覆盖

**建议优先级:**
1. **P0**: 补充输入验证 + 单元测试（质量底线）
2. **P1**: 实现风险热力图 + LLM配置编辑（核心功能）
3. **P2**: 完善统计、筛选等体验功能（优化）
