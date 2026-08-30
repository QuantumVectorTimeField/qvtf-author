const { invoke } = window.__TAURI__.core;

// DOM Elements
const lblFilePath = document.getElementById("lbl-file-path");
const btnOpenDoc = document.getElementById("btn-open-doc");
const btnNewDoc = document.getElementById("btn-new-doc");
const btnPreambles = document.getElementById("btn-preambles");
const selectRecentFiles = document.getElementById("select-recent-files");
const btnSaveDoc = document.getElementById("btn-save-doc");
const btnSaveAsDoc = document.getElementById("btn-save-as-doc");
const chkLineNums = document.getElementById("chk-line-nums");
const lineNums = document.getElementById("line-numbers");
const txtEditor = document.getElementById("txt-editor");
const preambleDialog = document.getElementById("preamble-dialog");
const preambleList = document.getElementById("preamble-list");
const preambleName = document.getElementById("preamble-name");
const preambleContent = document.getElementById("preamble-content");
const preambleDefault = document.getElementById("preamble-default");
const btnNewPreamble = document.getElementById("btn-new-preamble");
const btnSavePreamble = document.getElementById("btn-save-preamble");
const btnDeletePreamble = document.getElementById("btn-delete-preamble");
const btnInsertPreamble = document.getElementById("btn-insert-preamble");

// Split Preview Elements
const btnTogglePreview = document.getElementById("btn-toggle-preview");
const previewPane = document.getElementById("preview-pane");
const previewContent = document.getElementById("preview-content");

// Find/Replace
const txtFind = document.getElementById("txt-find");
const txtReplace = document.getElementById("txt-replace");
const btnFindNext = document.getElementById("btn-find-next");
const btnFindPrev = document.getElementById("btn-find-prev");
const btnReplace = document.getElementById("btn-replace");
const btnReplaceAll = document.getElementById("btn-replace-all");
const lblFindStatus = document.getElementById("lbl-find-status");

// Tabs
const tabBtnSpell = document.getElementById("tab-btn-spell");
const tabBtnErrors = document.getElementById("tab-btn-errors");
const tabBtnPublish = document.getElementById("tab-btn-publish");
const tabContentSpell = document.getElementById("tab-content-spell");
const tabContentErrors = document.getElementById("tab-content-errors");
const tabContentPublish = document.getElementById("tab-content-publish");

// Compiler Errors
const compileErrorsTbody = document.getElementById("compile-errors-tbody");
const compileResolverPanel = document.getElementById("compile-resolver-panel");
const lblCompileErrMsg = document.getElementById("lbl-compile-err-msg");
const lblCompileSuggestion = document.getElementById("lbl-compile-suggestion");
const btnApplyCompileFix = document.getElementById("btn-apply-compile-fix");

// Spell Checker
const btnScanSpell = document.getElementById("btn-scan-spell");
const errorsTbody = document.getElementById("errors-tbody");
const txtOrigWord = document.getElementById("txt-orig-word");
const txtReplaceVal = document.getElementById("txt-replace-val");
const listSuggestions = document.getElementById("list-suggestions");
const btnSpellReplace = document.getElementById("btn-spell-replace");
const btnSpellReplaceAll = document.getElementById("btn-spell-replace-all");
const btnSpellIgnore = document.getElementById("btn-spell-ignore");
const btnSpellAdd = document.getElementById("btn-spell-add");

const selectSpellLang = document.getElementById("select-spell-lang");
const chkForceIse = document.getElementById("chk-force-ise");

// Dependency Checker
const btnCheckDeps = document.getElementById("btn-check-deps");
const depPandoc = document.getElementById("dep-pandoc");
const depCspell = document.getElementById("dep-cspell");
const depNode = document.getElementById("dep-node");
const depLualatex = document.getElementById("dep-lualatex");
const depXelatex = document.getElementById("dep-xelatex");
const depPdflatex = document.getElementById("dep-pdflatex");

// Publisher Settings
const selectPreset = document.getElementById("select-preset");
const selectEngine = document.getElementById("select-engine");
const selectMathStyle = document.getElementById("select-math-style");
const selectMathFg = document.getElementById("select-math-fg");
const selectMathBg = document.getElementById("select-math-bg");
const selectMathSize = document.getElementById("select-math-size");
const selectPdfPageSize = document.getElementById("select-pdf-page-size");
const selectPdfMargin = document.getElementById("select-pdf-margin");
const divPdfCustomSize = document.getElementById("div-pdf-custom-size");
const txtPdfCustomW = document.getElementById("txt-pdf-custom-w");
const txtPdfCustomH = document.getElementById("txt-pdf-custom-h");
const selectHtmlTableWidth = document.getElementById("select-html-table-width");
const selectHtmlTableStyle = document.getElementById("select-html-table-style");
const txtBib = document.getElementById("txt-bib");
const btnBrowseBib = document.getElementById("btn-browse-bib");
const txtCsl = document.getElementById("txt-csl");
const btnBrowseCsl = document.getElementById("btn-browse-csl");

// Checkboxes
const chkCiteproc = document.getElementById("chk-citeproc");
const chkToc = document.getElementById("chk-toc");
const chkNum = document.getElementById("chk-num");
const chkLeftAlign = document.getElementById("chk-left-align");
const chkUnicodeMath = document.getElementById("chk-unicode-math");
const chkConvertInline = document.getElementById("chk-convert-inline");
const chkAutocopy = document.getElementById("chk-autocopy");

// Workflow Buttons
const btnBuildPdf = document.getElementById("btn-build-pdf");
const btnOpenPdf = document.getElementById("btn-open-pdf");
const btnBuildHtml = document.getElementById("btn-build-html");
const btnOpenHtml = document.getElementById("btn-open-html");
const btnBuildTex = document.getElementById("btn-build-tex");
const btnOpenTex = document.getElementById("btn-open-tex");
const btnBuildDocx = document.getElementById("btn-build-docx");
const btnOpenDocx = document.getElementById("btn-open-docx");

// Status Labels
const lblPdfStatus = document.getElementById("lbl-pdf-status");
const lblHtmlStatus = document.getElementById("lbl-html-status");
const lblTexStatus = document.getElementById("lbl-tex-status");
const lblDocxStatus = document.getElementById("lbl-docx-status");

// Console Log
const txtLog = document.getElementById("txt-log");

// Global State
let currentFile = "";
let lastPdf = "";
let lastHtml = "";
let lastTex = "";
let lastDocx = "";
let spellingErrors = [];
let selectedErrorIndex = -1;
let previewTimeout = null;
let previewRequestId = 0;
let searchIndex = 0;
let lastQuery = "";
let selectedPreambleId = null;

const PREAMBLES_STORAGE_KEY = "savedPreambles";
const DEFAULT_PREAMBLE_STORAGE_KEY = "defaultPreambleId";

function loadPreambles() {
  try {
    const value = JSON.parse(localStorage.getItem(PREAMBLES_STORAGE_KEY) || "[]");
    return Array.isArray(value) ? value : [];
  } catch (_) {
    return [];
  }
}

function storePreambles(preambles) {
  localStorage.setItem(PREAMBLES_STORAGE_KEY, JSON.stringify(preambles));
}

function selectPreamble(id) {
  selectedPreambleId = id;
  const preamble = loadPreambles().find(item => item.id === id);
  preambleName.value = preamble?.name || "";
  preambleContent.value = preamble?.content || "";
  preambleDefault.checked = id !== null && localStorage.getItem(DEFAULT_PREAMBLE_STORAGE_KEY) === id;
  btnDeletePreamble.disabled = !preamble;
  btnInsertPreamble.disabled = !preamble;
  renderPreambleList();
}

function renderPreambleList() {
  const preambles = loadPreambles();
  preambleList.replaceChildren();
  if (!preambles.length) {
    const empty = document.createElement("p");
    empty.className = "preamble-empty";
    empty.textContent = "No saved preambles yet.";
    preambleList.appendChild(empty);
    return;
  }
  const defaultId = localStorage.getItem(DEFAULT_PREAMBLE_STORAGE_KEY);
  preambles.forEach(preamble => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `preamble-list-item${preamble.id === selectedPreambleId ? " active" : ""}`;
    button.textContent = preamble.name + (preamble.id === defaultId ? "  • Default" : "");
    button.addEventListener("click", () => selectPreamble(preamble.id));
    preambleList.appendChild(button);
  });
}

function insertTextAtCursor(content) {
  if (!content) return;
  const start = txtEditor.selectionStart;
  const end = txtEditor.selectionEnd;
  const before = txtEditor.value.slice(0, start);
  const after = txtEditor.value.slice(end);
  const separatorBefore = before && !before.endsWith("\n") ? "\n" : "";
  const separatorAfter = after && !content.endsWith("\n") ? "\n" : "";
  const insertion = separatorBefore + content + separatorAfter;
  txtEditor.value = before + insertion + after;
  const cursor = before.length + insertion.length;
  txtEditor.setSelectionRange(cursor, cursor);
  txtEditor.dispatchEvent(new Event("input", { bubbles: true }));
  txtEditor.focus();
}

btnPreambles.addEventListener("click", () => {
  const preambles = loadPreambles();
  selectPreamble(selectedPreambleId && preambles.some(item => item.id === selectedPreambleId)
    ? selectedPreambleId
    : (preambles[0]?.id || null));
  preambleDialog.showModal();
});

btnNewPreamble.addEventListener("click", () => selectPreamble(null));

btnSavePreamble.addEventListener("click", () => {
  const name = preambleName.value.trim();
  if (!name) {
    preambleName.focus();
    return;
  }
  const preambles = loadPreambles();
  const id = selectedPreambleId || `preamble-${Date.now()}`;
  const saved = { id, name, content: preambleContent.value };
  const index = preambles.findIndex(item => item.id === id);
  if (index >= 0) preambles[index] = saved;
  else preambles.push(saved);
  storePreambles(preambles);
  selectedPreambleId = id;
  if (preambleDefault.checked) localStorage.setItem(DEFAULT_PREAMBLE_STORAGE_KEY, id);
  else if (localStorage.getItem(DEFAULT_PREAMBLE_STORAGE_KEY) === id) localStorage.removeItem(DEFAULT_PREAMBLE_STORAGE_KEY);
  selectPreamble(id);
});

btnDeletePreamble.addEventListener("click", () => {
  if (!selectedPreambleId) return;
  storePreambles(loadPreambles().filter(item => item.id !== selectedPreambleId));
  if (localStorage.getItem(DEFAULT_PREAMBLE_STORAGE_KEY) === selectedPreambleId) {
    localStorage.removeItem(DEFAULT_PREAMBLE_STORAGE_KEY);
  }
  const next = loadPreambles()[0];
  selectPreamble(next?.id || null);
});

btnInsertPreamble.addEventListener("click", () => {
  const preamble = loadPreambles().find(item => item.id === selectedPreambleId);
  if (!preamble) return;
  preambleDialog.close();
  insertTextAtCursor(preamble.content);
});

