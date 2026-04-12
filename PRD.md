# Harness Stack - AI Coding Workspace Framework

> A multi-methodology implementation framework for AI-driven PRD development

**Version**: 1.0  
**Last Updated**: 2026-04-11

---

## 1. Overview

### 1.1 What is Harness Stack?

Harness Stack is a **methodology comparison and execution framework** that enables running the same PRD (Product Requirements Document) through multiple AI coding methodologies simultaneously. It provides a structured environment to clone, configure, and execute different AI development workflows against a unified requirement baseline.

### 1.2 Core Problem It Solves

AI coding tools (Cursor, Devin, Windsurf) perform differently across methodologies, but there's no systematic way to compare them. Harness Stack provides:

- **Unified baseline**: Single PRD as source of truth
- **Methodology diversity**: 6 distinct AI development approaches
- **Comparable outputs**: Standardized directory structure and artifacts
- **Easy adoption**: One-command installation of any methodology

### 1.3 Target Users

| User Type | Use Case |
|-----------|----------|
| AI Engineering Researchers | Benchmarking methodology effectiveness |
| Development Teams | Selecting best methodology for their stack |
| Tool Builders | Extending with new methodologies |
| Individual Developers | Experimenting with different AI workflows |

---

## 2. Project Structure

### 2.1 Directory Layout

```
harness-stack/
├── PRD.md                          # Source requirements document
├── setup-projects.sh              # Initialize all projects/clones
├── install-methodology.sh          # Install methodology to any project (Bash)
├── install-methodology.ps1         # Install methodology to any project (PowerShell)
├── lib/
│   └── common.sh                  # Shared functions library
├── skill-source/                  # Cloned source repositories
│   ├── openspec/
│   ├── speckit/
│   ├── superpowers/
│   ├── everything-claude-code/
│   ├── planning-with-files/
│   └── gstack/
└── workspace/                    # Methodological workspaces
    ├── Workspace-openspec/
    ├── Workspace-speckit/
    ├── Workspace-superpowers/
    ├── Workspace-everything-claude-code/
    ├── Workspace-planning-with-files/
    └── Workspace-gstack/
```

### 2.2 Repository Clones (skill-source/)

Each methodology is stored as a cloned repository in `skill-source/`:

| Repository | Source | Content |
|------------|--------|---------|
| `openspec` | Fission-AI/OpenSpec | CLI tool + skills |
| `speckit` | github/spec-kit | Specify CLI + templates |
| `superpowers` | obra/superpowers | Skills system |
| `everything-claude-code` | ysyecust/everything-claude-code | Commands + skills |
| `planning-with-files` | OthmanAdi/planning-with-files | File-based workflow |
| `gstack` | garrytan/gstack | Sprint workflow skills |

### 2.3 Workspace Structure

Each workspace contains:

```
Workspace-<methodology>/
├── .opencode/
│   ├── skills/                    # Installed skills
│   └── commands/                 # Installed commands
├── iterate-prd.sh               # Incremental iteration script
├── iterate-prd.ps1              # PowerShell version
├── prompts.md                   # Methodology-specific prompts
└── PRD.md                       # Copy of requirements
```

---

## 3. Supported Methodologies

### 3.1 Methodology Comparison Matrix

| Workspace | Workflow | Steps | Skills/Commands | Best For |
|-----------|----------|-------|-----------------|----------|
| **openspec** | Propose → Apply | 3 | `/opsx:propose`, `/opsx:apply` | Quick prototypes |
| **gstack** | Office Hours → CEO Review → Eng Review → Implement → QA → Retro | 8 | `/office-hours`, `/plan-ceo-review`, `/plan-eng-review` | Enterprise projects |
| **planning-with-files** | task_plan + findings + progress | 5 | File-based tracking | Team collaboration |
| **speckit** | Constitution → Spec → Plan → Tasks → Implement | 6 | `/speckit.specify`, `/speckit.plan` | Architecture-heavy |
| **superpowers** | Brainstorming → Plans → SDD → Verification | 5 | `brainstorming`, `writing-plans`, `subagent-driven-development` | Complex features |
| **everything-claude-code** | Plan → TDD → Review → Verify → Security | 6 | `/plan`, `/tdd`, `/verify`, `/code-review` | General purpose |

### 3.2 Detailed Workflow Descriptions

#### OpenSpec (Minimalist)
```
PRD → Gap Analysis → Propose → Apply → Verify
```
- Direct implementation approach
- CLI-initiated workflow (`openspec init`)
- Best for: Hackathons, MVPs, rapid prototyping

