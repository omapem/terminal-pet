Repository: terminal-pet

Purpose
- Small terminal-based pet toy project. The primary project artefact in this repository is a Word document (terminal-pet-prd.docx) containing the product requirements.

Quick workspace notes
- There is no code, package, or test harness in this repository. The repo currently contains only terminal-pet-prd.docx.

Build, test, and lint commands
- None present. No package.json, Makefile, or other build/test/lint config found.
- If code is added later (suggested):
  - Common commands to add for consistency: npm test, npm run lint, make test
  - Running a single test: use the test runner's single-test flag (e.g., jest path/to/test -t "test name"), or use test.only in the test file.

High-level architecture
- Not applicable: no source code files or directories (src/, lib/, etc.) detected.

Key conventions and repository-specific patterns
- Product artefact is stored as a Word document at repository root: terminal-pet-prd.docx
- Any future code should live in a top-level src/ directory and include package.json, CI workflow, and CONTRIBUTING.md.

AI assistant integration notes
- No existing AI assistant configuration files detected (CLAUDE.md, AGENTS.md, .cursorrules, .windsurfrules, CONVENTIONS.md, etc.).
- When contributing code, include unit tests, a README, and CI workflows to enable useful guidance for Copilot sessions.

MCP servers
- No MCP servers suggested because this repository currently contains no runnable code or test suites.

Summary
- Created .github/copilot-instructions.md summarizing the current repository state and guidance for future contributors.

If you'd like, can expand this file to include templates for package.json, a sample GitHub Actions workflow for Node.js, or add MCP server suggestions once code is added.