// ---------------------------------------------------------
// Tab Switching
// ---------------------------------------------------------
function switchTab(activeTab) {
  tabBtnSpell.classList.remove("active");
  tabBtnErrors.classList.remove("active");
  tabBtnPublish.classList.remove("active");
  tabContentSpell.classList.remove("active");
  tabContentErrors.classList.remove("active");
  tabContentPublish.classList.remove("active");
  
  if (activeTab === "spell") {
    tabBtnSpell.classList.add("active");
    tabContentSpell.classList.add("active");
  } else if (activeTab === "errors") {
    tabBtnErrors.classList.add("active");
    tabContentErrors.classList.add("active");
  } else if (activeTab === "publish") {
    tabBtnPublish.classList.add("active");
    tabContentPublish.classList.add("active");
  }
}

tabBtnSpell.addEventListener("click", () => switchTab("spell"));
tabBtnErrors.addEventListener("click", () => switchTab("errors"));
tabBtnPublish.addEventListener("click", () => switchTab("publish"));

// ---------------------------------------------------------
// Editor Line Numbers
// ---------------------------------------------------------
function updateLineNumbers() {
  const lines = txtEditor.value.split("\n");
  const numLines = lines.length;
  const numsArray = [];
  for (let i = 1; i <= numLines; i++) {
    numsArray.push(i);
  }
  lineNums.innerHTML = numsArray.join("<br>");
}

txtEditor.addEventListener("input", () => {
  updateLineNumbers();
  lblPdfStatus.innerText = "";
  lblHtmlStatus.innerText = "";
  lblTexStatus.innerText = "";
  if (previewPane.classList.contains("active")) {
    triggerPreviewRender();
  }
});
txtEditor.addEventListener("scroll", () => {
  lineNums.scrollTop = txtEditor.scrollTop;
});


function applyLineNumbersState() {
  if (chkLineNums.checked) {
    lineNums.style.display = "block";
    txtEditor.style.whiteSpace = "pre";
    txtEditor.style.overflowX = "auto";
  } else {
    lineNums.style.display = "none";
    txtEditor.style.whiteSpace = "pre-wrap";
    txtEditor.style.overflowX = "hidden";
  }
}

chkLineNums.addEventListener("change", applyLineNumbersState);
applyLineNumbersState(); // Run on startup

// ---------------------------------------------------------
// Split Document Preview
// ---------------------------------------------------------
const resizerEP = document.getElementById("resizer-editor-preview");
const editorPaneEl = document.querySelector(".editor-pane");

btnTogglePreview.addEventListener("click", () => {
  const isActive = previewPane.classList.toggle("active");
  btnTogglePreview.classList.toggle("active", isActive);
  if (resizerEP) {
    resizerEP.style.display = isActive ? "block" : "none";
  }
  if (isActive) {
    triggerPreviewRender(true);
  } else if (editorPaneEl) {
    editorPaneEl.style.flex = "1 1 auto";
    editorPaneEl.style.width = "";
  }
});

// Scroll Sync between Editor and Preview
let isScrollingEditor = false;
let isScrollingPreview = false;

txtEditor.addEventListener("scroll", () => {
  if (!previewPane.classList.contains("active")) return;
  if (isScrollingPreview) {
    isScrollingPreview = false;
    return;
  }
  isScrollingEditor = true;
  const pct = txtEditor.scrollTop / (txtEditor.scrollHeight - txtEditor.clientHeight || 1);
  previewContent.scrollTop = pct * (previewContent.scrollHeight - previewContent.clientHeight);
});

previewContent.addEventListener("scroll", () => {
  if (!previewPane.classList.contains("active")) return;
  if (isScrollingEditor) {
    isScrollingEditor = false;
    return;
  }
  isScrollingPreview = true;
  const pct = previewContent.scrollTop / (previewContent.scrollHeight - previewContent.clientHeight || 1);
  txtEditor.scrollTop = pct * (txtEditor.scrollHeight - txtEditor.clientHeight);
});

// Workaround for macOS/WebKit drag-select autoscroll bug
let editorDragScrollInterval = null;
let isEditorMouseDown = false;
let dragScrollSpeed = 0;

txtEditor.addEventListener("mousedown", () => {
  isEditorMouseDown = true;
});

window.addEventListener("mouseup", () => {
  isEditorMouseDown = false;
  if (editorDragScrollInterval) {
    clearInterval(editorDragScrollInterval);
    editorDragScrollInterval = null;
  }
});

window.addEventListener("mousemove", (e) => {
  if (!isEditorMouseDown) return;
  
  const rect = txtEditor.getBoundingClientRect();
  const y = e.clientY;
  
  if (y > rect.bottom) {
    const distance = y - rect.bottom;
    dragScrollSpeed = Math.min(30, Math.max(5, distance * 0.3));
    if (!editorDragScrollInterval) {
      editorDragScrollInterval = setInterval(() => {
        txtEditor.scrollTop += dragScrollSpeed;
      }, 20);
    }
  } else if (y < rect.top) {
    const distance = rect.top - y;
    dragScrollSpeed = -Math.min(30, Math.max(5, distance * 0.3));
    if (!editorDragScrollInterval) {
      editorDragScrollInterval = setInterval(() => {
        txtEditor.scrollTop += dragScrollSpeed;
      }, 20);
    }
  } else {
    if (editorDragScrollInterval) {
      clearInterval(editorDragScrollInterval);
      editorDragScrollInterval = null;
    }
  }
});

let isPreviewRendering = false;
let pendingPreviewUpdate = false;
let mathJaxPromise = Promise.resolve();

function typesetMath(element) {
  if (window.MathJax && window.MathJax.typesetPromise) {
    mathJaxPromise = mathJaxPromise
      .then(() => {
        if (window.MathJax.typesetClear) {
          try { window.MathJax.typesetClear([element]); } catch (_) {}
        }
        return window.MathJax.typesetPromise([element]);
      })
      .catch((err) => {
        console.warn("MathJax typeset notice:", err);
      });
  }
}

function triggerPreviewRender(immediate = false) {
  if (previewTimeout) {
    clearTimeout(previewTimeout);
  }
  if (immediate) {
    updatePreview();
  } else {
    previewTimeout = setTimeout(updatePreview, 300);
  }
}

function escapePreviewText(value) {
  const element = document.createElement("div");
  element.textContent = String(value);
  return element.innerHTML;
}

async function updatePreview() {
  if (!previewPane || !previewPane.classList.contains("active")) return;

  if (isPreviewRendering) {
    pendingPreviewUpdate = true;
    return;
  }

  isPreviewRendering = true;
  pendingPreviewUpdate = false;
  const requestId = ++previewRequestId;
  const content = txtEditor.value;

  const isTex = currentFile
    ? currentFile.toLowerCase().endsWith(".tex")
    : (detectFileTypeFromContent() === "tex" || content.includes("\\documentclass") || content.includes("\\begin{document}"));

  try {
    if (isTex) {
      if (!previewContent.hasChildNodes() || previewContent.innerHTML.trim() === "") {
        previewContent.innerHTML = `<div class="preview-placeholder">Rendering LaTeX preview…</div>`;
      }

      const htmlContent = await invoke("render_latex_preview", {
        content: content,
        path: currentFile || null
      });

      if (requestId === previewRequestId && previewPane.classList.contains("active")) {
        const savedScroll = previewContent.scrollTop;
        previewContent.innerHTML = htmlContent || `<div class="preview-placeholder">The document has no previewable body content.</div>`;
        previewContent.scrollTop = savedScroll;
        typesetMath(previewContent);
      }
    } else {
      if (window.marked && window.marked.parse) {
        const htmlContent = window.marked.parse(content);
        const savedScroll = previewContent.scrollTop;
        previewContent.innerHTML = htmlContent;
        previewContent.scrollTop = savedScroll;
      } else {
        previewContent.innerHTML = `<div class="preview-placeholder">Markdown parser loading...</div>`;
      }
      typesetMath(previewContent);
    }
  } catch (err) {
    if (!previewContent.hasChildNodes() || previewContent.querySelector(".preview-placeholder")) {
      previewContent.innerHTML = `
        <div class="preview-placeholder preview-error">
          <strong>LaTeX preview could not be rendered.</strong>
          <span>${escapePreviewText(err)}</span>
          <small>PDF compilation remains the authoritative check for packages and custom commands.</small>
        </div>`;
    }
  } finally {
    isPreviewRendering = false;
    if (pendingPreviewUpdate) {
      triggerPreviewRender(false);
    }
  }
}

// ---------------------------------------------------------
// Find and Replace
// ---------------------------------------------------------
function findNext() {
  const query = txtFind.value;
  if (!query) return;
  const text = txtEditor.value;
  if (query !== lastQuery) {
    searchIndex = 0;
    lastQuery = query;
  }
  let idx = text.toLowerCase().indexOf(query.toLowerCase(), searchIndex);
  if (idx === -1) {
    idx = text.toLowerCase().indexOf(query.toLowerCase(), 0); // wrap
  }
  if (idx !== -1) {
    txtEditor.focus();
    txtEditor.setSelectionRange(idx, idx + query.length);
    searchIndex = idx + query.length;
    scrollEditorToOffset(idx);
    lblFindStatus.innerText = "Match found";
    lblFindStatus.style.color = "#30d158";
  } else {
    lblFindStatus.innerText = "No match found";
    lblFindStatus.style.color = "#ff453a";
  }
}

function findPrev() {
  const query = txtFind.value;
  if (!query) return;
  const text = txtEditor.value;
  if (query !== lastQuery) {
    searchIndex = text.length;
    lastQuery = query;
  }
  let searchStart = searchIndex - query.length - 1;
  if (searchStart < 0) searchStart = text.length;
  let idx = text.toLowerCase().lastIndexOf(query.toLowerCase(), searchStart);
  if (idx === -1) {
    idx = text.toLowerCase().lastIndexOf(query.toLowerCase(), text.length); // wrap
  }
  if (idx !== -1) {
    txtEditor.focus();
    txtEditor.setSelectionRange(idx, idx + query.length);
    searchIndex = idx;
    scrollEditorToOffset(idx);
    lblFindStatus.innerText = "Match found";
    lblFindStatus.style.color = "#30d158";
  } else {
    lblFindStatus.innerText = "No match found";
    lblFindStatus.style.color = "#ff453a";
  }
}

function scrollEditorToOffset(offset) {
  const text = txtEditor.value;
  const lineIndex = text.substring(0, offset).split("\n").length;
  const lineHeight = 20.8;
  txtEditor.scrollTop = (lineIndex - 5) * lineHeight;
}

btnFindNext.addEventListener("click", findNext);
btnFindPrev.addEventListener("click", findPrev);

btnReplace.addEventListener("click", () => {
  const query = txtFind.value;
  const repVal = txtReplace.value;
  if (!query) return;
  const start = txtEditor.selectionStart;
  const end = txtEditor.selectionEnd;
  const text = txtEditor.value;
  if (start !== end && text.substring(start, end).toLowerCase() === query.toLowerCase()) {
    txtEditor.value = text.substring(0, start) + repVal + text.substring(end);
    txtEditor.setSelectionRange(start, start + repVal.length);
    updateLineNumbers();
  } else {
    findNext();
  }
});

