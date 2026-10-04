# Cross-platform release process

QVTF Author is compiled natively on GitHub-hosted macOS, Windows, and Ubuntu runners. Local Windows and Linux hardware is not required to create packages; the virtual machines are used for final installation and functional testing.

## Development chain

1. Every pull request and push to `main` runs Rust tests and compilation checks on macOS, Windows, and Ubuntu.
2. Update the version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.
3. Create and push a matching tag, for example `v1.1.0`.
4. The release workflow creates a draft GitHub Release containing:
   - macOS Apple Silicon `.app`/`.dmg`
   - Windows x64 NSIS installer (`-setup.exe`)
   - Linux x86_64 `.AppImage` and `.deb`
5. Download the draft assets into clean Windows and Linux virtual machines and complete the checks below.
6. The workflow uploads a SHA-256 manifest after all packages build. Publish the release only after both VM passes succeed.

## Windows VM acceptance check

- Use a clean, fully updated Windows 11 x64 VM.
- Install the NSIS package and confirm QVTF Author appears in the Start menu.
- Expect a SmartScreen warning while the installer is unsigned.
- Check missing-dependency messages before installing document tools.
- Install Pandoc and MiKTeX (or TeX Live), restart QVTF Author, and confirm they are detected.
- Install CSpell globally with Node.js until the CSpell executable is bundled by the project.
- Open and save files whose paths contain spaces and non-ASCII characters.
- Export HTML, LaTeX, DOCX, and a PDF; open each output from QVTF Author.
- Uninstall QVTF Author and confirm user documents remain untouched.

## Linux VM acceptance check

- Test the `.AppImage` and `.deb` on a clean Ubuntu 22.04 or newer x86_64 VM.
- Confirm the AppImage is executable and launches; confirm the Debian package adds a desktop application entry.
- Check missing-dependency messages before installing document tools.
- Install Pandoc, TeX Live, Node.js, and CSpell, then confirm dependency detection.
- Open and save files whose paths contain spaces and non-ASCII characters.
- Export HTML, LaTeX, DOCX, and a PDF; open each output from QVTF Author.
- Remove the package and confirm user documents remain untouched.

## Dependency policy

The first Windows and Linux releases bundle QVTF Author and its language dictionaries. Pandoc, a LaTeX distribution, Node.js, and the CSpell executable remain system dependencies. A future release may bundle Pandoc and CSpell, but a complete TeX distribution should remain external because of its size and update requirements.

## Signing policy

- Current macOS Apple Silicon packages are Developer ID signed, Apple-notarized, stapled, and verified by Gatekeeper before distribution.
- Windows packages are initially unsigned and may trigger SmartScreen.
- All release assets should have SHA-256 checksums published alongside them.

## v1.1.2 release candidate status

The signing workflow was merged into `main` on 25 September 2026. The successful manual build at https://github.com/QuantumVectorTimeField/qvtf-author/actions/runs/36164967255 produced verified macOS packages; the maintainer installed the macOS build and reported it working. Windows and Linux builds also passed, but installation and export acceptance tests remain pending.

The dedicated `Refresh v1.1.2 draft with verified macOS packages` workflow promotes the exact verified macOS artifact into the existing v1.1.2 draft, preserves a backup, regenerates checksums, and updates release notes. It checks the source run and package hashes, preserves the existing Windows/Linux packages byte for byte, and refuses to modify a published release. Confirm that this workflow succeeds before using the draft for acceptance testing.

The draft stays unpublished until the Windows and Linux checklists above are completed. Manual runs of the general release workflow on a branch only retain macOS artifacts; they do not replace draft assets or retain Windows/Linux installers. Tag-triggered runs create draft releases and generate checksums after all platforms succeed.

