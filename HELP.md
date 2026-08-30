# LaTeX & Markdown Helper & Publisher — User Guide & Reference Manual

Welcome to **LaTeX & Markdown Helper & Publisher**, a high-performance desktop environment for writing, spellchecking, diagnostics, and multi-format publishing of scientific and technical documents in Markdown and LaTeX.

---

## Table of Contents
1. [Quick Start](#quick-start)
2. [Document Editor & Navigation](#document-editor--navigation)
3. [Live Split-Screen Document Preview](#live-split-screen-document-preview)
4. [Multi-Language Spell Checker](#multi-language-spell-checker)
5. [LaTeX Compiler Diagnostics & Error Resolver](#latex-compiler-diagnostics--error-resolver)
6. [Publisher & Export Workflows](#publisher--export-workflows)
   - [PDF Generation (Print & arXiv)](#pdf-generation)
   - [HTML5 Export (Substack, Medium & Web)](#html5-export)
   - [LaTeX Source (.tex) Export](#latex-source-export)
   - [One-Click Workflow Presets](#workflow-presets)
7. [Preambles Manager & Custom Packages](#preambles-manager)
8. [System Dependencies & Diagnostics](#system-dependencies)
9. [Keyboard Shortcuts Cheat Sheet](#keyboard-shortcuts)

---

## 1. Quick Start

### Opening Documents
- **Open Document Button**: Click **Open Document** in the top-right header to browse for `.tex`, `.md`, or `.markdown` files.
- **Drag and Drop**: Drag any document file directly from macOS Finder (or Windows Explorer / Linux Files) and drop it anywhere into the app window.
- **Open Recent**: Select previously opened documents from the **Open Recent...** dropdown menu.
- **New Document**: Click **New Document** or press `Cmd + N` / `Ctrl + N` to create a blank document with a starter template.

### Theme Selection
Use the theme switcher in the top bar to choose between:
- 🌙 **Dark Theme**: Low-glare dark environment optimized for long writing sessions.
- ☀️ **Light Academic**: Clean, high-contrast academic print theme.
- 🖥️ **System Auto**: Automatically follows your operating system appearance preferences.

---

## 2. Document Editor & Navigation

The Document Editor is designed for seamless authoring with minimal distraction:

- **Line Numbers**: Toggle line numbering on or off using the **Show Line Numbers** switch in the editor header.
- **Find & Replace**: Press `Cmd + F` / `Ctrl + F` to reveal the search utility bar. Supports forward/backward navigation, case-insensitive match, and single or batch replacement.
- **Right-Click Context Menu**: Right-click anywhere inside the editor to quickly access **Cut**, **Copy**, **Paste**, **Select All**, or jump straight to **Scan Spelling**.
- **Adjustable Panes**: Hover over the vertical dividers between the Editor, Preview, and Sidebar to drag and resize pane widths to your exact preference.

---

## 3. Live Split-Screen Document Preview

Click the **Split Preview** button in the top bar to toggle the real-time split preview pane.

- **Real-Time Auto-Update**: As you type, the preview automatically re-renders without flickering or losing your scroll position.
- **Full LaTeX & Markdown Math**: Equations, matrices, theorems, and inline mathematics are typeset with high-fidelity vector rendering (MathML & MathJax).
- **Synchronized Scrolling**: Scrolling through the editor smoothly syncs the preview position to keep corresponding sections aligned.

---

## 4. Multi-Language Spell Checker

The built-in spell checker scans prose while intelligently ignoring LaTeX macros, math environments (`$...$`, `\\[...\\]`, `equation`, `align`), citations (`\\cite{...}`), and labels.

### Supported Languages
Select your target language from the **Spelling Language** dropdown:
- **English (UK)** (with optional **Force -ise endings** enforcement)
- **English (US)**
- **English (Canada)**
- **English (Australia)**
- **French (Français)**
- **German (Deutsch)**
- **Spanish (Español)**
- **Italian (Italiano)**
- **Portuguese (Português)**
- **Dutch (Nederlands)**

### Performing a Spell Check
1. Click **Scan Spelling**.
2. Click on any flagged word in the errors table to jump to its exact location in the editor and view localized correction suggestions.
3. Choose an action:
   - **Replace**: Replace the current instance with the chosen suggestion.
   - **Replace All**: Replace all identical misspelled occurrences across the entire document.
   - **Ignore**: Skip this word for the current session.
   - **Add Word**: Permanently add the word to your project dictionary (`cspell.json`) or global user dictionary (`~/.cspell_global.json`).

---

## 5. LaTeX Compiler Diagnostics & Error Resolver

When compiling LaTeX documents, errors from the TeX engine are parsed, categorized, and presented in the **Compiler Errors** tab.

- **Source Line Mapping**: Errors in generated TeX or macro expansions are mapped back to the original source line.
- **Smart Suggestions**: Common errors (such as missing `\\item` tags, math symbols in text mode, missing packages, unescaped special characters) include concrete, human-readable correction advice.
- **1-Click Fix**: Click **Apply Suggested Fix** to automatically apply the recommended fix directly into your document.

---

## 6. Publisher & Export Workflows

The **Publisher** tab provides industrial-grade export pipelines for academic publishing and digital distribution.

### PDF Generation
- **Engines**: Choose between **LuaLaTeX** (recommended for Unicode and OpenType fonts), **XeLaTeX**, or **PDFLaTeX**.
- **Bibliography & Citations**: Connect `.bib` files and custom `.csl` citation styles with automatic Pandoc-Citeproc or BibLaTeX processing.
- **Typography & Formatting**: Configure page size (A4, Letter), custom margins, table of contents, and equation numbering (global, by chapter, or by section).

### HTML5 Export
Export standalone, responsive HTML5 documents ready for web publication:
- **MathML (HTML5 Native Vector)**: Clean, zero-dependency offline math that renders sharp at any zoom level in all modern browsers.
- **MathJax JS**: Embedded JavaScript engine ideal for blog platforms like Substack and Medium.
- **WebTeX (SVG / PNG)**: Image-based equation fallback for legacy readers.

### LaTeX Source (.tex) Export
Converts Markdown manuscripts into clean, standalone LaTeX source documents bundled with user-selected preambles and macro packages.

### Workflow Presets
Use the **Workflow Preset** dropdown for instant 1-click configuration:
- **Formal PDF (Print/arXiv)**: Configures LuaLaTeX, A4 geometry, section-numbered equations, and formal typography.
- **Substack HTML (Dark Mode)**: Configures dark-mode math styling with embedded MathJax.
- **Medium HTML (Light Mode)**: Configures light academic styling with MathML vector math.
- **LaTeX Source (.tex)**: Prepares pure TeX source for journal submission.

---

## 7. Preambles Manager & Custom Packages

Click the **Preambles** button in the top bar to open the Preambles Manager.

- **Custom Preambles**: Create, name, and edit reusable LaTeX preambles with custom `\\usepackage` declarations, theorems, macros, and font settings.
- **Default Preamble**: Mark your preferred preamble as the default template for all new documents and conversions.
- **Built-in Templates**: Includes standard presets for Academic Papers, arXiv Preprints, and Technical Reports.

---

## 8. System Dependencies & Diagnostics

Inside the **Publisher** tab, click **Run Check** under **System Dependencies** to verify that required command-line tools are available on your PATH:
- **Pandoc**: Document conversion engine.
- **CSpell**: Multi-language spellchecking engine.
- **Node.js / npm**: Runtime for dictionary packages.
- **LuaLaTeX / XeLaTeX / PDFLaTeX**: TeX compilation engines.

If any engine is marked missing, install it via Homebrew (`brew install pandoc mactex`) or your platform package manager.

---

## 9. Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Cmd + S` / `Ctrl + S` | **Save Document** |
| `Cmd + Shift + S` / `Ctrl + Shift + S` | **Save Document As...** |
| `Cmd + O` / `Ctrl + O` | **Open Document** |
| `Cmd + N` / `Ctrl + N` | **New Document** |
| `Cmd + F` / `Ctrl + F` | **Find & Replace** |
| `Esc` | **Close Dialog / Context Menu** |
| Right-Click Editor | **Context Menu (Cut, Copy, Paste, Spellcheck)** |

---

## 10. About & Credits

- **Application**: **QVTF Author** (v1.0.0)
- **Author & Creator**: **Simeon Peter Marriott**
- **Umbrella Organization**: [QVTF.org](https://qvtf.org)
- **Website**: [https://qvtf.org](https://qvtf.org)
- **Scientific Ecosystem**: Part of the **QVTF (Quantum Vector Time Field)** software suite, featuring **QVTF Studio** for semantic vector graphics and Blender 3D scientific visualization.
- **License**: MIT Open Source License (c) 2026 Simeon Peter Marriott, QVTF.org