btnReplaceAll.addEventListener("click", () => {
  const query = txtFind.value;
  const repVal = txtReplace.value;
  if (!query) return;
  const text = txtEditor.value;
  const regex = new RegExp(query.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&'), 'gi');
  txtEditor.value = text.replace(regex, repVal);
  updateLineNumbers();
});

// ---------------------------------------------------------
// File Loader & State Logic
// ---------------------------------------------------------
function updateRecentFilesDropdown() {
  let recents = [];
  try {
    recents = JSON.parse(localStorage.getItem("recentFiles") || "[]");
  } catch (e) {
    recents = [];
  }
  
  if (recents.length === 0) {
    selectRecentFiles.style.display = "none";
    return;
  }
  
  selectRecentFiles.innerHTML = `<option value="" disabled selected>Open Recent...</option>`;
  recents.forEach(path => {
    const parts = path.split(/[\/\\]/);
    const name = parts[parts.length - 1] || path;
    selectRecentFiles.innerHTML += `<option value="${path}" title="${path}">${name}</option>`;
  });
  selectRecentFiles.style.display = "inline-block";
}

function addFileToRecents(path) {
  if (!path) return;
  let recents = [];
  try {
    recents = JSON.parse(localStorage.getItem("recentFiles") || "[]");
  } catch (e) {
    recents = [];
  }
  
  recents = recents.filter(p => p !== path);
  recents.unshift(path);
  if (recents.length > 10) {
    recents = recents.slice(0, 10);
  }
  
  localStorage.setItem("recentFiles", JSON.stringify(recents));
  updateRecentFilesDropdown();
}

selectRecentFiles.addEventListener("change", () => {
  const path = selectRecentFiles.value;
  if (path) {
    loadDocument(path);
  }
});

async function openDocument() {
  const path = await invoke("select_file", { fileType: "doc" });
  if (path) {
    loadDocument(path);
  }
}

async function loadDocument(path) {
  try {
    // Clean up auxiliary files from any previous failed compiles
    await invoke("clean_aux_files", { path }).catch(err => console.warn("Failed to clean auxiliary files: " + err));
    
    const content = await invoke("read_file", { path });
    currentFile = path;
    addFileToRecents(path);
    lblFilePath.innerText = path;
    txtEditor.value = content;
    updateLineNumbers();
    
    // Reset file-type workflow flags
    lastPdf = "";
    lastHtml = "";
    lastTex = "";
    lastDocx = "";
    lblPdfStatus.innerText = "";
    lblHtmlStatus.innerText = "";
    lblTexStatus.innerText = "";
    lblDocxStatus.innerText = "";
    btnOpenPdf.disabled = true;
    btnOpenHtml.disabled = true;
    btnOpenTex.disabled = true;
    btnOpenDocx.disabled = true;
    
    btnSaveDoc.disabled = false;
    btnSaveAsDoc.disabled = false;
    btnScanSpell.disabled = false;
    btnBrowseBib.disabled = false;
    btnBrowseCsl.disabled = false;
    btnTogglePreview.disabled = false;
    
    if (previewPane.classList.contains("active")) {
      triggerPreviewRender(true);
    }
    
    updateUIForFileType();
    logConsole("Loaded file: " + path);
  } catch (e) {
    alert("Error reading file: " + e);
  }
}

function detectFileTypeFromContent() {
  const text = txtEditor.value;
  if (text.includes("\\documentclass") || text.includes("\\begin{document}") || text.includes("\\section{") || text.includes("\\chapter{")) {
    return "tex";
  }
  return "md";
}

async function saveDocument() {
  if (!currentFile) {
    const detectedType = detectFileTypeFromContent();
    const path = await invoke("select_save_file", { fileType: detectedType });
    if (path) {
      currentFile = path;
      lblFilePath.innerText = path;
      updateUIForFileType();
    } else {
      return; // User cancelled
    }
  }
  
  try {
    await invoke("write_file", { path: currentFile, content: txtEditor.value });
    addFileToRecents(currentFile);
    const status = document.getElementById("lbl-save-status");
    status.innerText = "Saved successfully";
    setTimeout(() => {
      status.innerText = "";
    }, 3000);
    logConsole("Saved file successfully.");
  } catch (e) {
    alert("Error saving file: " + e);
  }
}

async function saveDocumentAs() {
  const currentExt = currentFile ? currentFile.split('.').pop().toLowerCase() : "md";
  const path = await invoke("select_save_file", { fileType: currentExt });
  if (path) {
    currentFile = path;
    lblFilePath.innerText = path;
    updateUIForFileType();
    
    try {
      await invoke("write_file", { path: currentFile, content: txtEditor.value });
      addFileToRecents(currentFile);
      const status = document.getElementById("lbl-save-status");
      status.innerText = "Saved successfully";
      setTimeout(() => {
        status.innerText = "";
      }, 3000);
      logConsole("Saved file successfully as: " + currentFile);
    } catch (e) {
      alert("Error saving file: " + e);
    }
  }
}

btnOpenDoc.addEventListener("click", openDocument);
btnNewDoc.addEventListener("click", () => {
  currentFile = null;
  lblFilePath.innerText = "No document loaded";
  const defaultId = localStorage.getItem(DEFAULT_PREAMBLE_STORAGE_KEY);
  const defaultPreamble = loadPreambles().find(item => item.id === defaultId);
  txtEditor.value = defaultPreamble?.content || "";
  updateLineNumbers();
  
  lastPdf = "";
  lastHtml = "";
  lastTex = "";
  lastDocx = "";
  lblPdfStatus.innerText = "";
  lblHtmlStatus.innerText = "";
  lblTexStatus.innerText = "";
  lblDocxStatus.innerText = "";
  btnOpenPdf.disabled = true;
  btnOpenHtml.disabled = true;
  btnOpenTex.disabled = true;
  btnOpenDocx.disabled = true;
  
  updateUIForFileType();
  
  previewContent.innerHTML = `<div class="preview-placeholder">Write some Markdown content to preview.</div>`;
});
btnSaveDoc.addEventListener("click", saveDocument);
btnSaveAsDoc.addEventListener("click", saveDocumentAs);

function updateUIForFileType() {
  if (!currentFile) {
    btnBuildPdf.disabled = true;
    btnBuildHtml.disabled = true;
    btnBuildTex.disabled = true;
    btnBuildDocx.disabled = true;
    return;
  }
  
  const isTex = currentFile.toLowerCase().endsWith(".tex");
  
  // Enable compiler options appropriately
  btnBuildPdf.disabled = false;
  btnBuildHtml.disabled = false;
  btnBuildDocx.disabled = false;
  btnBuildTex.disabled = isTex; // LaTeX source export is only for Markdown (.md) files
  
  // Disable options that only apply to Markdown files when editing LaTeX (.tex)
  chkCiteproc.disabled = isTex;
  chkToc.disabled = isTex;
  chkNum.disabled = isTex;
  chkLeftAlign.disabled = false; // Keep enabled for direct LaTeX preamble syncing
  chkUnicodeMath.disabled = isTex;
  btnBrowseCsl.disabled = isTex;
  selectPdfPageSize.disabled = false; // Keep enabled for direct LaTeX preamble syncing
  selectPdfMargin.disabled = false; // Keep enabled for direct LaTeX preamble syncing
  selectHtmlTableWidth.disabled = isTex;
  selectHtmlTableStyle.disabled = isTex;
  
  if (isTex) {
    selectPreset.innerHTML = `
      <option value="Custom">Custom</option>
      <option value="Formal PDF (Print/arXiv)">Formal PDF (Print/arXiv)</option>
      <option value="Substack HTML (Dark Mode)">Substack HTML (Dark Mode)</option>
      <option value="Medium HTML (Light Mode)">Medium HTML (Light Mode)</option>
    `;
    
    // Parse LaTeX preamble to auto-populate layout options
    const text = txtEditor.value;
    const geoRegex = /\\usepackage\s*\[([^\]]*)\]\s*\{geometry\}/;
    const geoMatch = text.match(geoRegex);
    if (geoMatch) {
      const opts = geoMatch[1];
      if (opts.includes("a4paper")) selectPdfPageSize.value = "A4";
      else if (opts.includes("letterpaper")) selectPdfPageSize.value = "Letter";
      else if (opts.includes("legalpaper")) selectPdfPageSize.value = "Legal";
      else if (opts.includes("a5paper")) selectPdfPageSize.value = "A5";
      else if (opts.includes("a3paper")) selectPdfPageSize.value = "A3";
      else if (opts.includes("papersize={6in,9in}")) selectPdfPageSize.value = "6x9";
      else if (opts.includes("papersize={5.5in,8.5in}")) selectPdfPageSize.value = "5.5x8.5";
      else if (opts.includes("papersize={5in,8in}")) selectPdfPageSize.value = "5x8";
      else if (opts.includes("papersize={7in,10in}")) selectPdfPageSize.value = "7x10";
      else if (opts.includes("papersize={8in,10in}")) selectPdfPageSize.value = "8x10";
      else {
        // Check for custom papersize
        const customMatch = opts.match(/papersize=\{([^,]*),([^}]*)\}/);
        if (customMatch) {
          selectPdfPageSize.value = "custom";
          txtPdfCustomW.value = customMatch[1].trim();
          txtPdfCustomH.value = customMatch[2].trim();
        }
      }
      
      const marginMatch = opts.match(/margin=([^,\s\]]*)/);
      if (marginMatch) {
        selectPdfMargin.value = marginMatch[1];
      }
    }
    
    const raggedRegex = /\\usepackage\s*\[document\]\s*\{ragged2e\}/;
    chkLeftAlign.checked = raggedRegex.test(text);
  } else {
    selectPreset.innerHTML = `
      <option value="Custom">Custom</option>
      <option value="Formal PDF (Print/arXiv)">Formal PDF (Print/arXiv)</option>
      <option value="Substack HTML (Dark Mode)">Substack HTML (Dark Mode)</option>
      <option value="Medium HTML (Light Mode)">Medium HTML (Light Mode)</option>
      <option value="LaTeX Source (.tex)">LaTeX Source (.tex)</option>
    `;
  }
  
  if (selectPdfPageSize.value === "custom") {
    divPdfCustomSize.style.display = "flex";
  } else {
    divPdfCustomSize.style.display = "none";
  }
}