#### GStack (Review-Driven)
```
PRD → Office Hours → CEO Review → Eng Review → Implement → Review → QA → Retro
```
- 7-step审查流程
- Git worktree isolation
- Best for: Production projects, team reviews

#### Planning with Files (File-Tracking)
```
PRD → task_plan.md → findings.md → progress.md → Implement → Verify
```
- 3-file progress tracking
- Checkpoint-based workflow
- Best for: Team collaboration, progress visibility

#### SpecKit (Constitution-Driven)
```
PRD → Constitution → Specify → Plan → Tasks → Implement
```
- "宪法"机制确保原则一致
- Formal specification format
- Best for: Complex systems, long-term maintenance

#### Superpowers (Skills-System)
```
PRD → Brainstorming → Writing Plans → Subagent-Driven → Verification
```
- Reusable skills framework
- Autonomous subagent execution
- Best for: Repetitive patterns, complex features

#### Everything Claude Code (Toolbox)
```
PRD → /plan → /tdd → /review → /verify → /security-scan
```
- Command palette approach
- 30+ commands available
- Best for: General development, ad-hoc workflows

---

## 4. Core Scripts

### 4.1 setup-projects.sh

Initializes the entire framework by cloning all methodology repositories.

```bash
./setup-projects.sh
```

**Operations**:
1. Initializes git repo in harness-stack
2. Clones all 6 methodology repositories to `skill-source/`
3. Extracts skills/commands to each workspace
4. Runs CLI init for openspec and speckit
5. Creates initial commit

### 4.2 install-methodology.sh

Installs a single methodology to any target project.

```bash
# Basic usage
./install-methodology.sh <methodology> <target-folder>

# With clean install
./install-methodology.sh --clean superpowers ~/projects/my-app

# Short flags
./install-methodology.sh -c speckit ./workspace
```

**Options**:
| Option | Description |
|--------|-------------|
| `-c, --clean` | Remove existing .opencode/skills, .opencode/commands before install |

### 4.3 iterate-prd.sh Scripts

Each workspace has an `iterate-prd.sh` for incremental development:

```bash
cd workspace/workspace-gstack
./iterate-prd.sh --model minimax-cn/MiniMax-M2.7 --verbose
```

**Common Options** (all iterate-prd.sh scripts):

| Option | Description |
|--------|-------------|
| `--resume, -R <N>` | Resume from iteration N |
| `--model, -m <M>` | Set model (default: opencode/minimax-m2.5-free) |
| `--rounds, -r <N>` | Max implementation rounds (default: 10) |
| `--prd, -p <P>` | PRD file or directory |
| `--log, -l <F>` | Log file path |
| `--verbose, -v` | Enable verbose output |
| `--help, -h` | Show help |

### 4.5 lib/common.sh - Shared Functions

Provides reusable functions for all scripts:

| Function | Purpose |
|----------|---------|
| `parse_args()` | Standardized argument parsing |
| `log()` | Timestamped logging |
| `log_section()` | Section headers |
| `save_checkpoint()` | Phase checkpointing |
| `load_checkpoint()` | Resume from checkpoint |
| `get_next_iteration()` | Auto-detect iteration |
| `setup_iteration_output()` | Initialize output directory |
| `resolve_prd_path()` | PRD file/directory resolution |
| `GAP_ANALYSIS_PROMPT` | Shared gap analysis template |

---

## 5. Output Structure

### 5.1 Iteration Output Directory

Each run produces:

```
outputs/iteration-{N}/
├── gap-analysis.md              # PRD差距分析
├── design.md                   # Design document
├── plan.md                     # Implementation plan
├── spec.md                     # Specification (if applicable)
├── tasks.md                    # Task list
├── verification-report.md      # Verification report
├── constitution.md             # Constitution (speckit)
├── increment.md                # Increment document (openspec, gstack)
├── review-v{N}.md             # Review document (gstack)
└── .checkpoint                 # Phase checkpoint
```

### 5.2 Sessions Log

```
sessions/
├── iteration-1_20260411_143022.log
├── iteration-1_20260411_150530.log
└── iteration-2_20260412_090100.log
```

### 5.3 Build Verification

All implementations must pass:

```bash
npm run build
```

Failing builds are flagged in verification reports.

---

## 6. Dependencies

