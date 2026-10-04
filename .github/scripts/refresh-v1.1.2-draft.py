"""Promote the exact tested macOS artifact into the existing v1.1.2 draft."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

REPO = 'QuantumVectorTimeField/qvtf-author'
RELEASE = 389085879
RUN = 36164967255
ARTIFACT = 10877891630
SOURCE = 'a6f6b69ec222d7421a222e78a69a8fe4bf45380f'
ZIP_SHA = '46759948f0c00c4b8433a5b40e453fe898c318535a9430e1c9ea85bb0ceea84f'
MAC = {
    'QVTF.Author_1.1.2_aarch64.dmg': 'f307ccc569df3c57b5cb2ff0c507656489829ad3549180d3f63a056e26c4dbad',
    'QVTF.Author_aarch64.app.tar.gz': '5298528f61b754343ea37da479ed3d0db24b6459d8f2c45deb57a757ad1b2d2b',
}
OTHER = {
    'QVTF.Author_1.1.2_x64-setup.exe': 'f73f5330b34ad33b73daef78ff42c0df36216b2f19b4360d290458a7b2729fdb',
    'QVTF.Author_1.1.2_amd64.AppImage': '16709fe2de22e428fcf691d908b7150ed8c5c659b3438887e0a1d5217255e479',
    'QVTF.Author_1.1.2_amd64.deb': 'b1d1612557de3e7fdbba1fe5fa0ec6b788a625454614dfe32f131138361eeb17',
}
MANIFEST = 'QVTF_Author_v1.1.2_SHA256.txt'
ROOT = Path('draft-refresh')
NOTES = '''QVTF Author 1.1.2 release candidate — held as draft pending Windows and Linux acceptance tests.

Fixes Windows helper-process console popups, adds ARM64-aware MiKTeX discovery, and cleans British-English spelling mappings.

macOS Apple Silicon: the app and DMG are Developer ID signed, Apple-notarized, stapled, and validated by Gatekeeper. These are the exact packages from the successful 25 September 2026 run, subsequently installed and reported working by the maintainer.

Windows x64: unsigned NSIS installer; build and automated tests passed. Installation and export acceptance testing is pending.

Linux x86_64: AppImage and Debian packages; build and automated tests passed. Installation and export acceptance testing is pending.

Windows and Linux packages include language dictionaries. Pandoc, a LaTeX distribution, Node.js, and the CSpell executable remain external dependencies.

Verify downloads against QVTF_Author_v1.1.2_SHA256.txt.

Verified macOS build: https://github.com/QuantumVectorTimeField/qvtf-author/actions/runs/36164967255
'''

def gh(*args, output=None):
    if output:
        with Path(output).open('wb') as f:
            subprocess.run(['gh', *args], check=True, stdout=f)
    else:
        return subprocess.check_output(['gh', *args], text=True)

def api(path):
    return json.loads(gh('api', f'repos/{REPO}/{path}'))

def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def require(condition, message):
    if not condition:
        raise RuntimeError(message)

def draft():
    r = api(f'releases/{RELEASE}')
    require(r['draft'] and r['tag_name'] == 'v1.1.2', 'Expected the existing v1.1.2 draft; refusing to modify a published release.')
    require({a['name'] for a in r['assets']} == set(MAC) | set(OTHER) | {MANIFEST}, 'Unexpected draft asset inventory.')
    return r

def prepare():
    r = draft()
    run = api(f'actions/runs/{RUN}')
    require(run['conclusion'] == 'success' and run['head_sha'] == SOURCE and run['head_branch'] == 'main', 'Verified build provenance mismatch.')
    artifact = api(f'actions/artifacts/{ARTIFACT}')
    require(not artifact['expired'] and artifact['name'] == 'verified-macos-release' and artifact['workflow_run']['id'] == RUN, 'Verified artifact unavailable or mismatched.')
    require(artifact['digest'] == 'sha256:' + ZIP_SHA, 'Artifact digest changed.')
    ROOT.mkdir()
    backup = ROOT / 'backup'; backup.mkdir()
    (backup / 'release-before.json').write_text(json.dumps(r, indent=2))
    gh('release', 'download', 'v1.1.2', '--repo', REPO, '--dir', str(backup))
    for name, expected in OTHER.items():
        require(digest(backup / name) == expected, f'Original platform package changed: {name}')
    for a in r['assets']:
        require('sha256:' + digest(backup / a['name']) == a['digest'], f'Backup digest mismatch: {a["name"]}')
    archive = ROOT / 'verified.zip'
    gh('api', f'repos/{REPO}/actions/artifacts/{ARTIFACT}/zip', output=archive)
    require(digest(archive) == ZIP_SHA, 'Downloaded artifact ZIP checksum mismatch.')
    prepared = ROOT / 'prepared'; prepared.mkdir()
    with zipfile.ZipFile(archive) as z:
        require(set(z.namelist()) == set(MAC), 'Unexpected files in verified artifact.')
        for name, expected in MAC.items():
            data = z.read(name)
            require(hashlib.sha256(data).hexdigest() == expected, f'Verified package checksum mismatch: {name}')
            (prepared / name).write_bytes(data)
    for name in OTHER:
        shutil.copy2(backup / name, prepared / name)
    lines = [f'{digest(prepared / name)}  {name}\n' for name in sorted(set(MAC) | set(OTHER))]
    (prepared / MANIFEST).write_text(''.join(lines))
    (ROOT / 'release-notes.md').write_text(NOTES)
    print('Prepared verified packages, checksums, release notes, and a complete draft backup.')

def apply():
    prepared = ROOT / 'prepared'
    for name, expected in (MAC | OTHER).items():
        require(digest(prepared / name) == expected, f'Prepared package changed: {name}')
    for name in MAC:
        draft()
        gh('release', 'upload', 'v1.1.2', str(prepared / name), '--repo', REPO, '--clobber')
    draft()
    gh('release', 'upload', 'v1.1.2', str(prepared / MANIFEST), '--repo', REPO, '--clobber')
    draft()
    gh('release', 'edit', 'v1.1.2', '--repo', REPO, '--draft=true', '--notes-file', str(ROOT / 'release-notes.md'))
    r = draft()
    assets = {a['name']: a for a in r['assets']}
    for name, expected in (MAC | OTHER).items():
        require(assets[name]['digest'] == 'sha256:' + expected, f'Uploaded asset checksum mismatch: {name}')
    require(assets[MANIFEST]['digest'] == 'sha256:' + digest(prepared / MANIFEST), 'Uploaded manifest mismatch.')
    require(r['body'] == NOTES, 'Release notes were not updated.')
    print('v1.1.2 remains draft. Verified macOS packages, checksums, and release notes updated; Windows/Linux bytes preserved.')

if __name__ == '__main__':
    {'prepare': prepare, 'apply': apply}[sys.argv[1]]()