// ---------------------------------------------------------
// Preset Selection Handler
// ---------------------------------------------------------
selectPreset.addEventListener("change", () => {
  const preset = selectPreset.value;
  if (preset === "Formal PDF (Print/arXiv)") {
    selectEngine.value = "lualatex";
    chkCiteproc.checked = true;
    chkToc.checked = true;
    chkNum.checked = true;
    chkLeftAlign.checked = false;
    chkUnicodeMath.checked = true;
  } else if (preset === "Substack HTML (Dark Mode)") {
    selectMathStyle.value = "PNG (Standard)";
    selectMathFg.value = "White (Dark Mode)";
    selectMathBg.value = "Transparent";
    selectMathSize.value = "160 (Large)";
    chkConvertInline.checked = true;
    chkAutocopy.checked = true;
  } else if (preset === "Medium HTML (Light Mode)") {
    selectMathStyle.value = "PNG (Standard)";
    selectMathFg.value = "Black (Light Mode)";
    selectMathBg.value = "Transparent";
    selectMathSize.value = "160 (Large)";
    chkConvertInline.checked = true;
    chkAutocopy.checked = true;
  }
});

// ---------------------------------------------------------
// Spell Checker Implementation
// ---------------------------------------------------------
function scanMarkdownFormatting(text) {
  const errors = [];
  const lines = text.split("\n");
  
  const ext = currentFile ? (currentFile.toLowerCase().endsWith(".tex") ? "tex" : "md") : detectFileTypeFromContent();
  const isTex = (ext === "tex");
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const lineNum = i + 1;
    
    if (!isTex) {
      // 1. Check for mismatched double asterisks (bold)
      const boldCount = (line.match(/\*\*/g) || []).length;
      if (boldCount % 2 !== 0) {
        errors.push({
          word: "** (Mismatched Bold)",
          line: lineNum,
          context: "Line has an odd number of '**' bold markers."
        });
      }
      
      // 2. Check for mismatched single asterisks (italics)
      const trimmed = line.trim();
      const cleanLine = line.replace(/\*\*/g, "");
      const italicCount = (cleanLine.match(/\*/g) || []).length;
      if (italicCount % 2 !== 0 && !trimmed.startsWith("* ")) {
        errors.push({
          word: "* (Mismatched Italic)",
          line: lineNum,
          context: "Line has an odd number of '*' italic markers."
        });
      }
      
      // 3. Check for mismatched backticks (inline code)
      const codeCount = (line.match(/`/g) || []).length;
      if (codeCount % 2 !== 0) {
        errors.push({
          word: "` (Mismatched Code)",
          line: lineNum,
          context: "Line has an odd number of '`' backtick code markers."
        });
      }
    }
    
    // 4. Check for Unicode Em Dash "—"
    if (line.includes("—")) {
      errors.push({
        word: "—",
        line: lineNum,
        context: isTex ? "Found Em Dash. Suggest ' -- ' (En Dash with spaces)" : "Found Em Dash. Suggest ' – ' (En Dash with spaces)",
        suggested: isTex ? " -- " : " – "
      });
    }
    
    // 5. Check for LaTeX Em Dash "---"
    if (line.includes("---")) {
      errors.push({
        word: "---",
        line: lineNum,
        context: isTex ? "Found LaTeX Em Dash. Suggest ' -- ' (En Dash with spaces)" : "Found LaTeX Em Dash. Suggest ' – ' (En Dash with spaces)",
        suggested: isTex ? " -- " : " – "
      });
    }
  }
  return errors;
}

async function scanSpelling() {
  logConsole("Running spell check scan...");
  errorsTbody.innerHTML = `<tr><td colspan="3" class="placeholder-text">Scanning code spelling and markdown formatting...</td></tr>`;
  
  const ext = currentFile ? (currentFile.toLowerCase().endsWith(".tex") ? "tex" : "md") : detectFileTypeFromContent();
  
  let backendErrors = [];
  try {
    const res = await invoke("run_spell_check", { 
      content: txtEditor.value, 
      fileType: ext,
      docPath: currentFile,
      ignoreList: [],
      language: selectSpellLang.value,
      forceIse: chkForceIse.checked
    });
    backendErrors = JSON.parse(res);
  } catch (e) {
    logConsole("Backend spell check failed: " + e);
  }
  
  try {
    const formattingErrors = scanMarkdownFormatting(txtEditor.value);
    spellingErrors = [...formattingErrors, ...backendErrors];
    renderSpellingErrors();
  } catch (e) {
    errorsTbody.innerHTML = `<tr><td colspan="3" class="placeholder-text" style="color:#ff453a;">Spell check failed: ${e}</td></tr>`;
  }
}

function renderSpellingErrors() {
  if (spellingErrors.length === 0) {
    errorsTbody.innerHTML = `<tr><td colspan="3" class="placeholder-text" style="color:#30d158;">No spelling or formatting errors found!</td></tr>`;
    resetResolverPanel();
    return;
  }
  
  let html = "";
  spellingErrors.forEach((err, idx) => {
    const isFormatting = err.word.includes("(Mismatched");
    const labelStyle = isFormatting ? "color: var(--error-color); font-weight: 600;" : "font-weight: 500;";
    html += `
      <tr onclick="selectSpellingError(${idx})" id="err-row-${idx}">
        <td style="${labelStyle}">${err.word}</td>
        <td>${err.line}</td>
        <td style="color:var(--text-secondary); font-style:italic;">${err.context}</td>
      </tr>
    `;
  });
  errorsTbody.innerHTML = html;
}

window.selectSpellingError = async function(idx) {
  selectedErrorIndex = idx;
  
  // Highlight row in table
  const rows = errorsTbody.querySelectorAll("tr");
  rows.forEach(r => r.classList.remove("selected"));
  document.getElementById(`err-row-${idx}`).classList.add("selected");
  
  const err = spellingErrors[idx];
  txtOrigWord.value = err.word;
  
  if (err.suggested) {
    txtReplaceVal.value = err.suggested;
  } else {
    txtReplaceVal.value = err.word;
  }
  
  // Scroll to word in editor
  scrollEditorToLineAndHighlight(err.line, err.word);
  
  const isFormatting = err.word.includes("(Mismatched");
  const isDash = (err.word === "—" || err.word === "---");
  
  // Enable resolver buttons
  btnSpellReplace.disabled = isFormatting;
  btnSpellReplaceAll.disabled = isFormatting;
  btnSpellIgnore.disabled = isFormatting || isDash;
  btnSpellAdd.disabled = isFormatting || isDash;
  
  if (isFormatting) {
    txtReplaceVal.value = "";
    listSuggestions.innerHTML = "<span>Please edit the document directly to fix formatting errors.</span>";
  } else if (isDash) {
    txtReplaceVal.value = err.suggested;
    listSuggestions.innerHTML = "";
    const btn = document.createElement("button");
    btn.className = "suggestion-btn";
    btn.textContent = err.suggested;
    btn.onclick = () => selectSuggestion(err.suggested);
    listSuggestions.appendChild(btn);
  } else {
    // Load suggestions
    listSuggestions.innerHTML = "<span>Loading...</span>";
    try {
      const selectedLang = selectSpellLang ? selectSpellLang.value : "en-GB";
      const suggs = await invoke("fetch_suggestions", { word: err.word, language: selectedLang });
      listSuggestions.innerHTML = "";
      
      let allSuggs = [...suggs];
      if (err.suggested) {
        allSuggs = allSuggs.filter(s => s.toLowerCase() !== err.suggested.toLowerCase());
        allSuggs.unshift(err.suggested);
      }
      
      if (allSuggs.length === 0) {
        listSuggestions.innerHTML = "<span>No suggestions found</span>";
      } else {
        txtReplaceVal.value = allSuggs[0];
        
        allSuggs.forEach(s => {
          const btn = document.createElement("button");
          btn.className = "suggestion-btn";
          btn.textContent = s;
          btn.onclick = () => selectSuggestion(s);
          listSuggestions.appendChild(btn);
        });
      }
    } catch (e) {
      listSuggestions.innerHTML = "<span>Failed to load suggestions</span>";
    }
  }
};

window.selectSuggestion = function(word) {
  txtReplaceVal.value = word;
};

function scrollEditorToLineAndHighlight(lineNum, word) {
  const text = txtEditor.value;
  const lines = text.split("\n");
  if (lineNum > 0 && lineNum <= lines.length) {
    const lineText = lines[lineNum - 1];
    const lineOffset = lines.slice(0, lineNum - 1).join("\n").length + (lineNum > 1 ? 1 : 0);
    
    let searchWord = word;
    let wordIdx = -1;
    
    // Check if the word is a formatting warning label
    if (word.startsWith("** (Mismatched")) {
      searchWord = "**";
      wordIdx = lineText.lastIndexOf("**");
    } else if (word.startsWith("* (Mismatched")) {
      searchWord = "*";
      wordIdx = lineText.lastIndexOf("*");
    } else if (word.startsWith("` (Mismatched")) {
      searchWord = "`";
      wordIdx = lineText.lastIndexOf("`");
    } else {
      wordIdx = lineText.indexOf(word);
    }
    
    // Highlight the target or fallback to the whole line
    if (wordIdx === -1) {
      txtEditor.focus();
      txtEditor.setSelectionRange(lineOffset, lineOffset + lineText.length);
      scrollEditorToOffset(lineOffset);
    } else {
      txtEditor.focus();
      txtEditor.setSelectionRange(lineOffset + wordIdx, lineOffset + wordIdx + searchWord.length);
      scrollEditorToOffset(lineOffset + wordIdx);
    }
  }
}

let currentCompileErrors = [];
let selectedCompileErrorIndex = -1;

function findUnbalancedBraces() {
  if (!currentFile || !currentFile.toLowerCase().endsWith(".tex")) {
    return [];
  }
  const text = txtEditor.value;
  const lines = text.split("\n");
  const stack = [];
  const errors = [];
  let insideVerbatim = false;
  
  for (let lineIdx = 0; lineIdx < lines.length; lineIdx++) {
    const lineText = lines[lineIdx];
    
    if (lineText.includes("\\begin{verbatim}")) {
      insideVerbatim = true;
      continue;
    }
    if (lineText.includes("\\end{verbatim}")) {
      insideVerbatim = false;
      continue;
    }
    if (insideVerbatim) {
      continue;
    }
    
    for (let colIdx = 0; colIdx < lineText.length; colIdx++) {
      const char = lineText[colIdx];
      
      if (char === "%" && (colIdx === 0 || lineText[colIdx - 1] !== "\\")) {
        break;
      }
      
      if (colIdx > 0 && lineText[colIdx - 1] === "\\") {
        continue;
      }
      
      if (char === "{") {
        stack.push({ line: lineIdx + 1, col: colIdx + 1 });
      } else if (char === "}") {
        if (stack.length > 0) {
          stack.pop();
        } else {
          errors.push({
            type: "closing",
            line: lineIdx + 1,
            col: colIdx + 1,
            message: `Unmatched closing brace '}' found on line ${lineIdx + 1}.`
          });
        }
      }
    }
  }
  
  while (stack.length > 0) {
    const openBrace = stack.pop();
    errors.push({
      type: "opening",
      line: openBrace.line,
      col: openBrace.col,
      message: `Opening brace '{' on line ${openBrace.line} is never closed.`
    });
  }
  
  errors.sort((a, b) => a.line - b.line);
  return errors;
}