### 6.1 Core Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| opencode CLI | latest | AI coding agent |
| git | any | Version control |
| bash | 4+ | Shell scripting (Linux/macOS) |
| PowerShell | 5+ | Windows scripting |

### 6.2 Optional CLI Tools

| Tool | For Methodology | Install Command |
|------|-----------------|-----------------|
| `openspec` | openspec | `npm install -g @fission-ai/openspec` |
| `specify` | speckit | `uv tool install specify-cli` |

### 6.3 Python Environment (speckit)

| Component | Version | Purpose |
|-----------|---------|---------|
| Python | 3.11+ | Specify CLI runtime |
| uv | latest | Package manager (recommended) |
| pip/pipx | latest | Alternative install |

---

## 7. Functional Requirements

### 7.1 Core Features

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-001 | Framework shall clone all 6 methodology repositories | P0 |
| FR-002 | Framework shall install skills to .opencode/skills | P0 |
| FR-003 | Framework shall install commands to .opencode/commands | P0 |
| FR-004 | Framework shall execute iterate-prd.sh for each workspace | P0 |
| FR-005 | Framework shall auto-detect iteration number | P0 |
| FR-006 | Framework shall support resume from checkpoint | P1 |
| FR-007 | Framework shall log all operations with timestamps | P1 |
| FR-008 | Framework shall validate PRD exists before execution | P0 |

### 7.2 Installation Features

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-101 | install-methodology.sh shall accept methodology name | P0 |
| FR-102 | install-methodology.sh shall accept target directory | P0 |
| FR-103 | install-methodology.sh shall support --clean flag | P1 |
| FR-104 | CLI-init methods (openspec, speckit) shall run init | P0 |
| FR-105 | Non-CLI methods shall copy skills/commands | P0 |

### 7.3 Iteration Features

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-201 | iterate-prd.sh shall support --model flag | P0 |
| FR-202 | iterate-prd.sh shall support --verbose flag | P1 |
| FR-203 | iterate-prd.sh shall support --resume flag | P1 |
| FR-204 | iterate-prd.sh shall generate verification-report.md | P0 |
| FR-205 | iterate-prd.sh shall save checkpoints per phase | P1 |

### 7.4 Script Sharing Features

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-301 | All iterate-prd.sh shall source lib/common.sh | P0 |
| FR-302 | All iterate-prd.sh shall use GAP_ANALYSIS_PROMPT | P1 |
| FR-303 | All iterate-prd.sh shall use shared parse_args | P1 |
| FR-304 | All iterate-prd.sh shall use shared log functions | P1 |

---

## 8. Non-Functional Requirements

### 8.1 Performance

| Metric | Target |
|--------|--------|
| setup-projects.sh execution | < 5 minutes |
| Single workspace iteration | < 30 minutes |
| Memory usage | < 500MB |

### 8.2 Reliability

| Requirement | Implementation |
|-------------|----------------|
| Idempotent setup | Check before clone |
| Graceful degradation | Skip missing CLI tools |
| Error reporting | Exit codes + log files |

### 8.3 Portability

| Platform | Support Level |
|----------|---------------|
| macOS | Full (bash) |
| Linux | Full (bash) |
| Windows | Full (PowerShell) |

### 8.4 Maintainability

| Metric | Target |
|--------|--------|
| Script length | < 200 lines per iterate-prd.sh |
| Shared code ratio | > 50% via common.sh |
| Documentation | Inline comments for complex logic |

---

## 9. User Stories

### 9.1 Researcher Story

> **As a** researcher  
> **I want to** run the same PRD through 6 methodologies  
> **So that** I can compare code quality, architecture, and completeness

**Acceptance Criteria**:
- [ ] All 6 workspaces execute successfully
- [ ] Output directories are comparable
- [ ] Verification reports highlight differences

### 9.2 Team Lead Story

> **As a** team lead  
> **I want to** install a methodology into an existing project  
> **So that** my team can use AI-assisted development with best practices

**Acceptance Criteria**:
- [ ] `install-methodology.sh superpowers ./my-project` works
- [ ] Skills appear in `.opencode/skills/`
- [ ] Commands appear in `.opencode/commands/`
- [ ] Scripts are copied to project root

### 9.3 Developer Story

> **As a** developer  
> **I want to** iterate on a PRD across multiple cycles  
> **So that** I can incrementally improve implementation

**Acceptance Criteria**:
- [ ] Iteration 2 builds on Iteration 1 output
- [ ] `--resume 2` continues from checkpoint
- [ ] Gap analysis identifies remaining work

