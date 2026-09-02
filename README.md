# QVTF Author — Scientific Markdown & LaTeX Publisher

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/badge/Release-v1.1.1-green.svg)](https://github.com/QuantumVectorTimeField/qvtf-author/releases)
[![QVTF Ecosystem](https://img.shields.io/badge/QVTF-qvtf.org-purple.svg)](https://qvtf.org)
[![GitHub Org](https://img.shields.io/badge/GitHub-QuantumVectorTimeField-blue.svg)](https://github.com/QuantumVectorTimeField)

> **A high-performance desktop writing, diagnostics, and multi-format publishing environment for scientific manuscripts in Markdown and LaTeX.** Part of the **QVTF (Quantum Vector Time Field)** software ecosystem ([qvtf.org](https://qvtf.org)) created by **Simeon Peter Marriott**.

---

## ✨ Features

- **👁️ Live Auto-Updating Split Preview**: Instant vector MathML & MathJax rendering as you type with smooth scroll synchronization and zero screen flickering.
- **🔍 Multi-Language Scientific Spellchecker**: 10 language dictionaries (UK/US/CA/AU English, French, German, Spanish, Italian, Portuguese, Dutch) with smart LaTeX command/math ignoring and British `-ise` enforcement.
- **⚠️ Compiler Diagnostics & Error Resolver**: Automatically parses and categorizes LaTeX compilation errors, maps them back to the source line, and provides one-click suggested fixes.
- **📤 Industrial Multi-Format Publishing**:
  - **Academic PDF**: LuaLaTeX, XeLaTeX, PDFLaTeX with `.bib` bibliography and custom CSL citation styles.
  - **Responsive HTML5**: Standalone HTML5 with offline vector MathML or Substack/Medium MathJax JS.
  - **LaTeX Source (.tex)**: Clean, standalone TeX files ready for journal and arXiv submission.
  - **1-Click Workflow Presets**: *Formal PDF (Print/arXiv)*, *Substack HTML*, *Medium HTML*, and *LaTeX Source*.
- **📜 Preambles & Template Manager**: Store and manage reusable LaTeX preambles, custom packages, theorem environments, and macros.
- **🌓 Dark & Academic Light Themes**: High-contrast, mathematically legible color palettes.
- **🖱️ Draggable Split-Pane Dividers**: Dynamically resize Editor, Live Preview, and Sidebar widths in real time.
- **📥 Native Drag-and-Drop**: Drag any `.tex` or `.md` file onto the window to open instantly.

---

## 🚀 Quick Start & Installation

### macOS
Download the latest release from the [Releases](https://github.com/QuantumVectorTimeField/qvtf-author/releases) page:
1. Download the latest macOS Apple Silicon package.
2. Unzip and drag `QVTF Author.app` into your `/Applications` folder.

> [!IMPORTANT]
> **Apple notarization:** The current macOS build is ad-hoc signed but is not notarized by Apple because the project does not currently participate in the paid Apple Developer Program. macOS may therefore warn that it cannot verify the developer or check the app for malicious software. Only continue if you downloaded the app from this repository's official Releases page.

If macOS blocks the first launch:
1. Try to open `QVTF Author.app` once so macOS records the blocked attempt.
2. Open **System Settings → Privacy & Security**.
3. Scroll to **Security**, click **Open Anyway**, then confirm **Open**.

This creates an exception for the app on that Mac. See [Apple's guidance for opening an app from an unknown developer](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unknown-developer-mh40616/mac).

### Windows and Linux

Windows and Linux packages are built automatically for tagged releases and verified in clean virtual machines before publication:

- **Windows 10/11 x64:** NSIS `setup.exe` installer. The initial builds are not code-signed, so Microsoft SmartScreen may show an unknown-publisher warning.
- **Linux x86_64:** AppImage and Debian (`.deb`) packages, tested on Ubuntu 22.04 or newer.

See the [cross-platform release and VM acceptance process](docs/CROSS_PLATFORM_RELEASE.md) for the complete development chain.

### Requirements
The app integrates with standard command-line scientific tools if available:
- **Pandoc** (for document conversion and MathML generation)
- **LaTeX Engine** (LuaLaTeX, XeLaTeX, or PDFLaTeX via MacTeX / TeX Live)
- **Node.js and CSpell** (language dictionaries are bundled; the CSpell executable is currently a system dependency)

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Cmd + S` / `Ctrl + S` | **Save Document** |
| `Cmd + Shift + S` / `Ctrl + Shift + S` | **Save Document As...** |
| `Cmd + O` / `Ctrl + O` | **Open Document** |
| `Cmd + N` / `Ctrl + N` | **New Document** |
| `Cmd + F` / `Ctrl + F` | **Find & Replace** |
| `Esc` | **Close Dialogs / Context Menus** |
| Right-Click | **Editor Context Menu** |

---

## 🌐 Part of the QVTF Ecosystem

- **Author & Creator**: **Simeon Peter Marriott**
- **Umbrella Organization**: [QVTF.org](https://qvtf.org)
- **GitHub Organization**: [github.com/QuantumVectorTimeField](https://github.com/QuantumVectorTimeField)
- **Vector Graphics & 3D**: Pair with [QVTF Studio](https://github.com/QuantumVectorTimeField/qvtf-studio) for semantic vector diagrams and Blender 3D scientific scenes.

---

## 💖 Sponsorship

**QVTF Author** is free, open-source scientific software. Support ongoing development and multiplatform CI/CD via [GitHub Sponsors](https://github.com/sponsors/QuantumVectorTimeField) or [Open Collective](https://opencollective.com/qvtf).

---

## 📄 License & Copyright

Copyright &copy; 2026 **Simeon Peter Marriott**, [QVTF.org](https://qvtf.org). Licensed under the [MIT License](LICENSE).