const commonCommands = [
  "\\begin", "\\end", "\\equation", "\\begin{equation}", "\\end{equation}",
  "\\begin{document}", "\\end{document}", "\\usepackage", "\\cite", "\\ref",
  "\\label", "\\section", "\\subsection", "\\textbf", "\\textit", "\\frac",
  "\\right", "\\left", "\\alpha", "\\beta", "\\sigma", "\\tau", "\\mathbf",
  "\\mathrm", "\\begin{picture}", "\\end{picture}", "\\begin{tikzpicture}",
  "\\end{tikzpicture}", "\\bibitem"
];

function suggestCommandCorrection(typo) {
  let bestMatch = "";
  let minDistance = 999;
  for (let cmd of commonCommands) {
    let dist = LevenshteinDistance(typo, cmd);
    if (dist < minDistance && dist <= 3) {
      minDistance = dist;
      bestMatch = cmd;
    }
  }
  return bestMatch;
}

function LevenshteinDistance(a, b) {
  const matrix = [];
  for (let i = 0; i <= b.length; i++) matrix[i] = [i];
  for (let j = 0; j <= a.length; j++) matrix[0][j] = j;
  
  for (let i = 1; i <= b.length; i++) {
    for (let j = 1; j <= a.length; j++) {
      if (b.charAt(i - 1) === a.charAt(j - 1)) {
        matrix[i][j] = matrix[i - 1][j - 1];
      } else {
        matrix[i][j] = Math.min(
          matrix[i - 1][j - 1] + 1,
          matrix[i][j - 1] + 1,
          matrix[i - 1][j] + 1
        );
      }
    }
  }
  return matrix[b.length][a.length];
}

function clearCompileErrors() {
  currentCompileErrors = [];
  selectedCompileErrorIndex = -1;
  compileErrorsTbody.innerHTML = `
    <tr>
      <td colspan="2" class="placeholder-text">No compilation errors detected.</td>
    </tr>
  `;
  compileResolverPanel.style.display = "none";
}

function populateCompileErrors(errors) {
  currentCompileErrors = errors;
  selectedCompileErrorIndex = -1;
  compileResolverPanel.style.display = "none";
  
  if (errors.length === 0) {
    compileErrorsTbody.innerHTML = `
      <tr>
        <td colspan="2" class="placeholder-text">No errors parsed from log. Check console output.</td>
      </tr>
    `;
    return;
  }
  
  compileErrorsTbody.innerHTML = "";
  errors.forEach((err, idx) => {
    const row = document.createElement("tr");
    row.style.cursor = "pointer";
    row.innerHTML = `
      <td style="font-weight: bold; color: var(--accent-color);">${err.line}</td>
      <td style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 250px;">${escapeHtml(err.message)}</td>
    `;
    row.addEventListener("click", () => selectCompileError(idx));
    compileErrorsTbody.appendChild(row);
  });
  
  switchTab("errors");
  selectCompileError(0);
}

function selectCompileError(idx) {
  selectedCompileErrorIndex = idx;
  const err = currentCompileErrors[idx];
  
  const rows = compileErrorsTbody.querySelectorAll("tr");
  rows.forEach((r, i) => {
    if (i === idx) {
      r.style.background = "rgba(100, 200, 255, 0.1)";
    } else {
      r.style.background = "";
    }
  });
  
  lblCompileErrMsg.innerText = err.message;
  
  const text = txtEditor.value;
  const lines = text.split("\n");
  const isLineBlank = (err.line >= 1 && err.line <= lines.length && lines[err.line - 1].trim() === "");
  
  let suggestionText = err.suggestion;
  
  if (err.message.toLowerCase().includes("undefined control sequence")) {
    const cmdMatches = err.context.match(/\\[a-zA-Z]+/g);
    if (cmdMatches) {
      for (let typoCmd of cmdMatches) {
        const correctCmd = suggestCommandCorrection(typoCmd);
        if (correctCmd) {
          suggestionText = `Undefined control sequence '${typoCmd}'. Did you mean '${correctCmd}'? Click 'Apply Suggested Fix' to auto-replace it.`;
          err.suggestedReplacement = { typo: typoCmd, correct: correctCmd };
          break;
        }
      }
    }
  }

  const msgLower = err.message.toLowerCase();
  const hasSafeAutomaticFix = Boolean(
    err.localType === "opening" ||
    err.localType === "closing" ||
    err.suggestedReplacement ||
    msgLower.includes("missing $") ||
    msgLower.includes("display math") ||
    msgLower.includes("bad math environment") ||
    msgLower.includes("eqno") ||
    msgLower.includes("missing \\begin{document}") ||
    msgLower.includes("missing begin{document}") ||
    msgLower.includes("extra \\right") ||
    msgLower.includes("ended by \\end{figure}") ||
    (msgLower.includes("ended by \\end") && err.context.trim() === "\\end{pmatrix}")
  );
  btnApplyCompileFix.disabled = !hasSafeAutomaticFix;
  btnApplyCompileFix.innerText = hasSafeAutomaticFix ? "Apply Suggested Fix" : "Manual Fix Required";
  
  if (isLineBlank) {
    lblCompileSuggestion.innerHTML = suggestionText + "<br><br><span style='color: var(--accent-color); font-weight: bold;'>💡 Note: Since the highlighted line is blank, this is a paragraph boundary. The actual error is likely located inside this paragraph on one of the preceding lines.</span>";
  } else {
    lblCompileSuggestion.innerText = suggestionText;
  }
  
  compileResolverPanel.style.display = "block";
  
  scrollEditorToLineAndHighlight(err.line, err.context || "");
}