### 9.4 CI/CD Story

> **As a** DevOps engineer  
> **I want to** run all workspaces in CI  
> **So that** I can benchmark methodology effectiveness over time

**Acceptance Criteria**:
- [ ] `iterate-prd.sh` exits with proper code
- [ ] Logs are written to file
- [ ] Model can be passed via argument

---

## 10. Technical Design

### 10.1 Shared Library Architecture

```
lib/common.sh
├── parse_args()           # Argument parsing
├── log()                  # Logging
├── save_checkpoint()      # State persistence
├── get_next_iteration()   # Iteration detection
├── resolve_prd_path()     # Path resolution
└── GAP_ANALYSIS_PROMPT   # Shared prompt
```

### 10.2 Checkpoint System

```
outputs/iteration-N/.checkpoint
iteration=2
phase=phase3
timestamp=2026-04-11 14:30:22
```

### 10.3 Iteration Detection

```bash
LAST=$(ls -d outputs/iteration-* | sed 's/.*iteration-//' | sort -n | tail -1)
NEXT=$((LAST + 1))
```

### 10.4 PRD Resolution Priority

1. `--prd` flag if provided
2. `./PRD.md` in workspace root
3. Error if neither exists

---

## 11. Testing Strategy

### 11.1 Syntax Validation

All scripts must pass:
```bash
bash -n script.sh
```

### 11.2 Integration Tests

| Test | Expected Result |
|------|-----------------|
| `./setup-projects.sh` | 6 directories in skill-source/ |
| `./install-methodology.sh speckit /tmp/test` | Skills in /tmp/test/.opencode/ |
| `./iterate-prd.sh --help` | Help text displayed |

### 11.3 Convention Tests

All implementations must pass convention tests before verification:

| Test | Tool | Requirement |
|------|------|-------------|
| **No AI Slop** | `ai-slop-remover` | Remove AI-generated code smells from all changed files |
| **Linting** | ESLint/Prettier | Code follows project style guide |
| **Type Checking** | TypeScript | No type errors (`tsc --noEmit`) |
| **Formatting** | Prettier | Files are formatted (`prettier --check`) |
| **Secrets** | git-secrets/Whispers | No credentials committed |

**Convention Test Execution**:
```bash
# Run before any verification
ai-slop-remover --check ./src
prettier --check ./src/**/*.ts
tsc --noEmit
```

**Convention Test Enforcement**:
- Convention tests run BEFORE build verification
- If conventions fail, implementation is rejected
- Reports generated in `outputs/iteration-{N}/convention-report.md`

### 11.4 Manual Verification Checklist

- [ ] Each iterate-prd.sh has unique output directory
- [ ] Checkpoints save after each phase
- [ ] Resume continues from correct phase
- [ ] Logs appear in sessions/ directory
- [ ] Verification reports generate

---

## 12. Future Enhancements

### 12.1 Short-Term

| Enhancement | Priority |
|-------------|----------|
| Add more methodologies (克劳德代码, etc.) | P2 |
| Parallel workspace execution | P2 |
| JSON config file support | P3 |

### 12.2 Long-Term

| Enhancement | Priority |
|-------------|----------|
| Web dashboard for comparison | P3 |
| Automated code quality scoring | P3 |
| Methodology recommendation engine | P4 |

---

## 13. Glossary

| Term | Definition |
|------|-----------|
| PRD | Product Requirements Document - source requirements specification |
| Methodology | Distinct AI development workflow (e.g., GStack, SpecKit) |
| Workspace | Per-methodology directory with skills, commands, and scripts |
| Skill | Reusable prompt/instruction set for AI agents |
| Command | Slash commands (e.g., /plan, /verify) |
| Iteration | Single execution cycle of implementation |
| Gap Analysis | Comparison of current state vs. PRD requirements |
| Checkpoint | Saved state for resume capability |

---

## 14. References

- [OpenCode CLI](https://github.com/opencode-ai/opencode)
- [OpenSpec](https://github.com/Fission-AI/OpenSpec)
- [Spec-Kit](https://github.com/github/spec-kit)
- [Superpowers](https://github.com/obra/superpowers)
- [Everything Claude Code](https://github.com/ysyecust/everything-claude-code)
- [Planning with Files](https://github.com/OthmanAdi/planning-with-files)
- [GStack](https://github.com/garrytan/gstack)

---

## 15. Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-11 | Initial PRD document |
| 1.1 | 2026-04-12 | Add convention tests (section 11.3) |
