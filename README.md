# Camola - Claude Skills Spellbook

<div align="center">
  <img src="spellbook.png" alt="Spellbook" width="400"/>
</div>

A comprehensive collection of Claude Code skills for C# development, database operations, and cloud integrations. These skills provide Claude with deep knowledge of specific technologies and patterns used in production systems.

## What Are Claude Code Skills?

Claude Code skills are specialized knowledge modules that give Claude AI expertise in specific technical domains. When a skill is present in `.claude/skills/`, Claude automatically gains access to proven patterns, examples, and best practices for that technology.

## Available Skills

### 🔍 **CSharpener**
C# static analysis tool for call graphs, unused code detection, and impact analysis. Built on Roslyn for comprehensive codebase understanding.

**Key Features:**
- Find unused methods with confidence levels
- Analyze what would break before deletion
- Generate HTML documentation with call graphs
- Identify method dependencies and callers

### 💾 **Databases**
RDBMS access patterns for multiple database systems using both ODBC and native drivers.

**Supported Databases:**
- DuckDB (Parquet file queries)
- PostgreSQL (Npgsql native + ODBC)
- MySQL (native + ODBC)
- SQL Server / X3 / Sage1000 (ODBC)
- DBISAM / Exportmaster (ODBC)

**Key Features:**
- PgQuery command-line tool for ad-hoc queries
- Type-safe result mapping patterns
- Connection pooling and timeout handling
- PostgreSQL array support

### ⬆️ **Dotnet 8 to 9**
Migration guide for upgrading .NET projects from version 8 to 9.

**Key Features:**
- Pre-migration preparation checklist (for .NET 8 environments)
- String literal parsing fixes
- ImplicitUsings configuration
- Package version updates

### 🔎 **Elasticsearch**
Elasticsearch 5.2 operations using HTTP API with proven production patterns.

**Key Features:**
- Scroll API for large dataset downloads
- Bulk indexing with alias pattern for zero-downtime updates
- Search with range filters
- Index management and cleanup

### 📧 **Email**
SMTP email with automatic log file compression and multiple attachments.

**Key Features:**
- Read locked log files using FileShare.ReadWrite
- Compress logs to zip before sending
- Multiple attachment support
- Automatic cleanup of temporary files

### 🖼️ **Image Files**
Image manipulation using ImageMagick command-line tools.

**Key Features:**
- Resizing and format conversion
- Optimization and compression
- Batch processing workflows

### 📝 **Logging**
UTF-8 file logging with automatic date-based filenames and thread-safe operations.

**Key Features:**
- Automatic `{ProgramName}_YYYY_MM_DD.log` filenames
- UTF-8 encoding without BOM
- Thread-safe concurrent writes
- Smart directory path detection
- Includes complete Utf8LoggingExtensions.cs implementation

### 📊 **Parquet Files**
Creating, updating, and reading Parquet files in C# with multi-threading support.

**Key Features:**
- Dynamic schema generation from data sources
- Incremental updates with timestamp tracking
- Multi-threaded file creation with ConcurrentBag
- Streaming pattern for large datasets (prevents memory bloat)
- Batch processing with memory management

### ☁️ **SharePoint**
SharePoint Online integration using both CSOM (PnP.Framework) and Microsoft Graph API.

**Key Features:**
- File upload with automatic chunking for large files
- Folder hierarchy creation
- Excel cell updates via Graph API
- SharePoint URL normalization

## Repository Structure

```
.claude/
└── skills/
    ├── CSharpener/
    │   └── SKILL.md
    ├── Databases/
    │   ├── SKILL.md
    │   ├── ODBC.cs
    │   └── PgQuery.cs
    ├── Dotnet 8 to 9/
    │   └── SKILL.md
    ├── Elasticsearch/
    │   ├── SKILL.md
    │   ├── Elasticsearch.cs
    │   └── ElasticsearchService.cs
    ├── Email/
    │   └── SKILL.md
    ├── Image Files/
    │   └── SKILL.md
    ├── Logging/
    │   ├── SKILL.md
    │   └── Utf8LoggingExtensions.cs
    ├── Parquet Files/
    │   ├── SKILL.md
    │   ├── BPQuery_Parquet.cs
    │   ├── ParquetUpdateQueue.cs
    │   └── ElastiCompare_ParquetService.cs
    └── SharePoint/
        └── SKILL.md
```

## How to Use These Skills

### In Claude Code

When you open a project with these skills, Claude automatically has access to them. Simply ask Claude to help with tasks related to these technologies:

```
"Help me create a Parquet file from this MySQL query"
"Find unused methods in my C# solution using CSharpener"
"Set up UTF-8 logging for my service"
"Upload these images to SharePoint with chunking"
```

### As a Template Repository

You can use this repository as a template for new projects:

1. Click "Use this template" on GitHub
2. Create your new repository
3. Your project will automatically have these skills in `.claude/skills/`

### Adding Skills to Existing Projects

Copy the `.claude/skills/` directory into your project:

```bash
# Copy all skills
cp -r /path/to/Camola/.claude /path/to/your-project/

# Or copy specific skills
mkdir -p /path/to/your-project/.claude/skills
cp -r /path/to/Camola/.claude/skills/Databases /path/to/your-project/.claude/skills/
```

## Skill File Format

Each skill is defined by a `SKILL.md` file with this structure:

```markdown
---
name: Skill Name
description: Brief description for Claude
---

# Skill Name

## Instructions
Guidelines for Claude on when and how to use this skill

## Examples
Concrete examples of common use cases

## Reference Implementation Details
Working code examples from production systems
```

## Real-World Usage

These skills are extracted from production systems and include:

- **BPQuery**: MySQL to Parquet sync with 50k record batches
- **ElastiCompare**: Elasticsearch diff tool with scroll streaming
- **JordanPrice**: Price discount uploader with Elasticsearch aliases
- **CRMPollerFixer**: Keycloak event synchronization
- **PgQuery**: PostgreSQL ad-hoc query tool
- **RocsMiddleware Services**: Various data processing pipelines

## Contributing

To add a new skill:

1. Create a folder in `.claude/skills/[SkillName]/`
2. Add a `SKILL.md` file with the format above
3. Include reference implementation files (.cs, .py, etc.)
4. Update this README with the new skill

### Skill Guidelines

- **Focus on patterns, not boilerplate** - Show the key techniques
- **Include working code** - All examples should be production-tested
- **Document gotchas** - Highlight common pitfalls and solutions
- **Provide context** - Explain when to use each pattern
- **Keep it concise** - Claude doesn't need verbose explanations

## License

These skills document production patterns and best practices. Reference implementations are provided for educational purposes.