function escapeHtml(str) {
  if (!str) return "";
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

async function applyCompileFix() {
  if (selectedCompileErrorIndex === -1) return;
  const err = currentCompileErrors[selectedCompileErrorIndex];
  if (btnApplyCompileFix.disabled) return;
  
  const text = txtEditor.value;
  const lines = text.split("\n");
  if (err.line < 1 || err.line > lines.length) return;
  
  const lineText = lines[err.line - 1];
  let newLineText = lineText;
  let linesSpliced = false;
  
  const msg = err.message.toLowerCase();
  
  const isMathError = msg.includes("missing $") || 
                     msg.includes("display math") || 
                     msg.includes("bad math environment") || 
                     msg.includes("eqno");
                     
  if (err.localType === "opening") {
    newLineText = lineText + "}";
  } else if (err.localType === "closing") {
    if (err.col >= 1 && err.col <= lineText.length) {
      newLineText = lineText.substring(0, err.col - 1) + lineText.substring(err.col);
    }
  } else if (isMathError) {
    let blankLineIdx = -1;
    const checkIndices = [err.line - 2, err.line - 1, err.line];
    for (let idx of checkIndices) {
      if (idx >= 0 && idx < lines.length && lines[idx].trim() === "") {
        blankLineIdx = idx;
        break;
      }
    }
    
    if (blankLineIdx !== -1) {
      lines.splice(blankLineIdx, 1);
      linesSpliced = true;
    } else {
      alert("No blank lines found near this equation. Please make sure there are no empty lines inside your math environments.");
      return;
    }
  } else if (msg.includes("missing \\begin{document}") || msg.includes("missing begin{document}")) {
    newLineText = "% " + lineText;
  } else if (err.suggestedReplacement) {
    const typo = err.suggestedReplacement.typo;
    const correct = err.suggestedReplacement.correct;
    newLineText = lineText.replace(typo, correct);
  } else if (msg.includes("extra \\right")) {
    newLineText = lineText.replace(/\\right\./g, "").replace(/\\right/g, "");
  } else if (msg.includes("ended by \\end{figure}") && lineText.includes("\\end{figure}")) {
    newLineText = lineText.replace(/\\end\{figure\}/g, "\\end{picture}");
  } else if (msg.includes("ended by \\end") && lineText.trim() === "\\end{pmatrix}") {
    lines.splice(err.line - 1, 1);
    linesSpliced = true;
  } else {
    alert("No automatic fix is defined for this error. Please correct it manually in the editor.");
    return;
  }
  
  if (!linesSpliced && newLineText === lineText) {
    alert("Could not locate target text to replace. Please correct it manually in the editor.");
    return;
  }
  
  if (!linesSpliced) {
    lines[err.line - 1] = newLineText;
  }
  
  txtEditor.value = lines.join("\n");
  updateLineNumbers();
  await saveDocument();
  
  alert("Suggested fix applied successfully! Please re-compile to verify.");
  clearCompileErrors();
}

btnApplyCompileFix.addEventListener("click", applyCompileFix);

function resetResolverPanel() {
  txtOrigWord.value = "";
  txtReplaceVal.value = "";
  listSuggestions.innerHTML = "";
  btnSpellReplace.disabled = true;
  btnSpellReplaceAll.disabled = true;
  btnSpellIgnore.disabled = true;
  btnSpellAdd.disabled = true;
  selectedErrorIndex = -1;
}

btnScanSpell.addEventListener("click", scanSpelling);

// Resolve Handlers
function preserveCase(original, replacement) {
  if (!original || !replacement) return replacement;
  if (original === original.toUpperCase() && original !== original.toLowerCase()) {
    return replacement.toUpperCase();
  }
  if (original[0] === original[0].toUpperCase() && original[0] !== original[0].toLowerCase()) {
    return replacement[0].toUpperCase() + replacement.slice(1);
  }
  if (original === original.toLowerCase() && original !== original.toUpperCase()) {
    return replacement.toLowerCase();
  }
  return replacement;
}

btnSpellReplace.addEventListener("click", () => {
  if (selectedErrorIndex === -1) return;
  const err = spellingErrors[selectedErrorIndex];
  const repVal = txtReplaceVal.value;
  const text = txtEditor.value;
  
  const lines = text.split("\n");
  if (err.line <= lines.length) {
    const lineText = lines[err.line - 1];
    const escapedWord = err.word.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&');
    const regex = new RegExp(escapedWord, 'i');
    const match = lineText.match(regex);
    
    if (match && match.index !== undefined) {
      const wordIdx = match.index;
      const matchedWord = match[0];
      const repCase = preserveCase(matchedWord, repVal);
      
      const lineOffset = lines.slice(0, err.line - 1).join("\n").length + (err.line > 1 ? 1 : 0);
      txtEditor.value = text.substring(0, lineOffset + wordIdx) + repCase + text.substring(lineOffset + wordIdx + matchedWord.length);
      updateLineNumbers();
      
      logConsole(`Replaced occurrence of '${err.word}' on line ${err.line} with '${repCase}'.`);
      
      const oldText = btnSpellReplace.innerText;
      btnSpellReplace.innerText = "✅ Replaced!";
      btnSpellReplace.disabled = true;
      setTimeout(() => {
        btnSpellReplace.innerText = oldText;
        btnSpellReplace.disabled = false;
      }, 1500);
      
      spellingErrors.splice(selectedErrorIndex, 1);
      renderSpellingErrors();
      resetResolverPanel();
    } else {
      // Word was not found on the line (e.g. user corrected it manually in the editor)
      logConsole(`Word '${err.word}' no longer found on line ${err.line} (already corrected manually). Removing from list.`);
      spellingErrors.splice(selectedErrorIndex, 1);
      renderSpellingErrors();
      resetResolverPanel();
    }
  }
});

btnSpellReplaceAll.addEventListener("click", () => {
  if (selectedErrorIndex === -1) return;
  const err = spellingErrors[selectedErrorIndex];
  const repVal = txtReplaceVal.value;
  const text = txtEditor.value;
  
  const escapedWord = err.word.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&');
  const regex = new RegExp(escapedWord, 'gi');
  const matchCount = (text.match(regex) || []).length;
  
  txtEditor.value = text.replace(regex, (match) => preserveCase(match, repVal));
  updateLineNumbers();
  
  logConsole(`Successfully replaced all ${matchCount} occurrences of '${err.word}' with '${repVal}'.`);
  
  const oldText = btnSpellReplaceAll.innerText;
  btnSpellReplaceAll.innerText = `✅ Replaced ${matchCount}!`;
  btnSpellReplaceAll.disabled = true;
  setTimeout(() => {
    btnSpellReplaceAll.innerText = oldText;
    btnSpellReplaceAll.disabled = false;
  }, 1500);
  
  const lowerWord = err.word.toLowerCase();
  spellingErrors = spellingErrors.filter(e => e.word.toLowerCase() !== lowerWord);
  renderSpellingErrors();
  resetResolverPanel();
});

btnSpellIgnore.addEventListener("click", () => {
  if (selectedErrorIndex === -1) return;
  const err = spellingErrors[selectedErrorIndex];
  const lowerWord = err.word.toLowerCase();
  spellingErrors = spellingErrors.filter(e => e.word.toLowerCase() !== lowerWord);
  renderSpellingErrors();
  resetResolverPanel();
});

btnSpellAdd.addEventListener("click", async () => {
  if (selectedErrorIndex === -1) return;
  const err = spellingErrors[selectedErrorIndex];
  try {
    await invoke("add_to_dictionary", { path: currentFile, word: err.word });
    const lowerWord = err.word.toLowerCase();
    spellingErrors = spellingErrors.filter(e => e.word.toLowerCase() !== lowerWord);
    renderSpellingErrors();
    resetResolverPanel();
    logConsole("Added '" + err.word + "' to dictionary config.");
  } catch (e) {
    alert("Failed to add to dictionary: " + e);
  }
});

// ---------------------------------------------------------
// Browse Buttons (Bib, CSL)
// ---------------------------------------------------------
btnBrowseBib.addEventListener("click", async () => {
  const path = await invoke("select_file", { fileType: "bib" });
  if (path) {
    txtBib.value = path;
  }
});

btnBrowseCsl.addEventListener("click", async () => {
  const path = await invoke("select_file", { fileType: "csl" });
  if (path) {
    txtCsl.value = path;
  }
});

// ---------------------------------------------------------
// Workflows and Log Console
// ---------------------------------------------------------
function logConsole(str) {
  txtLog.innerText += "\n" + str;
  txtLog.scrollTop = txtLog.scrollHeight;
}

function clearConsole() {
  txtLog.innerText = "";
}

function cleanDimension(val, defaultVal) {
  val = val.trim().toLowerCase();
  if (!val) return defaultVal;
  
  const match = val.match(/^([\d.]+)\s*([a-z"”]+)?/);
  if (match) {
    const num = match[1];
    let unit = match[2] || "in";
    if (unit === "\"" || unit === "”" || unit === "in" || unit.startsWith("inch")) {
      unit = "in";
    } else if (unit !== "cm" && unit !== "mm" && unit !== "pt" && unit !== "em" && unit !== "ex") {
      unit = "in";
    }
    return num + unit;
  }
  return defaultVal;
}

// 1. Build PDF
btnBuildPdf.addEventListener("click", async () => {
  if (!currentFile) return;
  await saveDocument();
  clearConsole();
  logConsole("Starting PDF Build Compilation...");
  
  lblPdfStatus.innerText = "⏳ Compiling...";
  lblPdfStatus.style.color = "var(--text-secondary)";
  
  btnBuildPdf.disabled = true;
  btnBuildHtml.disabled = true;
  btnBuildTex.disabled = true;
  btnBuildDocx.disabled = true;
  
  try {
    let pageSizeVal = selectPdfPageSize.value;
    if (pageSizeVal === "custom") {
      const w = cleanDimension(txtPdfCustomW.value, "6in");
      const h = cleanDimension(txtPdfCustomH.value, "9in");
      pageSizeVal = `custom:${w},${h}`;
    }

    const result = await invoke("compile_pdf", {
      engine: selectEngine.value,
      path: currentFile,
      bibFile: txtBib.value,
      cslFile: txtCsl.value,
      useCiteproc: chkCiteproc.checked,
      toc: chkToc.checked,
      num: chkNum.checked,
      leftAlign: chkLeftAlign.checked,
      unicodeMath: chkUnicodeMath.checked,
      pageSize: pageSizeVal,
      pageMargin: selectPdfMargin.value
    });
    
    logConsole(result.logs);
    
    if (result.success) {
      lastPdf = currentFile.replace(/\.[^/.]+$/, "") + ".pdf";
      btnOpenPdf.disabled = false;
      lblPdfStatus.innerText = "✅ Success";
      lblPdfStatus.style.color = "var(--success-color)";
      clearCompileErrors();
    } else {
      lblPdfStatus.innerText = "❌ Failed: Compilation failed";
      lblPdfStatus.style.color = "var(--error-color)";
      
      const localErrors = findUnbalancedBraces();
      const formattedLocal = localErrors.map(e => ({
        line: e.line,
        message: e.message,
        context: e.type === "opening" ? "{" : "}",
        suggestion: e.type === "opening" 
          ? "This opening brace '{' is never closed. Click 'Apply Suggested Fix' to append a closing brace '}' to the end of this line, or close it manually."
          : "This closing brace '}' has no matching opening brace. Click 'Apply Suggested Fix' to remove it, or verify bracket nesting.",
        localType: e.type,
        col: e.col
      }));
      
      const combinedErrors = [...formattedLocal, ...result.errors];
      populateCompileErrors(combinedErrors);
      alert("LaTeX compilation failed. Check the 'Compiler Errors' tab for details.");
    }
  } catch (e) {
    logConsole("SYSTEM ERROR:\n" + e);
    lblPdfStatus.innerText = "❌ System Error: " + e;
    lblPdfStatus.style.color = "var(--error-color)";
    alert(e);
  } finally {
    btnBuildPdf.disabled = false;
    btnBuildHtml.disabled = false;
    btnBuildDocx.disabled = false;
    btnBuildTex.disabled = currentFile.toLowerCase().endsWith(".tex");
  }
});

btnOpenPdf.addEventListener("click", () => {
  if (lastPdf) {
    invoke("open_file", { path: lastPdf });
  }
});

// 2. Export HTML
btnBuildHtml.addEventListener("click", async () => {
  if (!currentFile) return;
  await saveDocument();
  clearConsole();
  logConsole("Starting HTML Export...");
  
  lblHtmlStatus.innerText = "⏳ Exporting...";
  lblHtmlStatus.style.color = "var(--text-secondary)";
  
  btnBuildPdf.disabled = true;
  btnBuildHtml.disabled = true;
  btnBuildTex.disabled = true;
  btnBuildDocx.disabled = true;
  
  try {
    const logs = await invoke("export_html", {
      path: currentFile,
      mathStyle: selectMathStyle.value,
      mathFg: selectMathFg.value,
      mathBg: selectMathBg.value,
      mathSize: selectMathSize.value,
      bibFile: txtBib.value,
      cslFile: txtCsl.value,
      useCiteproc: chkCiteproc.checked,
      autocopy: chkAutocopy.checked,
      toc: chkToc.checked,
      num: chkNum.checked,
      plainContent: txtEditor.value,
      convertInline: chkConvertInline.checked,
      tableWidth: selectHtmlTableWidth.value,
      tableStyle: selectHtmlTableStyle.value
    });
    
    logConsole(logs);
    
    const stem = currentFile.replace(/\.[^/.]+$/, "");
    lastHtml = stem + "_blog.html";
    btnOpenHtml.disabled = false;
    lblHtmlStatus.innerText = "✅ Success";
    lblHtmlStatus.style.color = "var(--success-color)";
    
    if (chkAutocopy.checked) {
      alert("HTML Export successful!\n\nContent has been copied to your clipboard. You can paste it directly into Substack or Medium.");
    } else {
      alert("HTML Export successful!");
    }
  } catch (e) {
    logConsole("ERROR:\n" + e);
    lblHtmlStatus.innerText = "❌ Export failed. See logs.";
    lblHtmlStatus.style.color = "var(--error-color)";
    alert("HTML Export failed. Check the compilation log for details.");
  } finally {
    btnBuildPdf.disabled = false;
    btnBuildHtml.disabled = false;
    btnBuildDocx.disabled = false;
    btnBuildTex.disabled = currentFile.toLowerCase().endsWith(".tex");
  }
});

btnOpenHtml.addEventListener("click", () => {
  if (lastHtml) {
    invoke("open_file", { path: lastHtml });
  }
});

// 3. Export to LaTeX
btnBuildTex.addEventListener("click", async () => {
  if (!currentFile) return;
  await saveDocument();
  clearConsole();
  logConsole("Starting LaTeX Source Code Export...");
  
  lblTexStatus.innerText = "⏳ Exporting...";
  lblTexStatus.style.color = "var(--text-secondary)";
  
  btnBuildPdf.disabled = true;
  btnBuildHtml.disabled = true;
  btnBuildTex.disabled = true;
  btnBuildDocx.disabled = true;
  
  try {
    const logs = await invoke("export_latex", {
      path: currentFile,
      bibFile: txtBib.value
    });
    
    logConsole(logs);
    
    lastTex = currentFile.replace(/\.[^/.]+$/, "") + ".export.tex";
    btnOpenTex.disabled = false;
    lblTexStatus.innerText = "✅ Success";
    lblTexStatus.style.color = "var(--success-color)";
    alert("LaTeX Export successful!");
  } catch (e) {
    logConsole("ERROR:\n" + e);
    lblTexStatus.innerText = "❌ Export failed. See logs.";
    lblTexStatus.style.color = "var(--error-color)";
    alert("LaTeX Export failed. Check the compilation log for details.");
  } finally {
    btnBuildPdf.disabled = false;
    btnBuildHtml.disabled = false;
    btnBuildDocx.disabled = false;
    btnBuildTex.disabled = currentFile.toLowerCase().endsWith(".tex");
  }
});

btnOpenTex.addEventListener("click", () => {
  if (lastTex) {
    invoke("open_file", { path: lastTex });
  }
});

// 4. Export to Word (DOCX)
btnBuildDocx.addEventListener("click", async () => {
  if (!currentFile) return;
  await saveDocument();
  clearConsole();
  logConsole("Starting Word Export (.docx)...");
  
  lblDocxStatus.innerText = "⏳ Exporting...";
  lblDocxStatus.style.color = "var(--text-secondary)";
  
  btnBuildPdf.disabled = true;
  btnBuildHtml.disabled = true;
  btnBuildTex.disabled = true;
  btnBuildDocx.disabled = true;
  
  try {
    try {
      const issues = await invoke("validate_manuscript", { path: currentFile });
      if (issues && issues.length > 0) {
        logConsole(`--- MANUSCRIPT VALIDATION REPORT (${issues.length} item(s)) ---`);
        for (const issue of issues) {
          logConsole(`[${issue.level}] Line ${issue.line}: ${issue.message}`);
          if (issue.snippet) logConsole(`    Snippet: ${issue.snippet}`);
        }
        logConsole("-------------------------------------\n");
      } else {
        logConsole("✅ Pre-export manuscript validation: Clean math structure.\n");
      }
    } catch (ve) {
      logConsole("Validation note: " + ve);
    }

    const logs = await invoke("export_docx", {
      path: currentFile,
      bibFile: txtBib.value,
      useCiteproc: chkCiteproc.checked
    });
    
    logConsole(logs);
    
    lastDocx = currentFile.replace(/\.[^/.]+$/, "") + ".export.docx";
    btnOpenDocx.disabled = false;
    lblDocxStatus.innerText = "✅ Success";
    lblDocxStatus.style.color = "var(--success-color)";
    alert("DOCX Export successful!");
  } catch (e) {
    logConsole("ERROR:\n" + e);
    lblDocxStatus.innerText = "❌ Export failed. See logs.";
    lblDocxStatus.style.color = "var(--error-color)";
    alert("DOCX Export failed. Check the compilation log for details.");
  } finally {
    btnBuildPdf.disabled = false;
    btnBuildHtml.disabled = false;
    btnBuildDocx.disabled = false;
    btnBuildTex.disabled = currentFile.toLowerCase().endsWith(".tex");
  }
});

btnOpenDocx.addEventListener("click", () => {
  if (lastDocx) {
    invoke("open_file", { path: lastDocx });
  }
});

// Bind all settings changes to reset Preset dropdown to "Custom"
function setPresetToCustom() {
  selectPreset.value = "Custom";
}

selectEngine.addEventListener("change", setPresetToCustom);
selectMathStyle.addEventListener("change", setPresetToCustom);
selectMathFg.addEventListener("change", setPresetToCustom);
selectMathBg.addEventListener("change", setPresetToCustom);
selectMathSize.addEventListener("change", setPresetToCustom);
chkCiteproc.addEventListener("change", setPresetToCustom);
chkToc.addEventListener("change", setPresetToCustom);
chkNum.addEventListener("change", setPresetToCustom);
chkLeftAlign.addEventListener("change", setPresetToCustom);
chkUnicodeMath.addEventListener("change", setPresetToCustom);
chkConvertInline.addEventListener("change", setPresetToCustom);
chkAutocopy.addEventListener("change", setPresetToCustom);
selectPdfPageSize.addEventListener("change", setPresetToCustom);
selectPdfMargin.addEventListener("change", setPresetToCustom);
selectHtmlTableWidth.addEventListener("change", setPresetToCustom);
selectHtmlTableStyle.addEventListener("change", setPresetToCustom);

// Real-time LaTeX preamble synchronizers
function syncTexGeometry() {
  if (!currentFile || !currentFile.toLowerCase().endsWith(".tex")) return;
  
  let text = txtEditor.value;
  const val = selectPdfPageSize.value;
  let pageSizeOption = "";
  if (val === "custom") {
    const w = cleanDimension(txtPdfCustomW.value, "6in");
    const h = cleanDimension(txtPdfCustomH.value, "9in");
    pageSizeOption = `paperwidth=${w},paperheight=${h}`;
  } else if (["6x9", "5.5x8.5", "5x8", "7x10", "8x10"].includes(val)) {
    const dims = val.split("x");
    pageSizeOption = `paperwidth=${dims[0]}in,paperheight=${dims[1]}in`;
  } else {
    pageSizeOption = val.toLowerCase() + "paper";
  }
  
  const margin = selectPdfMargin.value;
  const geoRegex = /\\usepackage\s*\[([^\]]*)\]\s*\{geometry\}/;
  
  if (geoRegex.test(text)) {
    text = text.replace(geoRegex, `\\usepackage[${pageSizeOption}, margin=${margin}]{geometry}`);
    txtEditor.value = text;
    logConsole(`Updated LaTeX geometry to: ${pageSizeOption}, margin=${margin}`);
    updateLineNumbers();
    if (previewPane.classList.contains("active")) {
      triggerPreviewRender(true);
    }
  } else {
    const docBeginRegex = /\\begin\s*\{\s*document\s*\}/;
    if (docBeginRegex.test(text)) {
      text = text.replace(docBeginRegex, `\\usepackage[${pageSizeOption}, margin=${margin}]{geometry}\n\n\\begin{document}`);
      txtEditor.value = text;
      logConsole(`Inserted LaTeX geometry package: ${pageSizeOption}, margin=${margin}`);
      updateLineNumbers();
      if (previewPane.classList.contains("active")) {
        triggerPreviewRender(true);
      }
    }
  }
}

function syncTexAlignment() {
  if (!currentFile || !currentFile.toLowerCase().endsWith(".tex")) return;
  
  let text = txtEditor.value;
  const isChecked = chkLeftAlign.checked;
  const raggedRegex = /\\usepackage\s*\[document\]\s*\{ragged2e\}\r?\n?/;
  
  if (isChecked) {
    if (!raggedRegex.test(text)) {
      const docBeginRegex = /\\begin\s*\{\s*document\s*\}/;
      if (docBeginRegex.test(text)) {
        text = text.replace(docBeginRegex, `\\usepackage[document]{ragged2e}\n\\begin{document}`);
        txtEditor.value = text;
        logConsole("Added ragged2e package for left-alignment in LaTeX preamble");
        updateLineNumbers();
        if (previewPane.classList.contains("active")) {
          triggerPreviewRender(true);
        }
      }
    }
  } else {
    if (raggedRegex.test(text)) {
      text = text.replace(raggedRegex, "");
      txtEditor.value = text;
      logConsole("Removed ragged2e package from LaTeX preamble (restored full justification)");
      updateLineNumbers();
      if (previewPane.classList.contains("active")) {
        triggerPreviewRender(true);
      }
    }
  }
}

selectPdfPageSize.addEventListener("change", () => {
  if (selectPdfPageSize.value === "custom") {
    divPdfCustomSize.style.display = "flex";
  } else {
    divPdfCustomSize.style.display = "none";
  }
  if (currentFile && currentFile.toLowerCase().endsWith(".tex")) {
    syncTexGeometry();
  }
});
selectPdfMargin.addEventListener("change", () => {
  if (currentFile && currentFile.toLowerCase().endsWith(".tex")) {
    syncTexGeometry();
  }
});
chkLeftAlign.addEventListener("change", () => {
  if (currentFile && currentFile.toLowerCase().endsWith(".tex")) {
    syncTexAlignment();
  }
});

// Update preamble dynamically when custom page dimensions are typed
txtPdfCustomW.addEventListener("input", () => {
  if (currentFile && currentFile.toLowerCase().endsWith(".tex") && selectPdfPageSize.value === "custom") {
    syncTexGeometry();
  }
});
txtPdfCustomH.addEventListener("input", () => {
  if (currentFile && currentFile.toLowerCase().endsWith(".tex") && selectPdfPageSize.value === "custom") {
    syncTexGeometry();
  }
});

// Dependency Checker implementation
async function checkSystemDependencies() {
  if (!btnCheckDeps) return;
  btnCheckDeps.disabled = true;
  btnCheckDeps.innerText = "Checking...";
  
  try {
    const status = await invoke("check_dependencies");
    
    updateDepLabel(depPandoc, status.has_pandoc);
    updateDepLabel(depCspell, status.has_cspell);
    updateDepLabel(depNode, status.has_node);
    updateDepLabel(depLualatex, status.has_lualatex);
    updateDepLabel(depXelatex, status.has_xelatex);
    updateDepLabel(depPdflatex, status.has_pdflatex);
    
    // Log warnings if dependencies are missing
    let missing = [];
    if (!status.has_pandoc) missing.push("Pandoc (needed for Markdown PDF compilation)");
    if (!status.has_cspell) missing.push("CSpell (needed for Spell Checker)");
    if (!status.has_node) missing.push("Node.js/npm (needed for Spell Checker database)");
    if (!status.has_lualatex && !status.has_xelatex && !status.has_pdflatex) {
      missing.push("LaTeX (LuaLaTeX, XeLaTeX, or PDFLaTeX needed for PDF compilation)");
    }
    
    if (missing.length > 0) {
      logConsole("WARNING: Missing system dependencies:\n- " + missing.join("\n- "));
    } else {
      logConsole("All system dependencies are successfully installed.");
    }
  } catch (e) {
    logConsole("Error checking system dependencies: " + e);
  } finally {
    btnCheckDeps.disabled = false;
    btnCheckDeps.innerText = "Run Check";
  }
}

function updateDepLabel(element, isInstalled) {
  if (!element) return;
  if (isInstalled) {
    element.innerText = "Installed";
    element.style.color = "var(--success-color)";
  } else {
    element.innerText = "Missing";
    element.style.color = "var(--error-color)";
  }
}

if (btnCheckDeps) {
  btnCheckDeps.addEventListener("click", checkSystemDependencies);
}

const divForceIse = document.getElementById("div-force-ise");

function updateForceIseVisibility(lang) {
  if (divForceIse) {
    if (lang === "en-GB") {
      divForceIse.style.display = "flex";
      chkForceIse.disabled = false;
      chkForceIse.checked = (localStorage.getItem("forceIse") !== "false");
    } else {
      divForceIse.style.display = "none";
      chkForceIse.disabled = true;
      chkForceIse.checked = false;
    }
  }
}

if (selectSpellLang) {
  selectSpellLang.addEventListener("change", () => {
    const lang = selectSpellLang.value;
    localStorage.setItem("spellingLanguage", lang);
    updateForceIseVisibility(lang);
    if (txtEditor.value.trim().length > 0) {
      scanSpelling();
    }
  });
}

if (chkForceIse) {
  chkForceIse.addEventListener("change", () => {
    localStorage.setItem("forceIse", chkForceIse.checked ? "true" : "false");
    if (txtEditor.value.trim().length > 0) {
      scanSpelling();
    }
  });
}

function loadSpellcheckSettings() {
  const savedLang = localStorage.getItem("spellingLanguage") || "en-GB";
  
  if (selectSpellLang) {
    selectSpellLang.value = savedLang;
    updateForceIseVisibility(savedLang);
  }
}

// Run initialization
updateRecentFilesDropdown();
loadSpellcheckSettings();
checkSystemDependencies();

// ---------------------------------------------------------
// Theme Switcher & System Theme Support
// ---------------------------------------------------------
const selectTheme = document.getElementById("select-theme");

function applyTheme(themeName) {
  if (themeName === "system") {
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
  } else {
    document.documentElement.setAttribute("data-theme", themeName);
  }
  localStorage.setItem("appTheme", themeName);
  if (selectTheme) selectTheme.value = themeName;
  if (previewPane && previewPane.classList.contains("active")) {
    triggerPreviewRender(true);
  }
}

function initTheme() {
  const savedTheme = localStorage.getItem("appTheme") || "dark";
  applyTheme(savedTheme);
  
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (localStorage.getItem("appTheme") === "system") {
      applyTheme("system");
    }
  });
}

if (selectTheme) {
  selectTheme.addEventListener("change", () => {
    applyTheme(selectTheme.value);
  });
}
initTheme();

// ---------------------------------------------------------
// Drag & Drop File Loading Overlay & Handlers
// ---------------------------------------------------------
const dropOverlay = document.getElementById("drop-overlay");

async function handleDroppedFiles(files) {
  if (!files || files.length === 0) return;
  const file = files[0];
  
  // 1. If absolute path is available, load document via Tauri backend
  if (file.path && (file.path.startsWith("/") || file.path.includes(":\\") || file.path.includes(":/"))) {
    loadDocument(file.path);
    return;
  }

  // 2. Otherwise read file content directly via HTML5 FileReader / file.text()
  try {
    const content = await file.text();
    txtEditor.value = content;
    lblFilePath.innerText = file.name ? `${file.name} (Dropped File)` : "Dropped Document";
    currentFile = "";
    updateLineNumbers();
    
    btnSaveDoc.disabled = false;
    btnSaveAsDoc.disabled = false;
    btnScanSpell.disabled = false;
    btnTogglePreview.disabled = false;
    
    if (previewPane && previewPane.classList.contains("active")) {
      triggerPreviewRender(true);
    }
    updateUIForFileType();
    logConsole(`Loaded dropped file: ${file.name || "document"}`);
  } catch (err) {
    console.error("Failed to read dropped file content: ", err);
  }
}

// 1. Tauri Native Drag & Drop Event Listeners
if (window.__TAURI__ && window.__TAURI__.event && window.__TAURI__.event.listen) {
  window.__TAURI__.event.listen("tauri://drag-drop", async (event) => {
    if (dropOverlay) dropOverlay.classList.remove("active");
    const payload = event.payload;
    const paths = payload && (payload.paths || (Array.isArray(payload) ? payload : []));
    if (paths && paths.length > 0) {
      const firstPath = paths[0];
      if (firstPath) {
        loadDocument(firstPath);
      }
    }
  }).catch(() => {});

  window.__TAURI__.event.listen("tauri://drag-enter", () => {
    if (dropOverlay) dropOverlay.classList.add("active");
  }).catch(() => {});

  window.__TAURI__.event.listen("tauri://drag-leave", () => {
    if (dropOverlay) dropOverlay.classList.remove("active");
  }).catch(() => {});
}

// 2. HTML5 Webview Drag & Drop Event Listeners
["dragenter", "dragover"].forEach((eventName) => {
  window.addEventListener(eventName, (e) => {
    e.preventDefault();
    e.stopPropagation();
    if (dropOverlay) dropOverlay.classList.add("active");
  }, false);
});

window.addEventListener("dragleave", (e) => {
  e.preventDefault();
  e.stopPropagation();
  if (e.clientX <= 0 || e.clientY <= 0 || e.clientX >= window.innerWidth || e.clientY >= window.innerHeight) {
    if (dropOverlay) dropOverlay.classList.remove("active");
  }
}, false);

window.addEventListener("drop", async (e) => {
  e.preventDefault();
  e.stopPropagation();
  if (dropOverlay) dropOverlay.classList.remove("active");
  
  if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files.length > 0) {
    handleDroppedFiles(e.dataTransfer.files);
  }
}, false);

// ---------------------------------------------------------
// Editor Context Menu & Global Keybindings
// ---------------------------------------------------------
const editorContextMenu = document.getElementById("editor-context-menu");
const ctxCut = document.getElementById("ctx-cut");
const ctxCopy = document.getElementById("ctx-copy");
const ctxPaste = document.getElementById("ctx-paste");
const ctxSelectAll = document.getElementById("ctx-select-all");
const ctxSpellcheck = document.getElementById("ctx-spellcheck");

if (txtEditor) {
  txtEditor.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    if (!editorContextMenu) return;
    editorContextMenu.style.left = `${Math.min(e.clientX, window.innerWidth - 200)}px`;
    editorContextMenu.style.top = `${Math.min(e.clientY, window.innerHeight - 200)}px`;
    editorContextMenu.style.display = "block";
  });
}

window.addEventListener("click", () => {
  if (editorContextMenu) editorContextMenu.style.display = "none";
});

if (ctxCut) {
  ctxCut.addEventListener("click", () => {
    document.execCommand("cut");
    if (editorContextMenu) editorContextMenu.style.display = "none";
  });
}

if (ctxCopy) {
  ctxCopy.addEventListener("click", () => {
    document.execCommand("copy");
    if (editorContextMenu) editorContextMenu.style.display = "none";
  });
}

if (ctxPaste) {
  ctxPaste.addEventListener("click", async () => {
    try {
      const text = await navigator.clipboard.readText();
      if (text && txtEditor) {
        const start = txtEditor.selectionStart;
        const end = txtEditor.selectionEnd;
        txtEditor.value = txtEditor.value.substring(0, start) + text + txtEditor.value.substring(end);
        txtEditor.selectionStart = txtEditor.selectionEnd = start + text.length;
        updateLineNumbers();
      }
    } catch (err) {
      document.execCommand("paste");
    }
    if (editorContextMenu) editorContextMenu.style.display = "none";
  });
}

if (ctxSelectAll) {
  ctxSelectAll.addEventListener("click", () => {
    if (txtEditor) {
      txtEditor.select();
    }
    if (editorContextMenu) editorContextMenu.style.display = "none";
  });
}

if (ctxSpellcheck) {
  ctxSpellcheck.addEventListener("click", () => {
    if (editorContextMenu) editorContextMenu.style.display = "none";
    if (typeof switchTab === "function") switchTab("spell");
    scanSpelling();
  });
}

// Global Keyboard Shortcuts
window.addEventListener("keydown", (e) => {
  const isCmdOrCtrl = e.metaKey || e.ctrlKey;
  if (!isCmdOrCtrl) return;

  const key = e.key.toLowerCase();
  if (key === "s") {
    e.preventDefault();
    if (e.shiftKey && typeof saveAsDocument === "function") {
      saveAsDocument();
    } else {
      saveDocument();
    }
  } else if (key === "o") {
    e.preventDefault();
    openDocument();
  } else if (key === "n") {
    e.preventDefault();
    newDocument();
  } else if (key === "f") {
    e.preventDefault();
    if (txtFind) {
      txtFind.focus();
      txtFind.select();
    }
  }
});

// ---------------------------------------------------------
// Split-Pane Drag Resizers
// ---------------------------------------------------------
function initResizers() {
  const resizerEP = document.getElementById("resizer-editor-preview");
  const resizerPS = document.getElementById("resizer-preview-sidebar");
  const editorPane = document.querySelector(".editor-pane");
  const previewPane = document.getElementById("preview-pane");
  const sidebarPane = document.querySelector(".sidebar-pane");

  // 1. Resizer between Document Editor and Document Preview
  if (resizerEP && editorPane && previewPane) {
    let isDraggingEP = false;
    let startX = 0;
    let startEditorWidth = 0;

    resizerEP.addEventListener("mousedown", (e) => {
      if (!previewPane.classList.contains("active")) return;
      isDraggingEP = true;
      startX = e.clientX;
      startEditorWidth = editorPane.getBoundingClientRect().width;
      resizerEP.classList.add("resizing");
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
      e.preventDefault();
    });

    window.addEventListener("mousemove", (e) => {
      if (!isDraggingEP) return;
      const deltaX = e.clientX - startX;
      const newWidth = Math.max(180, startEditorWidth + deltaX);
      editorPane.style.flex = `0 0 ${newWidth}px`;
      editorPane.style.width = `${newWidth}px`;
    });

    window.addEventListener("mouseup", () => {
      if (isDraggingEP) {
        isDraggingEP = false;
        resizerEP.classList.remove("resizing");
        document.body.style.cursor = "";
        document.body.style.userSelect = "";
      }
    });
  }

  // 2. Resizer between Preview / Editor and Sidebar
  if (resizerPS && sidebarPane) {
    let isDraggingPS = false;
    let startX = 0;
    let startSidebarWidth = 0;

    resizerPS.addEventListener("mousedown", (e) => {
      isDraggingPS = true;
      startX = e.clientX;
      startSidebarWidth = sidebarPane.getBoundingClientRect().width;
      resizerPS.classList.add("resizing");
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
      e.preventDefault();
    });

    window.addEventListener("mousemove", (e) => {
      if (!isDraggingPS) return;
      const deltaX = startX - e.clientX;
      const newWidth = Math.max(280, Math.min(750, startSidebarWidth + deltaX));
      sidebarPane.style.width = `${newWidth}px`;
      sidebarPane.style.flex = `0 0 ${newWidth}px`;
    });

    window.addEventListener("mouseup", () => {
      if (isDraggingPS) {
        isDraggingPS = false;
        resizerPS.classList.remove("resizing");
        document.body.style.cursor = "";
        document.body.style.userSelect = "";
      }
    });
  }
}

initResizers();

// ---------------------------------------------------------
// Help & User Guide Dialog
// ---------------------------------------------------------
const btnHelp = document.getElementById("btn-help");
const helpDialog = document.getElementById("help-dialog");
const txtHelpSearch = document.getElementById("txt-help-search");
const helpNavBtns = document.querySelectorAll(".help-nav-btn");
const helpSections = document.querySelectorAll(".help-section");

if (btnHelp && helpDialog) {
  btnHelp.addEventListener("click", () => {
    helpDialog.showModal();
  });
}

if (helpNavBtns && helpSections) {
  helpNavBtns.forEach((btn) => {
    btn.addEventListener("click", () => {
      const targetId = btn.getAttribute("data-target");
      helpNavBtns.forEach((b) => b.classList.remove("active"));
      helpSections.forEach((s) => s.classList.remove("active"));
      btn.classList.add("active");
      const targetSection = document.getElementById(targetId);
      if (targetSection) {
        targetSection.classList.add("active");
      }
    });
  });
}

if (txtHelpSearch) {
  txtHelpSearch.addEventListener("input", () => {
    const query = txtHelpSearch.value.trim().toLowerCase();
    if (!query) {
      helpNavBtns.forEach((b) => (b.style.display = "block"));
      return;
    }
    helpNavBtns.forEach((b) => {
      const targetId = b.getAttribute("data-target");
      const targetSec = document.getElementById(targetId);
      const text = (targetSec ? targetSec.innerText : "") + " " + b.innerText;
      if (text.toLowerCase().includes(query)) {
        b.style.display = "block";
      } else {
        b.style.display = "none";
      }
    });
  });
}

