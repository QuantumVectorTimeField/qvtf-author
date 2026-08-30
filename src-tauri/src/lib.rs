use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct SpellError {
    line: usize,
    col: usize,
    word: String,
    context: String,
    suggested: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LaTeXError {
    line: usize,
    message: String,
    context: String,
    suggestion: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CompileResult {
    success: bool,
    logs: String,
    errors: Vec<LaTeXError>,
}

fn generate_suggestion(message: &str, context: &str) -> String {
    let msg_lower = message.to_lowercase();
    let combined = format!("{} {}", message, context);
    if msg_lower.contains("undefined control sequence") {
        let command = Regex::new(r"\\[A-Za-z@]+")
            .ok()
            .and_then(|re| re.find(&combined))
            .map(|m| m.as_str().to_string());
        if let Some(command) = command {
            format!("The command '{}' is undefined. Check its spelling and capitalisation, or load the package that defines it. The highlighted source line contains the command LaTeX could not expand.", command)
        } else {
            "A LaTeX command is undefined. Check the highlighted line for a misspelled command or a command whose package has not been loaded.".to_string()
        }
    } else if msg_lower.contains("file") && msg_lower.contains("not found") {
        "A required file, image, bibliography, class or package could not be found. Check the filename and path (including letter case), then confirm the dependency is installed.".to_string()
    } else if msg_lower.contains("runaway argument") || msg_lower.contains("paragraph ended before") {
        "An argument was left open. Starting at the highlighted line, check for a missing closing brace '}', bracket ']' or environment terminator. The real opening delimiter is often a few lines earlier.".to_string()
    } else if msg_lower.contains("something's wrong") || msg_lower.contains("missing \\item") || msg_lower.contains("lonely \\item") {
        "A list structure is malformed. Ensure every item in itemize/enumerate begins with \\item, remove any \\item outside a list, and check that the list's \\begin and \\end match.".to_string()
    } else if msg_lower.contains("environment") && msg_lower.contains("undefined") {
        let environment = Regex::new(r"Environment\s+([^\s]+)")
            .ok().and_then(|re| re.captures(message))
            .and_then(|caps| caps.get(1)).map(|m| m.as_str()).unwrap_or("the reported environment");
        format!("{} is not defined. Check the environment name and load the package that supplies it, then confirm its \\begin and \\end names match exactly.", environment)
    } else if msg_lower.contains("already defined") || msg_lower.contains("command \\end") && msg_lower.contains("already") {
        if [r"\eth", r"\smallsetminus", r"\digamma", r"\backepsilon"].iter().any(|cmd| combined.contains(cmd)) {
            "The legacy amssymb symbol set is being loaded after unicode-math, so both packages try to define the same symbols. Remove the later/duplicate amssymb load, or load amsmath/amssymb before unicode-math. Do not edit the highlighted prose line.".to_string()
        } else {
            "A command or environment is being defined twice in the LaTeX preamble. This is usually a package conflict, not an error in the highlighted prose. Remove the duplicate package/definition; if Unicode Math is enabled, avoid also loading unicode-math or a conflicting legacy symbol package manually.".to_string()
        }
    } else if msg_lower.contains("option clash for package") {
        "The same package is loaded more than once with incompatible options. Combine its options in a single \\usepackage[...] declaration and remove the duplicate load.".to_string()
    } else if msg_lower.contains("missing number") || msg_lower.contains("number expected") {
        "LaTeX expected a number here. Check lengths, counters and optional arguments for a missing numeric value; for dimensions include a valid unit such as pt, mm, cm or in.".to_string()
    } else if msg_lower.contains("illegal unit of measure") {
        "A length has a missing or invalid unit. Use a supported unit such as pt, mm, cm, em, ex or in—for example, '12pt' rather than just '12'.".to_string()
    } else if msg_lower.contains("double subscript") || msg_lower.contains("double superscript") {
        "The same math atom has two consecutive subscripts or superscripts. Group the intended expression with braces, for example x_{a_b}, or remove the extra '_'/'^'.".to_string()
    } else if msg_lower.contains("cannot determine size of graphic") || msg_lower.contains("no boundingbox") {
        "LaTeX cannot determine the image dimensions. Check the image path and format; use PDF/PNG/JPEG with pdfLaTeX or a compatible graphics driver, and verify the file is not corrupt.".to_string()
    } else if msg_lower.contains("use of") && msg_lower.contains("doesn't match its definition") {
        "A macro was called with arguments that do not match its definition. Check the command's required braces, optional brackets and delimiter order on this line.".to_string()
    } else if msg_lower.contains("missing \\endcsname inserted") {
        "A command name or reference key contains invalid syntax, often from an unexpected backslash, brace or expansion inside a label/citation. Simplify the key and check delimiter balance.".to_string()
    } else if msg_lower.contains("extra \\right") {
        "Found a \\right that does not have a matching \\left. Check for an extra period (\\right.) or check bracket balance.".to_string()
    } else if msg_lower.contains("ended by \\end") {
        "A math or environment block ended with the wrong environment name. Compare the nearest \\begin{...} and \\end{...} pair.".to_string()
    } else if msg_lower.contains("missing } inserted") || msg_lower.contains("too many }") {
        "A curly bracket is unbalanced. Check the nesting of your brackets { } (LaTeX often reports this at the end of a paragraph or blank line; check preceding lines if this line is blank).".to_string()
    } else if msg_lower.contains("missing $ inserted") || msg_lower.contains("math shift") {
        "Math mode is unbalanced near this line. Check for a missing or extra '$', '$$', '\\(' or '\\[' delimiter, and ensure math-only commands are inside a math environment.".to_string()
    } else if msg_lower.contains("allowed only in math mode") || msg_lower.contains("only allowed in math mode") {
        let command = Regex::new(r"\\[A-Za-z@]+")
            .ok().and_then(|re| re.find(&combined))
            .map(|m| m.as_str()).unwrap_or("This command");
        format!("{} is a math-only command but is being used as ordinary text. Put the complete mathematical expression in math mode—for example, change '{}{{A}}' to '${}{{A}}$'—or use a text-mode alternative.", command, command, command)
    } else if msg_lower.contains("display math") || msg_lower.contains("bad math environment") || msg_lower.contains("eqno") {
        "Check for nested or mismatched display-math delimiters. Do not wrap an equation/align environment inside '$$...$$' or '\\[...\\]'.".to_string()
    } else if msg_lower.contains("missing \\begin{document}") || msg_lower.contains("missing begin{document}") {
        "LaTeX detected plain text or an illegal document command inside the preamble (before \\begin{document}). You cannot write regular document text in the preamble. Check if you recently uncommented or added plain text above \\begin{document}.".to_string()
    } else if msg_lower.contains("misplaced alignment tab") || msg_lower.contains("misplaced \\crcr") || msg_lower.contains("extra alignment tab") {
        "An '&' alignment marker or '\\\\' row break is outside a valid table/alignment position, or the row has too many columns. Check the complete row containing the highlighted line.".to_string()
    } else if msg_lower.contains("unicode character") && msg_lower.contains("not set up") {
        "This engine cannot typeset the reported Unicode character. Use LuaLaTeX/XeLaTeX, replace the character with a LaTeX command, or load an appropriate input/font package.".to_string()
    } else if msg_lower.contains("there's no line here to end") {
        "A '\\\\' line break appears where LaTeX has no active text line. Remove it or place it inside the intended paragraph, table or alignment environment.".to_string()
    } else if msg_lower.contains("capacity exceeded") {
        "TeX exhausted an internal limit, often because of recursive macro expansion or a cyclic include. Check recently defined commands and included files near the highlighted location.".to_string()
    } else if msg_lower.contains("float(s) lost") {
        "A floating environment (like a figure or table) was not closed properly. Check that all \\begin{figure} blocks have a matching \\end{figure}.".to_string()
    } else if msg_lower.contains("emergency stop") {
        "Compilation stopped after an earlier fatal error. Fix the first error in the list first; this stop message is usually a consequence rather than the root cause.".to_string()
    } else if msg_lower.contains("latex error") || msg_lower.contains("package error") {
        format!("The compiler reported: '{}'. Check the highlighted source against that message; if it names a command, environment or package, correct or load that specific item first.", message.trim())
    } else {
        format!("The compiler reported: '{}'. Start at the highlighted line and read one statement backwards; check the exact command or delimiter named in the message before addressing later errors.", message.trim())
    }
}

fn context_from_source(source: &str, line: usize) -> String {
    if line == 0 { return String::new(); }
    source.lines().nth(line - 1).unwrap_or("").trim().to_string()
}

fn infer_source_line(source: &str, reported_line: usize, context: &str) -> usize {
    let source_lines: Vec<&str> = source.lines().collect();
    if source_lines.is_empty() { return reported_line.max(1); }

    let command_re = Regex::new(r"\\[A-Za-z@]+").unwrap();
    if let Some(command) = command_re.find(context) {
        let hits: Vec<usize> = source_lines.iter().enumerate()
            .filter(|(_, line)| line.contains(command.as_str()))
            .map(|(idx, _)| idx + 1)
            .collect();
        if hits.len() == 1 { return hits[0]; }
        if !hits.is_empty() {
            return *hits.iter().min_by_key(|line| line.abs_diff(reported_line)).unwrap();
        }
    }

    let needle = context.trim().trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c.is_whitespace());
    if needle.len() >= 8 {
        let shortened: String = needle.chars().take(48).collect();
        if let Some((idx, _)) = source_lines.iter().enumerate().find(|(_, line)| line.contains(&shortened)) {
            return idx + 1;
        }
    }

    // A generated TeX line number is not a Markdown source line. Returning
    // zero lets the caller label it as a generated-preamble error rather than
    // highlighting an unrelated manuscript line by coincidence.
    0
}

fn parse_compiler_errors(output: &str, source: &str, remap_generated_lines: bool) -> Vec<LaTeXError> {
    let mut errors = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    let file_line_re = Regex::new(r"^(.*?):(\d+):\s*(?:LaTeX Error:\s*)?(.*)$").unwrap();
    let pandoc_line_re = Regex::new(r"(?i)(?:at |on |near )?line\s+(\d+)(?:\s+column\s+\d+)?[:,:]?\s*(.*)").unwrap();
    let tex_line_re = Regex::new(r"^l\.(\d+)\s*(.*)$").unwrap();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim_end();
        let mut message = String::new();
        let mut line_num = 0usize;
        let mut context = String::new();

        if let Some(caps) = file_line_re.captures(line) {
            let diagnostic_file = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let is_dependency_file = diagnostic_file.ends_with(".sty") || diagnostic_file.ends_with(".cls");
            line_num = caps[2].parse().unwrap_or(0);
            message = caps.get(3).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            // TeX wraps long physical log lines at max_print_line, which can
            // split even the words "LaTeX Error" into "LaTeX Er" + "ror:".
            // Reassemble continuation fragments before classifying the error.
            for next in lines.iter().skip(i + 1).take(8) {
                let continuation = next.trim();
                if continuation.is_empty()
                    || continuation.starts_with("l.")
                    || continuation.starts_with("! ")
                    || file_line_re.is_match(continuation)
                    || continuation.starts_with("Type H")
                    || continuation.starts_with("See the LaTeX manual")
                    || continuation.starts_with('<')
                {
                    break;
                }
                let looks_wrapped = continuation.starts_with("ror:")
                    || message.ends_with("LaTeX Er")
                    || message.ends_with("Package Er")
                    || (!message.ends_with('.') && !message.ends_with('!') && !message.ends_with('?'));
                if !looks_wrapped { break; }
                if continuation.starts_with("ror:") && message.ends_with("Er") {
                    message.push_str(continuation);
                } else {
                    message.push(' ');
                    message.push_str(continuation);
                }
            }
            if message.is_empty() || message.eq_ignore_ascii_case("latex error:") || message.eq_ignore_ascii_case("latex error") {
                message = lines.iter().skip(i + 1).take(8)
                    .map(|next| next.trim())
                    .find(|next| !next.is_empty()
                        && !next.starts_with("l.")
                        && !next.starts_with("!")
                        && !next.starts_with("Type H")
                        && !next.starts_with("See the LaTeX manual"))
                    .unwrap_or("LaTeX reported an error without additional detail.")
                    .to_string();
            }
            context = if is_dependency_file {
                format!("LaTeX dependency file: {} (not a manuscript source line).", diagnostic_file)
            } else if remap_generated_lines {
                String::new()
            } else {
                context_from_source(source, line_num)
            };
            if remap_generated_lines && !is_dependency_file {
                for next in lines.iter().skip(i + 1).take(12) {
                    if let Some(line_caps) = tex_line_re.captures(next.trim()) {
                        context = line_caps.get(2)
                            .map(|m| m.as_str().trim().to_string())
                            .unwrap_or_default();
                        break;
                    }
                }
            }
        } else if line.starts_with("! ") {
            message = line[2..].trim().to_string();
            let mut j = i + 1;
            while j < lines.len() && j < i + 30 {
                let next_line = lines[j].trim();
                if next_line.starts_with("! ") { break; }
                if let Some(caps) = tex_line_re.captures(next_line) {
                    line_num = caps[1].parse().unwrap_or(0);
                    context = caps.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
                    if context.is_empty() && j + 1 < lines.len() {
                        context = lines[j + 1].trim().to_string();
                    }
                    break;
                }
                j += 1;
            }
            i = j;
        } else if let Some(caps) = pandoc_line_re.captures(line) {
            line_num = caps[1].parse().unwrap_or(0);
            message = caps.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            if message.is_empty() { message = line.trim().to_string(); }
            context = context_from_source(source, line_num);
        }

        if !message.is_empty() && line_num > 0 {
            if context.starts_with("LaTeX dependency file:") {
                line_num = 1;
            } else if remap_generated_lines {
                let inferred_line = infer_source_line(source, line_num, &context);
                if inferred_line == 0 {
                    line_num = 1;
                    context = "Generated LaTeX preamble (not a manuscript source line).".to_string();
                } else {
                    line_num = inferred_line;
                    if context.is_empty() { context = context_from_source(source, line_num); }
                }
            }
            let suggestion = generate_suggestion(&message, &context);
            if !errors.iter().any(|e: &LaTeXError| e.line == line_num && e.message == message) {
                errors.push(LaTeXError { line: line_num, message, context, suggestion });
            }
        } else {
            // Fatal messages without their own location are useful only when no
            // more precise error has been found.
            if line.starts_with("! Emergency stop") && errors.is_empty() {
                errors.push(LaTeXError {
                    line: 1,
                    message: "Emergency stop".to_string(),
                    context: context_from_source(source, 1),
                    suggestion: generate_suggestion("Emergency stop", ""),
                });
            }
        }
        i += 1;
    }
    errors.sort_by_key(|e| e.line);

    let conflict_commands = [r"\eth", r"\smallsetminus", r"\digamma", r"\backepsilon"];
    let found_conflicts: Vec<&str> = conflict_commands.iter().copied()
        .filter(|command| errors.iter().any(|error| {
            error.message.to_lowercase().contains("already defined") && error.message.contains(command)
        }))
        .collect();
    if found_conflicts.len() >= 2 {
        errors.retain(|error| !(
            error.message.to_lowercase().contains("already defined")
                && conflict_commands.iter().any(|command| error.message.contains(command))
        ));
        let command_list = found_conflicts.join(", ");
        let message = format!("Symbol package conflict: {} are already defined.", command_list);
        errors.push(LaTeXError {
            line: 1,
            context: "LaTeX package/preamble conflict (not a manuscript source line).".to_string(),
            suggestion: generate_suggestion(&message, &command_list),
            message,
        });
        errors.sort_by_key(|e| e.line);
    }
    errors
}

fn get_env_path() -> String {
    let current_path = std::env::var("PATH").unwrap_or_default();
    // Always place /Library/TeX/texbin (MacTeX) at the very front of PATH so native MacTeX binaries
    // (latexpand, pdflatex, etc.) are prioritized over stale/broken MiKTeX symlinks in /usr/local/bin.
    let prioritized_paths = vec!["/Library/TeX/texbin", "/opt/homebrew/bin", "/usr/local/bin"];
    let existing_parts: Vec<&str> = current_path
        .split(':')
        .filter(|p| !p.is_empty() && !prioritized_paths.contains(p))
        .collect();
    let mut all_paths = prioritized_paths;
    all_paths.extend(existing_parts);
    all_paths.join(":")
}

fn latex_to_unicode_html(latex: &str) -> Option<String> {
    let mut latex_clean = latex.replace("\\displaystyle", "").replace("\\textstyle", "").trim().to_string();
    
    // Check if it has complex keywords that are better left as images
    let complex_keywords = vec!["\\frac", "\\int", "\\sum", "\\sqrt", "\\begin", "\\end", "\\left", "\\right", "\\over"];
    for kw in complex_keywords {
        if latex_clean.contains(kw) {
            return None;
        }
    }
    
    let greek_map = vec![
        (r"\alpha", "α"), (r"\beta", "β"), (r"\gamma", "γ"), (r"\delta", "δ"),
        (r"\epsilon", "ε"), (r"\zeta", "ζ"), (r"\eta", "η"), (r"\theta", "θ"),
        (r"\iota", "ι"), (r"\kappa", "κ"), (r"\lambda", "λ"), (r"\mu", "μ"),
        (r"\nu", "ν"), (r"\xi", "ξ"), (r"\omicron", "o"), (r"\pi", "π"),
        (r"\rho", "ρ"), (r"\sigma", "σ"), (r"\tau", "τ"), (r"\upsilon", "υ"),
        (r"\phi", "φ"), (r"\chi", "χ"), (r"\psi", "ψ"), (r"\omega", "ω"),
        (r"\Gamma", "Γ"), (r"\Delta", "Δ"), (r"\Theta", "Θ"), (r"\Lambda", "Λ"),
        (r"\Xi", "Ξ"), (r"\Pi", "Π"), (r"\Sigma", "Σ"), (r"\Upsilon", "Υ"),
        (r"\Phi", "Φ"), (r"\Psi", "Ψ"), (r"\Omega", "Ω"),
        (r"\boldsymbol{\omega}", "𝝎"),
        (r"\boldsymbol{\tau}", "𝝳"),
        (r"\mathbf{D}", "𝐃"),
        (r"\mathbf{F}", "𝐅"),
        (r"\mathbf{v}", "𝐯"),
        (r"\mathrm{rel}", "rel"),
        (r"\nabla", "∇"),
        (r"\times", "×"),
        (r"\propto", "∝"),
    ];
    
    for (k, v) in greek_map {
        latex_clean = latex_clean.replace(k, v);
    }
    
    // Convert subscripts like _{TF} -> <sub>TF</sub>
    let sub_brace = Regex::new(r"\_\{([^}]+)\}").unwrap();
    latex_clean = sub_brace.replace_all(&latex_clean, "<sub>$1</sub>").to_string();
    let sub_single = Regex::new(r"\_([a-zA-Z0-9])").unwrap();
    latex_clean = sub_single.replace_all(&latex_clean, "<sub>$1</sub>").to_string();
    
    // Convert superscripts like ^{2} -> <sup>2</sup>
    let sup_brace = Regex::new(r"\^\{([^}]+)\}").unwrap();
    latex_clean = sup_brace.replace_all(&latex_clean, "<sup>$1</sup>").to_string();
    let sup_single = Regex::new(r"\^([a-zA-Z0-9])").unwrap();
    latex_clean = sup_single.replace_all(&latex_clean, "<sup>$1</sup>").to_string();
    
    // Remove remaining LaTeX formatting commands
    let bsymb = Regex::new(r"\\boldsymbol\{([^}]+)\}").unwrap();
    latex_clean = bsymb.replace_all(&latex_clean, "<strong>$1</strong>").to_string();
    let bmath = Regex::new(r"\\mathbf\{([^}]+)\}").unwrap();
    latex_clean = bmath.replace_all(&latex_clean, "<strong>$1</strong>").to_string();
    let rm_cmd = Regex::new(r"\\mathrm\{([^}]+)\}").unwrap();
    latex_clean = rm_cmd.replace_all(&latex_clean, "$1").to_string();
    let txt_cmd = Regex::new(r"\\text\{([^}]+)\}").unwrap();
    latex_clean = txt_cmd.replace_all(&latex_clean, "$1").to_string();
    
    // Remove any remaining backslashes
    latex_clean = latex_clean.replace("\\", "");
    
    Some(format!("<em>{}</em>", latex_clean))
}

fn post_process_math_styles(file_path: &Path) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    // Matches any img tag that loads from latex.codecogs.com
    let img_pattern = Regex::new(r#"(?i)<img\s+[^>]*?src=["'][^"']*?latex\.codecogs\.com[^"']*?["'][^>]*?/?>"#).unwrap();
    let style_strip = Regex::new(r#"(?i)style\s*=\s*"[^"]*""#).unwrap();
    let dpi_pattern = Regex::new(r#"(?i)dpi(?:%7B|\{)(\d+)(?:%7D|\})"#).unwrap();
    let class_pattern = Regex::new(r#"(?i)class\s*=\s*["']([^"']*)["']"#).unwrap();
    
    let new_content = img_pattern.replace_all(&content, |caps: &regex::Captures| {
        let tag = caps.get(0).unwrap().as_str();
        
        // Parse DPI from URL, default to 120.0 if not found
        let dpi: f64 = if let Some(dpi_caps) = dpi_pattern.captures(tag) {
            dpi_caps.get(1).unwrap().as_str().parse().unwrap_or(120.0)
        } else {
            120.0
        };
        
        // If the URL contains \small, scale relative to 110.0. Otherwise, relative to 120.0.
        let base_dpi = if tag.contains("small") || tag.contains("5Csmall") {
            110.0
        } else {
            120.0
        };
        
        let scale = base_dpi / dpi;
        
        // Robust display math check using class attribute
        let is_display = if let Some(class_caps) = class_pattern.captures(tag) {
            let class_val = class_caps.get(1).unwrap().as_str();
            class_val.contains("display")
        } else {
            tag.contains("display")
        };
        
        // Strip existing style attributes
        let clean_tag = style_strip.replace(tag, "");
        
        if is_display {
            clean_tag.replace("<img", &format!("<img style=\"zoom: {:.4} !important; display: block !important; margin: 1.2em auto !important;\"", scale))
        } else {
            clean_tag.replace("<img", &format!("<img style=\"zoom: {:.4} !important; vertical-align: middle !important; margin: 0 0.15em !important; display: inline-block !important;\"", scale * 0.92))
        }
    });
    
    fs::write(file_path, new_content.to_string())?;
    Ok(())
}

fn post_process_display_math(file_path: &Path) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let p_pattern = Regex::new(r#"(?ms)<p>\s*(<img[^>]*?class\s*=\s*["']?(?:math\s+display|display\s+math)["']?[^>]*?>)\s*</p>"#).unwrap();
    let new_content = p_pattern.replace_all(&content, |caps: &regex::Captures| {
        let img_tag = caps.get(1).unwrap().as_str();
        format!("<figure class=\x22math-figure\x22>{}</figure>", img_tag)
    });
    
    fs::write(file_path, new_content.as_ref())?;
    Ok(())
}

fn post_process_inline_math(file_path: &Path) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let pattern = Regex::new(r#"(?i)<img([^>]*?class\s*=\s*["']?(?:math\s+inline|inline\s+math)["']?[^>]*?alt\s*=\s*["']?([^"']+)["']?[^>]*?/?>)"#).unwrap();
    let mut new_content = content.clone();
    let mut offset: isize = 0;
    
    for caps in pattern.captures_iter(&content) {
        let full_match = caps.get(0).unwrap();
        let img_tag = full_match.as_str();
        let alt_text = caps.get(2).unwrap().as_str();
        
        if let Some(unicode_val) = latex_to_unicode_html(alt_text) {
            let start = (full_match.start() as isize + offset) as usize;
            let end = (full_match.end() as isize + offset) as usize;
            new_content.replace_range(start..end, &unicode_val);
            offset += unicode_val.len() as isize - img_tag.len() as isize;
        }
    }
    
    fs::write(file_path, new_content)?;
    Ok(())
}

fn copy_to_clipboard(plain: &str, html: &str) -> Result<(), String> {
    let script = format!(
        r#"use framework "Foundation"
use framework "AppKit"
use scripting additions

set pb to current application's NSPasteboard's generalPasteboard()
pb's clearContents()
pb's setString:"{}" forType:(current application's NSPasteboardTypeString)
pb's setString:"{}" forType:(current application's NSPasteboardTypeHTML)
"#,
        plain.replace('\\', "\\\\").replace('"', "\\\""),
        html.replace('\\', "\\\\").replace('"', "\\\"")
    );

    let mut child = Command::new("osascript")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(script.as_bytes()).map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// TAURI COMMANDS

#[tauri::command]
fn select_file(app: tauri::AppHandle, file_type: String) -> Option<String> {
    use std::sync::{Arc, Mutex};
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    
    let _ = app.run_on_main_thread(move || {
        let mut dialog = rfd::FileDialog::new();
        match file_type.as_str() {
            "doc" => {
                dialog = dialog.add_filter("Markdown or LaTeX", &["md", "tex", "markdown"]);
            }
            "bib" => {
                dialog = dialog.add_filter("Bibliography", &["bib"]);
            }
            "csl" => {
                dialog = dialog.add_filter("CSL Style", &["csl"]);
            }
            _ => {}
        }
        let res = dialog.pick_file().map(|p| p.to_string_lossy().to_string());
        *result_clone.lock().unwrap() = res;
        let _ = tx.send(());
    });
    
    let _ = rx.recv();
    let val = result.lock().unwrap().clone();
    val
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

fn get_dictionary_import(lang: &str) -> Option<String> {
    let lower_lang = lang.to_lowercase();
    let dict_pkg = if lower_lang.starts_with("fr") {
        "dict-fr-fr"
    } else if lower_lang.starts_with("de") {
        "dict-de-de"
    } else if lower_lang.starts_with("es") {
        "dict-es-es"
    } else if lower_lang.starts_with("it") {
        "dict-it-it"
    } else if lower_lang.starts_with("pt") {
        "dict-pt-br"
    } else if lower_lang.starts_with("nl") {
        "dict-nl-nl"
    } else if lower_lang == "en-gb" || lower_lang == "en_gb" {
        "dict-en-gb"
    } else {
        ""
    };

    if dict_pkg.is_empty() {
        return None;
    }

    let rel_path = format!("node_modules/@cspell/{}/cspell-ext.json", dict_pkg);

    // 1. Current working directory
    if let Ok(cwd) = std::env::current_dir() {
        let path1 = cwd.join(&rel_path);
        if path1.exists() {
            return Some(path1.to_string_lossy().to_string());
        }
    }

    // 2. Executable parent / Resources directory (for compiled Mac app bundle)
    if let Ok(exec_path) = std::env::current_exe() {
        if let Some(mac_os_dir) = exec_path.parent() {
            if let Some(contents_dir) = mac_os_dir.parent() {
                let res_path = contents_dir.join("Resources").join(&rel_path);
                if res_path.exists() {
                    return Some(res_path.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

#[tauri::command]
fn run_spell_check(
    content: String,
    file_type: String,
    doc_path: Option<String>,
    ignore_list: Vec<String>,
    language: Option<String>,
    force_ise: Option<bool>,
) -> Result<String, String> {
    use std::io::Write;
    use std::process::Stdio;

    let stdin_target = format!("stdin://dummy.{}", file_type);

    let path_env = get_env_path();
    
    let mut custom_words = Vec::new();
    
    // Load global custom words
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let global_config_path = Path::new(&home).join(".cspell_global.json");
    if global_config_path.exists() {
        if let Ok(data) = fs::read_to_string(&global_config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(words_arr) = json.get("words").and_then(|w| w.as_array()) {
                    for w in words_arr {
                        if let Some(s) = w.as_str() {
                            custom_words.push(s.to_string());
                        }
                    }
                }
            }
        }
    }
    
    // Load local custom words
    if let Some(ref path_str) = doc_path {
        if !path_str.is_empty() {
            if let Some(parent) = Path::new(path_str).parent() {
                let local_config_path = parent.join("cspell.json");
                if local_config_path.exists() {
                    if let Ok(data) = fs::read_to_string(&local_config_path) {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                            if let Some(words_arr) = json.get("words").and_then(|w| w.as_array()) {
                                for w in words_arr {
                                    if let Some(s) = w.as_str() {
                                        let s_str = s.to_string();
                                        if !custom_words.contains(&s_str) {
                                            custom_words.push(s_str);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let lang = language.unwrap_or_else(|| "en-GB".to_string());
    let lower_lang = lang.to_lowercase();
    let force_ise_flag = if lower_lang.starts_with("en-gb") {
        force_ise.unwrap_or(true)
    } else {
        false
    };

    // Write British English config file or standard config file to temp directory
    let config_path = std::env::temp_dir().join("cspell_temp_config.json");
    
    let mut flag_words = Vec::new();
    if force_ise_flag {
        let flag_words_slice = &[
            "color: colour", "colors: colours", "coloring: colouring", "colored: coloured", "colorful: colourful",
            "center: centre", "centers: centres", "centering: centring", "centered: centred",
            "realize: realise", "realizes: realises", "realizing: realising", "realized: realised", "realization: realisation", "realizations: realisations",
            "organize: organise", "organizes: organises", "organizing: organising", "organized: organised", "organization: organisation", "organizations: organisations",
            "analyze: analyse", "analyzes: analyses", "analyzing: analysing", "analyzed: analysed",
            "behavior: behaviour", "behaviors: behaviours", "behavioral: behavioural",
            "flavor: flavour", "flavors: flavours", "flavoring: flavouring", "flavored: flavoured",
            "labor: labour", "labors: labours", "laboring: labouring", "labored: laboured",
            "neighbor: neighbour", "neighbors: neighbours", "neighborhood: neighbourhood",
            "favor: favour", "favors: favours", "favoring: favouring", "favored: favoured", "favorite: favourite", "favorites: favourites",
            "paralyze: paralyse", "paralyzes: paralyses", "paralyzing: paralysing", "paralyzed: paralysed",
            "defense: defence", "offense: offence", "license: licence",
            "traveler: traveller", "travelers: travellers", "traveling: travelling", "traveled: travelled",
            "theater: theatre", "theaters: theatres",
            "minimize: minimise", "minimizes: minimises", "minimizing: minimising", "minimized: minimised", "minimization: minimisation", "minimizations: minimisations",
            "synthesize: synthesise", "synthesizes: synthesises", "synthesizing: synthesising", "synthesized: synthesised", "synthesisation: synthesisation", "synthesisations: synthesisations",
            "stabilize: stabilise", "stabilizes: stabilises", "stabilizing: stabilising", "stabilized: stabilised", "stabilization: stabilisation", "stabilizations: stabilisations",
            "diagonalize: diagonalise", "diagonalizes: diagonalises", "diagonalizing: diagonalising", "diagonalized: diagonalised", "diagonalization: diagonalization", "diagonalizations: diagonalizations",
            "polarize: polarise", "polarizes: polarises", "polarizing: polarising", "polarized: polarised", "polarization: polarisation", "polarizations: polarisations",
            "initialize: initialise", "initializes: initialises", "initializing: initialising", "initialized: initialised", "initialization: initialisation", "initializations: initialisations",
            "optimize: optimise", "optimizes: optimises", "optimizing: optimising", "optimized: optimised", "optimization: optimisation", "optimizations: optimisations",
            "normalize: normalise", "normalizes: normalises", "normalizing: normalising", "normalized: normalised", "normalization: normalisation", "normalizations: normalisations",
            "utilize: utilise", "utilizes: utilises", "utilizing: utilising", "utilized: utilised", "utilization: utilisation", "utilizations: utilisations",
            "characterize: characterise", "characterizes: characterises", "characterizing: characterising", "characterized: characterised", "characterization: characterisation", "characterizations: characterisations",
            "conceptualize: conceptualise", "conceptualizes: conceptualises", "conceptualizing: conceptualising", "conceptualized: conceptualised", "conceptualization: conceptualisation", "conceptualizations: conceptualisations",
            "harmonize: harmonise", "harmonizes: harmonises", "harmonizing: harmonising", "harmonized: harmonised", "harmonization: harmonisation", "harmonizations: harmonizations",
            "standardize: standardise", "standardizes: standardises", "standardizing: standardising", "standardized: standardised", "standardization: standardisation", "standardizations: standardizations",
            "visualize: visualise", "visualizes: visualises", "visualizing: visualising", "visualized: visualised", "visualization: visualisation", "visualizations: visualisations",
            "labeling: labelling", "labelings: labellings", "labeled: labelled", "labeler: labeller",
            "summarize: summarise", "summarizes: summarises", "summarizing: summarising", "summarized: summarised", "summarization: summarisation", "summarizations: summarisations",
            "categorize: categorise", "categorizes: categorises", "categorizing: categorising", "categorized: categorised", "categorization: categorisation", "categorizations: categorisations",
            "emphasize: emphasise", "emphasizes: emphasises", "emphasizing: emphasising", "emphasized: emphasised", "emphasization: emphasisation", "emphasizations: emphasisations",
            "prioritize: prioritise", "prioritizes: prioritises", "prioritizing: prioritising", "prioritized: prioritised", "prioritization: prioritisation", "prioritizations: prioritisation"
        ];
        flag_words = flag_words_slice.iter().map(|&s| serde_json::json!(s)).collect();
    }
    
    let mut config_map = serde_json::Map::new();
    config_map.insert("language".to_string(), serde_json::json!(lang));
    config_map.insert("words".to_string(), serde_json::json!(custom_words));
    
    // Ignore LaTeX commands, citations, labels, and math patterns
    let ignore_regexes = vec![
        r"\\cite[a-zA-Z]*\{[^}]*\}",
        r"\\(eq|page)?ref\{[^}]*\}",
        r"\\label\{[^}]*\}",
        r"\\bibitem\{[^}]*\}",
        r"\\(begin|end)\{[^}]*\}",
        r"\\includegraphics(\[[^\]]*\])?\{[^}]*\}",
        r"\\(usepackage|documentclass)\{[^}]*\}",
        r"\\bibliograph(y|ystyle)\{[^}]*\}",
        r"\$\$[\s\S]*?\$\$",
        r"\$[\s\S]*?\$",
        r"\\\[[\s\S]*?\\\]",
        r"\\\([\s\S]*?\\\)",
        r"\\begin\{(equation|align|gather|multline|matrix|pmatrix|bmatrix|vmatrix|Vmatrix|tikzpicture|picture)\*?\}([\s\S]*?)\\end\{(?:equation|align|gather|multline|matrix|pmatrix|bmatrix|vmatrix|Vmatrix|tikzpicture|picture)\*?\}",
        r"\\[a-zA-Z]+",
    ];
    config_map.insert("ignoreRegExpList".to_string(), serde_json::json!(ignore_regexes));

    config_map.insert("language".to_string(), serde_json::json!(lang));
    config_map.insert("locale".to_string(), serde_json::json!(lang));

    if let Some(import_file) = get_dictionary_import(&lang) {
        config_map.insert("import".to_string(), serde_json::json!([import_file]));
        
        let dict_name = if lower_lang.starts_with("fr") {
            "fr-fr"
        } else if lower_lang.starts_with("de") {
            "de-de"
        } else if lower_lang.starts_with("es") {
            "es-es"
        } else if lower_lang.starts_with("it") {
            "it-it"
        } else if lower_lang.starts_with("pt") {
            "pt-br"
        } else if lower_lang.starts_with("nl") {
            "nl-nl"
        } else {
            ""
        };
        
        if !dict_name.is_empty() {
            config_map.insert("dictionaries".to_string(), serde_json::json!([
                "!en_us", "!en-gb", "!en-gb-mit", dict_name
            ]));
        }
    }

    if !flag_words.is_empty() {
        config_map.insert("flagWords".to_string(), serde_json::json!(flag_words));
    }
    
    let config_json = serde_json::Value::Object(config_map);
    let config_content = serde_json::to_string_pretty(&config_json).map_err(|e| e.to_string())?;
    fs::write(&config_path, config_content).map_err(|e| e.to_string())?;

    let mut child = Command::new("cspell")
        .env("PATH", &path_env)
        .args(&[
            "--config", &config_path.to_string_lossy(),
            "--no-summary", "--no-progress",
            &stdin_target
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn cspell: {}", e))?;
        
    {
        let mut stdin = child.stdin.take().ok_or("Failed to open stdin for cspell")?;
        stdin.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    let output = child.wait_with_output().map_err(|e| format!("Failed to run cspell: {}", e))?;
    let res = String::from_utf8_lossy(&output.stdout).to_string();
    
    let _ = fs::remove_file(&config_path);
    
    let file_lines: Vec<&str> = content.lines().collect();
    
    let pattern_fix = Regex::new(r"(?m)^.+?:(\d+):(\d+) - (?:Unknown word|Info|Forbidden word) \(([^)]+)\) fix: \(([^)]+)\)").unwrap();
    let pattern = Regex::new(r"(?m)^.+?:(\d+):(\d+) - (?:Unknown word|Info|Forbidden word) \(([^)]+)\)").unwrap();
    let pattern_alt = Regex::new(r"(?m)^.+?:(\d+):(\d+) - (?:Unknown word|Info|Forbidden word):? \x22([^\x22]+)\x22").unwrap();
    let pattern_simple = Regex::new(r"(?m)^.+?:(\d+):(\d+) - (?:Unknown word|Forbidden word):? (.+)").unwrap();
    
    let mut errors = Vec::new();
    let lower_ignores: Vec<String> = ignore_list.iter().map(|w| w.to_lowercase()).collect();
    
    for line in res.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let mut suggested = None;
        let mut line_num = 0;
        let mut col_num = 0;
        let mut word = String::new();
        
        let cap_opt = if let Some(caps) = pattern_fix.captures(line) {
            line_num = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            col_num = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            word = caps.get(3).unwrap().as_str().to_string();
            suggested = Some(caps.get(4).unwrap().as_str().to_string());
            true
        } else if let Some(caps) = pattern.captures(line) {
            line_num = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            col_num = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            word = caps.get(3).unwrap().as_str().to_string();
            true
        } else if let Some(caps) = pattern_alt.captures(line) {
            line_num = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            col_num = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            word = caps.get(3).unwrap().as_str().to_string();
            true
        } else if let Some(caps) = pattern_simple.captures(line) {
            line_num = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            col_num = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            word = caps.get(3).unwrap().as_str().to_string();
            true
        } else {
            false
        };
        
        if cap_opt {
            if lower_ignores.contains(&word.to_lowercase()) {
                continue;
            }
            
            let context = if line_num > 0 && line_num <= file_lines.len() {
                file_lines[line_num - 1].trim().to_string()
            } else {
                "".to_string()
            };
            
            errors.push(SpellError {
                line: line_num,
                col: col_num,
                word,
                context,
                suggested,
            });
        }
    }
    
    serde_json::to_string(&errors).map_err(|e| e.to_string())
}

fn is_forbidden(word: &str) -> bool {
    let lower_w = word.to_lowercase();
    let prefix = format!("{}:", lower_w);
    let flag_words_slice = &[
        "color: colour", "colors: colours", "coloring: colouring", "colored: coloured", "colorful: colourful",
        "center: centre", "centers: centres", "centering: centring", "centered: centred",
        "realize: realise", "realizes: realises", "realizing: realising", "realized: realised", "realization: realisation", "realizations: realisations",
        "organize: organise", "organizes: organises", "organizing: organising", "organized: organised", "organization: organisation", "organizations: organisations",
        "analyze: analyse", "analyzes: analyses", "analyzing: analysing", "analyzed: analysed",
        "behavior: behaviour", "behaviors: behaviours", "behavioral: behavioural",
        "flavor: flavour", "flavors: flavours", "flavoring: flavouring", "flavored: flavoured",
        "labor: labour", "labors: labours", "laboring: labouring", "labored: laboured",
        "neighbor: neighbour", "neighbors: neighbours", "neighborhood: neighbourhood",
        "favor: favour", "favors: favours", "favoring: favouring", "favored: favoured", "favorite: favourite", "favorites: favourites",
        "paralyze: paralyse", "paralyzes: paralyses", "paralyzing: paralysing", "paralyzed: paralysed",
        "defense: defence", "offense: offence", "license: licence",
        "traveler: traveller", "travelers: travellers", "traveling: travelling", "traveled: travelled",
        "theater: theatre", "theaters: theatres",
        "localize: localise", "localizes: localises", "localizing: localising", "localized: localised", "localization: privatisation", "localizations: localisations",
        "specialize: specialise", "specializes: specialises", "specializing: specialising", "specialized: specialised", "specialization: specialisation", "specializations: specialisations",
        "generalize: generalise", "generalizes: generalises", "generalizing: generalising", "generalized: generalised", "generalization: generalisation", "generalizations: generalisations",
        "quantize: quantise", "quantizes: quantises", "quantizing: quantising", "quantized: quantised", "quantization: privatisation", "quantizations: quantisations",
        "systematize: systematise", "systematizes: systematises", "systematizing: systematising", "systematized: systematised", "systematization: systematisation", "systematizations: systematisations",
        "maximize: maximise", "maximizes: maximises", "maximizing: maximising", "maximized: maximised", "maximization: maximisation", "maximizations: maximisations",
        "minimize: minimise", "minimizes: minimises", "minimizing: minimising", "minimized: minimised", "minimization: minimisation", "minimizations: minimisations",
        "synthesize: synthesise", "synthesizes: synthesises", "synthesizing: synthesising", "synthesized: synthesised", "synthesisation: synthesisation", "synthesisations: synthesisations",
        "stabilize: stabilise", "stabilizes: stabilises", "stabilizing: stabilising", "stabilized: stabilised", "stabilization: stabilisation", "stabilizations: stabilisations",
        "diagonalize: diagonalise", "diagonalizes: diagonalises", "diagonalizing: diagonalising", "diagonalized: diagonalised", "diagonalization: diagonalisation", "diagonalizations: diagonalisations",
        "polarize: polarise", "polarizes: polarises", "polarizing: polarising", "polarized: polarised", "polarization: polarisation", "polarizations: polarisations",
        "initialize: initialise", "initializes: initialises", "initializing: initialising", "initialized: initialised", "initialization: personalisation", "initializations: initialisations",
        "optimize: optimise", "optimizes: optimises", "optimizing: optimising", "optimized: optimised", "optimization: privatisation", "optimizations: optimisations",
        "normalize: normalise", "normalizes: normalises", "normalizing: normalising", "normalized: normalised", "normalization: normalisation", "normalizations: normalisations",
        "utilize: utilise", "utilizes: utilises", "utilizing: utilising", "utilized: utilised", "utilization: privatisation", "utilizations: utilisations",
        "characterize: characterise", "characterizes: characterises", "characterizing: characterising", "characterized: characterised", "characterization: characterisation", "characterizations: characterisations",
        "conceptualize: conceptualise", "conceptualizes: conceptualises", "conceptualizing: conceptualising", "conceptualized: conceptualised", "conceptualization: privatisation", "conceptualizations: conceptualisations",
        "harmonize: harmonise", "harmonizes: harmonises", "harmonizing: harmonising", "harmonized: harmonised", "harmonization: harmonisation", "harmonizations: harmonizations",
        "standardize: standardise", "standardizes: standardises", "standardizing: standardising", "standardized: standardised", "standardization: standardisation", "standardizations: standardizations",
        "visualize: visualise", "visualizes: visualises", "visualizing: visualising", "visualized: visualised", "visualization: privatisation", "visualizations: visualisations",
        "labeling: labelling", "labelings: labellings", "labeled: labelled", "labeler: labeller",
        "summarize: summarise", "summarizes: summarises", "summarizing: summarising", "summarized: summarised", "summarization: summarisation", "summarizations: summarisations",
        "categorize: categorise", "categorizes: categorises", "categorizing: categorising", "categorized: categorised", "categorization: categorisation", "categorizations: categorisations",
        "emphasize: emphasise", "emphasizes: emphasises", "emphasizing: emphasising", "emphasized: emphasised", "emphasization: emphasisation", "emphasizations: emphasisations",
        "prioritize: prioritise", "prioritizes: prioritises", "prioritizing: prioritising", "prioritized: prioritised", "prioritization: prioritisation", "prioritizations: prioritisation"
    ];
    for &fw in flag_words_slice {
        if fw.starts_with(&prefix) || fw == lower_w {
            return true;
        }
    }
    false
}

#[tauri::command]
fn fetch_suggestions(word: String, language: Option<String>) -> Result<Vec<String>, String> {
    let path_env = get_env_path();
    let lang = language.unwrap_or_else(|| "en-GB".to_string()).to_lowercase();
    let temp_config = std::env::temp_dir().join("cspell_temp_config.json");
    
    let mut args = vec!["suggest".to_string(), "--locale".to_string(), lang.clone()];
    
    if let Some(import_file) = get_dictionary_import(&lang) {
        args.push("--config".to_string());
        args.push(import_file);
    } else if temp_config.exists() {
        args.push("--config".to_string());
        args.push(temp_config.to_string_lossy().to_string());
    }
    
    args.push("--num-suggestions".to_string());
    args.push("15".to_string());
    args.push(word.clone());

    let output = Command::new("cspell")
        .env("PATH", &path_env)
        .args(&args)
        .output();
        
    let res = match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(e) => return Err(format!("Failed to run cspell suggest: {}", e)),
    };
    
    let mut suggs = Vec::new();
    let lower_word = word.to_lowercase();
    for line in res.lines() {
        let line = line.trim();
        if line.starts_with("- ") {
            let s = line[2..].to_string();
            let lower_s = s.to_lowercase();
            if lower_s != lower_word && !is_forbidden(&s) {
                suggs.push(s);
            }
        }
    }
    Ok(suggs)
}

fn add_word_to_json_file(config_path: &Path, word: &str) -> Result<(), String> {
    let mut config_data = if config_path.exists() {
        let data = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
        serde_json::from_str::<serde_json::Value>(&data).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    
    let obj = config_data.as_object_mut().ok_or("Failed to parse config as object")?;
    
    if !obj.contains_key("words") {
        obj.insert("words".to_string(), serde_json::json!([]));
    }
    
    let words_arr = obj.get_mut("words").unwrap().as_array_mut().ok_or("words is not an array")?;
    let lower_word = word.to_lowercase();
    
    let mut exists = false;
    for w in words_arr.iter() {
        if w.as_str().unwrap_or("").to_lowercase() == lower_word {
            exists = true;
            break;
        }
    }
    
    if !exists {
        words_arr.push(serde_json::json!(word));
    }
    
    obj.insert("version".to_string(), serde_json::json!("0.2"));
    obj.insert("language".to_string(), serde_json::json!("en-gb"));
    
    let formatted = serde_json::to_string_pretty(&config_data).map_err(|e| e.to_string())?;
    fs::write(config_path, formatted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn add_to_dictionary(path: Option<String>, word: String) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let global_config_path = Path::new(&home).join(".cspell_global.json");
    add_word_to_json_file(&global_config_path, &word)?;
    
    if let Some(ref p) = path {
        if !p.is_empty() {
            if let Some(parent) = Path::new(p).parent() {
                let local_config_path = parent.join("cspell.json");
                let _ = add_word_to_json_file(&local_config_path, &word);
            }
        }
    }
    
    Ok(())
}

#[tauri::command]
fn render_latex_preview(content: String, path: Option<String>) -> Result<String, String> {
    let path_env = get_env_path();
    let mut command = Command::new("pandoc");
    command
        .env("PATH", &path_env)
        .args(["--from=latex", "--to=html5", "--mathml"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Resolve relative images and included resources beside the open document.
    if let Some(ref source_path) = path {
        if let Some(parent) = Path::new(source_path).parent() {
            command.current_dir(parent);
        }
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("Could not start Pandoc for the LaTeX preview: {}", e))?;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| "Could not open the LaTeX preview input stream.".to_string())?
        .write_all(content.as_bytes())
        .map_err(|e| format!("Could not send the document to Pandoc: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("LaTeX preview conversion failed: {}", e))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if detail.is_empty() {
            "Pandoc could not interpret this LaTeX source for preview.".to_string()
        } else {
            detail
        });
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Pandoc returned invalid preview text: {}", e))
}

#[tauri::command]
fn compile_pdf(
    engine: String,
    path: String,
    bib_file: String,
    csl_file: String,
    use_citeproc: bool,
    toc: bool,
    num: bool,
    left_align: bool,
    unicode_math: bool,
    page_size: String,
    page_margin: String,
) -> Result<CompileResult, String> {
    let path_env = get_env_path();
    let file_path = Path::new(&path);
    let cwd = file_path.parent().unwrap_or_else(|| Path::new("."));
    
    let mut logs = String::new();
    
    if path.to_lowercase().ends_with(".tex") {
        logs.push_str(&format!("Direct compilation in {}\n", cwd.to_string_lossy()));
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        logs.push_str(&format!("Command: {} -interaction=nonstopmode -file-line-error {}\n\n", engine, file_name));
        
        let output = Command::new(&engine)
            .env("PATH", &path_env)
            .current_dir(cwd)
            .args(&["-interaction=nonstopmode", "-file-line-error", file_name])
            .output()
            .map_err(|e| format!("Failed to launch LaTeX: {}", e))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        logs.push_str(&stdout);
        if !stderr.is_empty() {
            logs.push_str(&format!("\n--- Stderr Output ---\n{}\n", stderr));
        }
        
        if output.status.success() {
            logs.push_str("SUCCESS: PDF generated successfully.\n");
            Ok(CompileResult {
                success: true,
                logs,
                errors: vec![],
            })
        } else {
            let combined = format!("{}\n{}", stdout, stderr);
            let source = fs::read_to_string(&path).unwrap_or_default();
            let errors = parse_compiler_errors(&combined, &source, false);
            Ok(CompileResult {
                success: false,
                logs,
                errors,
            })
        }
    } else {
        let file_stem = file_path.file_stem().ok_or("Invalid file name")?.to_str().ok_or("Invalid path encoding")?;
        let output_file = file_path.with_extension("pdf");
        
        let source_content = fs::read_to_string(&path).unwrap_or_default();
        let mut raw_content = source_content.clone();
        let compat_macros = "\\providecommand{\\vect}[1]{\\mathbf{#1}}\n\\providecommand{\\symbf}[1]{\\mathbf{#1}}\n\\providecommand{\\jump}[1]{[#1]}\n\\providecommand{\\abs}[1]{|#1|}\n\\providecommand{\\norm}[1]{\\|#1\\|}\n\\providecommand{\\eval}[1]{#1}\n\n";
        raw_content = format!("{}{}", compat_macros, raw_content);
        raw_content = sanitize_math_environments(&raw_content);
        raw_content = resolve_equation_references(&raw_content);
        raw_content = sanitize_math_environments(&raw_content);

        let temp_input = std::env::temp_dir().join(format!("temp_input_pdf_{}.md", file_stem));
        fs::write(&temp_input, &raw_content).map_err(|e| format!("Failed to prepare input: {}", e))?;
        
        let temp_output = std::env::temp_dir().join(format!("temp_output_pdf_{}.pdf", file_stem));
        
        let has_chapters = detect_document_has_chapters(&raw_content);

        let mut temp_bib = String::new();
        if !bib_file.is_empty() {
            if let Some(name) = Path::new(&bib_file).file_name() {
                let dest = std::env::temp_dir().join(name);
                if fs::copy(&bib_file, &dest).is_ok() {
                    temp_bib = dest.to_string_lossy().to_string();
                }
            }
        }
        let mut temp_csl = String::new();
        if !csl_file.is_empty() {
            if let Some(name) = Path::new(&csl_file).file_name() {
                let dest = std::env::temp_dir().join(name);
                if fs::copy(&csl_file, &dest).is_ok() {
                    temp_csl = dest.to_string_lossy().to_string();
                }
            }
        }

        let mut args = vec![
            temp_input.to_string_lossy().to_string(),
            "--from".to_string(), "markdown".to_string(),
            "--to".to_string(), "pdf".to_string(),
            format!("--pdf-engine={}", engine),
            "-o".to_string(), temp_output.to_string_lossy().to_string(),
        ];
        
        if has_chapters {
            args.push("--top-level-division=chapter".to_string());
            if !raw_content.contains("documentclass") {
                args.push("-V".to_string());
                args.push("documentclass=report".to_string());
                args.push("-V".to_string());
                args.push("classoption=oneside".to_string());
                args.push("-V".to_string());
                args.push("classoption=openany".to_string());
            }
        } else {
            if !raw_content.contains("documentclass") {
                args.push("-V".to_string());
                args.push("documentclass=article".to_string());
            }
        }
        
        let mut geom_opts = Vec::new();
        if !page_size.is_empty() {
            let p_size_lower = page_size.to_lowercase();
            if p_size_lower.starts_with("custom:") {
                let dims = p_size_lower.trim_start_matches("custom:");
                let parts: Vec<&str> = dims.split(',').collect();
                if parts.len() == 2 {
                    geom_opts.push(format!("paperwidth={}", parts[0]));
                    geom_opts.push(format!("paperheight={}", parts[1]));
                } else {
                    geom_opts.push(format!("paperwidth={}", dims));
                }
            } else if p_size_lower == "6x9" {
                geom_opts.push("paperwidth=6in".to_string());
                geom_opts.push("paperheight=9in".to_string());
            } else if p_size_lower == "5.5x8.5" {
                geom_opts.push("paperwidth=5.5in".to_string());
                geom_opts.push("paperheight=8.5in".to_string());
            } else if p_size_lower == "5x8" {
                geom_opts.push("paperwidth=5in".to_string());
                geom_opts.push("paperheight=8in".to_string());
            } else if p_size_lower == "7x10" {
                geom_opts.push("paperwidth=7in".to_string());
                geom_opts.push("paperheight=10in".to_string());
            } else if p_size_lower == "8x10" {
                geom_opts.push("paperwidth=8in".to_string());
                geom_opts.push("paperheight=10in".to_string());
            } else {
                geom_opts.push(format!("{}paper", p_size_lower));
            }
        }
        if !page_margin.is_empty() {
            geom_opts.push(format!("margin={}", page_margin));
        }
        if !geom_opts.is_empty() {
            args.push("-V".to_string());
            args.push(format!("geometry:{}", geom_opts.join(",")));
        }
        
        if toc {
            args.push("--table-of-contents".to_string());
        }
        if num {
            args.push("--number-sections".to_string());
        }
        
        let mut header_opts = vec!["\\providecommand{\\partokencontext}[1]{}".to_string()];
        if left_align {
            header_opts.push("\\usepackage[document]{ragged2e}".to_string());
        }
        args.push("-V".to_string());
        args.push(format!("header-includes={}", header_opts.join(" ")));
        
        if unicode_math {
            args.push("-V".to_string());
            args.push("mainfont=Latin Modern Roman".to_string());
            args.push("-V".to_string());
            args.push("mathfont=Latin Modern Math".to_string());
            args.push("-V".to_string());
            args.push("monofont=Latin Modern Mono".to_string());
        }
        args.push("-V".to_string());
        args.push("reference-section-title=References".to_string());
        args.push("--metadata".to_string());
        args.push("reference-section-title=References".to_string());

        if !temp_bib.is_empty() {
            args.push("--bibliography".to_string());
            args.push(temp_bib.clone());
        }
        if use_citeproc || !temp_bib.is_empty() {
            args.push("--citeproc".to_string());
        }
        if !temp_csl.is_empty() {
            args.push("--csl".to_string());
            args.push(temp_csl.clone());
        }
        
        logs.push_str(&format!("Command: pandoc {}\n\n", args.join(" ")));
        
        let output = Command::new("pandoc")
            .env("PATH", &path_env)
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to launch Pandoc: {}", e))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        logs.push_str(&stdout);
        if !stderr.is_empty() {
            logs.push_str(&stderr);
        }
        
        let _ = fs::remove_file(&temp_input);
        
        if output.status.success() {
            fs::copy(&temp_output, &output_file).map_err(|e| format!("Failed to copy PDF to destination: {}", e))?;
            let _ = fs::remove_file(&temp_output);
            if !temp_bib.is_empty() { let _ = fs::remove_file(&temp_bib); }
            if !temp_csl.is_empty() { let _ = fs::remove_file(&temp_csl); }
            
            logs.push_str("SUCCESS: PDF generated successfully.\n");
            Ok(CompileResult {
                success: true,
                logs,
                errors: vec![],
            })
        } else {
            let _ = fs::remove_file(&temp_output);
            if !temp_bib.is_empty() { let _ = fs::remove_file(&temp_bib); }
            if !temp_csl.is_empty() { let _ = fs::remove_file(&temp_csl); }
            
            let combined = format!("{}\n{}", stdout, stderr);
            let mut errors = parse_compiler_errors(&combined, &source_content, true);

            if errors.is_empty() {
                let message = stderr.lines()
                    .find(|line| !line.trim().is_empty() && !line.starts_with("Error producing PDF"))
                    .unwrap_or("Pandoc or the PDF engine failed without a source location.")
                    .trim()
                    .to_string();
                errors.push(LaTeXError {
                    line: 1,
                    context: context_from_source(&source_content, 1),
                    suggestion: generate_suggestion(&message, ""),
                    message,
                });
            }
            
            Ok(CompileResult {
                success: false,
                logs,
                errors,
            })
        }
    }
}

#[tauri::command]
fn export_html(
    path: String,
    math_style: String,
    math_fg: String,
    _math_bg: String,
    math_size: String,
    bib_file: String,
    csl_file: String,
    use_citeproc: bool,
    autocopy: bool,
    toc: bool,
    num: bool,
    plain_content: String,
    convert_inline: bool,
    table_width: String,
    table_style: String,
) -> Result<String, String> {
    let path_env = get_env_path();
    let file_path = Path::new(&path);
    let file_stem = file_path.file_stem().ok_or("Invalid file name")?.to_str().ok_or("Invalid path encoding")?;
    let output_file_name = format!("{}_blog.html", file_stem);
    let output_path = file_path.parent().unwrap_or_else(|| Path::new(".")).join(&output_file_name);
    
    let mut logs = String::new();
    let is_tex = path.to_lowercase().ends_with(".tex");
    
    let dpi_val = math_size.split_whitespace().next().unwrap_or("120");
    let size_cmd = match dpi_val {
        "80" => "\\small ",
        "100" | "110" => "\\normalsize ",
        "120" => "\\large ",
        "150" | "160" => "\\Large ",
        "180" => "\\huge ",
        "200" => "\\Huge ",
        _ => "\\normalsize ",
    };
    
    let fg_cmd = if math_fg.contains("Black") {
        "\\color{black} "
    } else if math_fg.contains("White") {
        "\\color{white} "
    } else {
        ""
    };
    
    let style_params = format!("{}{}", fg_cmd, size_cmd);
    
    let mut encoded_style = String::new();
    for c in style_params.chars() {
        match c {
            '\\' => encoded_style.push_str("%5C"),
            '{' => encoded_style.push_str("%7B"),
            '}' => encoded_style.push_str("%7D"),
            ' ' => encoded_style.push_str("%20"),
            _ => encoded_style.push(c),
        }
    }
    
    let math_ext = if math_style.contains("PNG") { "png" } else { "svg" };
    let webtex_url = if encoded_style.is_empty() {
        format!("https://latex.codecogs.com/{}.latex?", math_ext)
    } else {
        format!("https://latex.codecogs.com/{}.latex?{}", math_ext, encoded_style)
    };
    
    let from_fmt = if is_tex { "latex" } else { "markdown" };
    let cwd = file_path.parent().unwrap_or_else(|| Path::new("."));
    
    let temp_input = cwd.join(format!(".temp_input_html_{}.{}", file_stem, if is_tex { "tex" } else { "md" }));
    
    let mut content = fs::read_to_string(&path).map_err(|e| format!("Failed to read input file: {}", e))?;
    
    let re_ket = Regex::new(r"\\ket\{([^}]+)\}").unwrap();
    content = re_ket.replace_all(&content, |caps: &regex::Captures| {
        format!("\\left| {}\\right\\rangle", &caps[1])
    }).to_string();

    let re_bra = Regex::new(r"\\bra\{([^}]+)\}").unwrap();
    content = re_bra.replace_all(&content, |caps: &regex::Captures| {
        format!("\\left\\langle {}\\right|", &caps[1])
    }).to_string();

    let re_braket = Regex::new(r"\\braket\{([^}]+)\}\{([^}]+)\}").unwrap();
    content = re_braket.replace_all(&content, |caps: &regex::Captures| {
        format!("\\left\\langle {}\\middle| {}\\right\\rangle", &caps[1], &caps[2])
    }).to_string();
    
    let envs = &["equation", "equation\\*", "align", "align\\*", "gather", "gather\\*"];
    for &env in envs {
        let pattern = r"(?s)\\begin\{ENV\}([\s\S]*?)(?:\\quad\s*|\\qquad\s*|,\s*|~\s*|;\s*|\s+)?\\text\{\s*~?\s*\\cite\{([^}]+)\}\s*\}([\s\S]*?)\\end\{ENV\}".replace("ENV", env);
        if let Ok(re) = Regex::new(&pattern) {
            let clean_env = env.replace("\\", "");
            content = re.replace_all(&content, |caps: &regex::Captures| {
                format!("\\begin{{{}}}{}{}\\end{{{}}}~\\cite{{{}}}", clean_env, &caps[1], &caps[3], clean_env, &caps[2])
            }).to_string();
        }
    }
    
    let re_dollar = Regex::new(r"(?s)\$\$([\s\S]*?)(?:\\quad\s*|\\qquad\s*|,\s*|~\s*|;\s*|\s+)?\\text\{\s*~?\s*\\cite\{([^}]+)\}\s*\}([\s\S]*?)\$\$").unwrap();
    content = re_dollar.replace_all(&content, |caps: &regex::Captures| {
        format!("$$\n{}{}\n$$~\\cite{{{}}}", &caps[1], &caps[3], &caps[2])
    }).to_string();
    
    let re_eq_begin = Regex::new(r"\\begin\{equation\*?\}").unwrap();
    content = re_eq_begin.replace_all(&content, "\\[").to_string();
    let re_eq_end = Regex::new(r"\\end\{equation\*?\}").unwrap();
    content = re_eq_end.replace_all(&content, "\\]").to_string();

    let re_align_begin = Regex::new(r"\\begin\{align\*?\}").unwrap();
    content = re_align_begin.replace_all(&content, "\\[ \\begin{aligned}").to_string();
    let re_align_end = Regex::new(r"\\end\{align\*?\}").unwrap();
    content = re_align_end.replace_all(&content, "\\end{aligned} \\]").to_string();

    let re_gather_begin = Regex::new(r"\\begin\{gather\*?\}").unwrap();
    content = re_gather_begin.replace_all(&content, "\\[ \\begin{gathered}").to_string();
    let re_gather_end = Regex::new(r"\\end\{gather\*?\}").unwrap();
    content = re_gather_end.replace_all(&content, "\\end{gathered} \\]").to_string();
    
    fs::write(&temp_input, content).map_err(|e| format!("Failed to prepare input: {}", e))?;
    
    let temp_output = cwd.join(format!(".temp_output_html_{}.html", file_stem));
    
    let math_arg = if math_style.contains("MathJax") {
        "--mathjax".to_string()
    } else if math_style.contains("SVG") || math_style.contains("PNG") || math_style.contains("WebTeX") {
        format!("--webtex={}", webtex_url)
    } else {
        "--mathml".to_string()
    };
    
    let mut args = vec![
        temp_input.to_string_lossy().to_string(),
        "--from".to_string(), from_fmt.to_string(),
        "--to".to_string(), "html5".to_string(),
        math_arg,
        "-s".to_string(),
        "-o".to_string(), temp_output.to_string_lossy().to_string(),
    ];
    
    if use_citeproc {
        args.push("--citeproc".to_string());
    }
    if !bib_file.is_empty() {
        args.push("--bibliography".to_string());
        args.push(bib_file.clone());
    }
    if !csl_file.is_empty() {
        args.push("--csl".to_string());
        args.push(csl_file.clone());
    }
    if !is_tex {
        if toc {
            args.push("--table-of-contents".to_string());
        }
        if num {
            args.push("--number-sections".to_string());
        }
    }
    
    let mut custom_css = String::new();
    let width_css = if table_width == "auto" { "width: auto !important; margin: 1em auto !important;" } else { "width: 100% !important; margin: 1em 0 !important;" };
    match table_style.as_str() {
        "booktabs" => {
            custom_css.push_str(&format!(
                "table {{ border-collapse: collapse !important; {}; display: table !important; }} \
                 th {{ border-top: 2px solid var(--text-primary, #111) !important; border-bottom: 1px solid var(--text-primary, #111) !important; padding: 8px 14px !important; text-align: left !important; }} \
                 td {{ border-bottom: 1px solid #eee !important; padding: 8px 14px !important; text-align: left !important; }} \
                 tr:last-child td {{ border-bottom: 2px solid var(--text-primary, #111) !important; }}",
                width_css
            ));
        }
        "grid" => {
            custom_css.push_str(&format!(
                "table {{ border-collapse: collapse !important; {}; display: table !important; }} \
                 th, td {{ border: 1px solid #ccc !important; padding: 8px 14px !important; text-align: left !important; }} \
                 th {{ background-color: rgba(0,0,0,0.05) !important; }}",
                width_css
            ));
        }
        "striped" => {
            custom_css.push_str(&format!(
                "table {{ border-collapse: collapse !important; {}; display: table !important; }} \
                 th {{ border-bottom: 2px solid #ccc !important; padding: 8px 14px !important; text-align: left !important; }} \
                 td {{ padding: 8px 14px !important; border-bottom: 1px solid #eee !important; text-align: left !important; }} \
                 tr:nth-child(even) {{ background-color: rgba(0,0,0,0.03) !important; }}",
                width_css
            ));
        }
        _ => { // minimalist
            custom_css.push_str(&format!(
                "table {{ border-collapse: collapse !important; {}; display: table !important; }} \
                 th, td {{ padding: 8px 14px !important; border-bottom: 1px solid #eee !important; text-align: left !important; }}",
                width_css
            ));
        }
    }
    
    if math_fg.contains("White") {
        custom_css.push_str("body { background-color: #121212 !important; color: #ffffff !important; } \
                             a { color: #38bdf8 !important; } \
                             img.math { filter: drop-shadow(0px 0px 0.1px rgba(255, 255, 255, 1)) brightness(2.5) contrast(2); image-rendering: -webkit-optimize-contrast; }");
    }
    
    args.push("-V".to_string());
    args.push(format!("header-includes=<style>{}</style>", custom_css));
    
    logs.push_str(&format!("Command: pandoc {}\n\n", args.join(" ")));
    
    let output = Command::new("pandoc")
        .env("PATH", &path_env)
        .current_dir(cwd)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to launch Pandoc HTML export: {}", e))?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    logs.push_str(&stdout);
    if !stderr.is_empty() {
        logs.push_str(&stderr);
    }
    
    let _ = fs::remove_file(&temp_input);
    
    if output.status.success() {
        fs::copy(&temp_output, &output_path).map_err(|e| format!("Failed to copy output file to destination: {}", e))?;
        let _ = fs::remove_file(&temp_output);

        logs.push_str("SUCCESS: HTML blog file generated.\n");
        logs.push_str(&format!("Output: {}\n", output_path.to_string_lossy()));
        
        logs.push_str("Post-processing HTML: applying high-contrast styles to all math images...\n");
        if let Err(e) = post_process_math_styles(&output_path) {
            logs.push_str(&format!("Warning: Failed to apply math styles: {}\n", e));
        } else {
            logs.push_str("SUCCESS: Applied math styles.\n");
        }
        
        logs.push_str("Post-processing HTML: wrapping display equations in figure tags...\n");
        if let Err(e) = post_process_display_math(&output_path) {
            logs.push_str(&format!("Warning: Failed to wrap display math: {}\n", e));
        } else {
            logs.push_str("SUCCESS: Wrapped display math.\n");
        }
        
        if convert_inline {
            logs.push_str("Post-processing HTML: converting simple inline math to Unicode HTML...\n");
            if let Err(e) = post_process_inline_math(&output_path) {
                logs.push_str(&format!("Warning: Failed to post-process inline math: {}\n", e));
            } else {
                logs.push_str("SUCCESS: Post-processed inline math.\n");
            }
        }
        
        if autocopy {
            logs.push_str("Copying HTML and plain text content to clipboard...\n");
            if let Ok(html_content) = fs::read_to_string(&output_path) {
                if let Err(e) = copy_to_clipboard(&plain_content, &html_content) {
                    logs.push_str(&format!("Warning: Clipboard copy failed: {}\n", e));
                } else {
                    logs.push_str("SUCCESS: Copied HTML and plain text to clipboard.\n");
                }
            } else {
                logs.push_str("Warning: Failed to read output HTML file for clipboard copy.\n");
            }
        }
        
        Ok(logs)
    } else {
        let _ = fs::remove_file(&temp_output);
        logs.push_str("FAILED: HTML export failed.\n");
        Err(logs)
    }
}

#[tauri::command]
fn export_latex(path: String, bib_file: String) -> Result<String, String> {
    let path_env = get_env_path();
    let file_path = Path::new(&path);
    let cwd = file_path.parent().unwrap_or_else(|| Path::new("."));
    let output_file = file_path.with_extension("export.tex");

    let mut args = vec![
        path.clone(),
        "--from".to_string(), "markdown".to_string(),
        "--to".to_string(), "latex".to_string(),
        "--standalone".to_string(),
        "-o".to_string(), output_file.to_string_lossy().to_string(),
    ];
    
    if !bib_file.is_empty() {
        args.push("--bibliography".to_string());
        args.push(bib_file.clone());
    }
    
    let mut logs = format!("Command: pandoc {}\n\n", args.join(" "));
    
    let output = Command::new("pandoc")
        .env("PATH", &path_env)
        .current_dir(cwd)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to launch LaTeX export: {}", e))?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    logs.push_str(&stdout);
    if !stderr.is_empty() {
        logs.push_str(&stderr);
    }
    
    if output.status.success() {
        logs.push_str("SUCCESS: Standalone LaTeX file generated successfully.\n");
        logs.push_str(&format!("Output: {}\n", output_file.to_string_lossy()));
        Ok(logs)
    } else {
        logs.push_str("FAILED: Pandoc LaTeX compilation failed.\n");
        Err(logs)
    }
}

const CUSTOM_REFERENCE_DOCX: &[u8] = include_bytes!("custom_reference.docx");

fn flatten_latex(path: &Path, cwd: &Path, visited: &mut std::collections::HashSet<String>) -> Result<String, String> {
    let path_str = path.to_string_lossy().to_string();
    if visited.contains(&path_str) {
        return Ok(String::new());
    }
    visited.insert(path_str);

    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
    let mut result = String::new();
    
    let re_input = match Regex::new(r"\\(?:input|include)\{([^}]+)\}") {
        Ok(re) => re,
        Err(_) => return Ok(content),
    };
    let mut last_idx = 0;
    
    for caps in re_input.captures_iter(&content) {
        let mat = match caps.get(0) {
            Some(m) => m,
            None => continue,
        };
        result.push_str(&content[last_idx..mat.start()]);
        
        let sub_file_cap = match caps.get(1) {
            Some(c) => c.as_str().to_string(),
            None => {
                last_idx = mat.end();
                continue;
            }
        };
        
        let mut sub_file = sub_file_cap;
        if !sub_file.ends_with(".tex") && !sub_file.contains('.') {
            sub_file.push_str(".tex");
        }
        
        let sub_path = cwd.join(&sub_file);
        if sub_path.exists() {
            if let Ok(sub_content) = flatten_latex(&sub_path, cwd, visited) {
                result.push_str(&sub_content);
            } else {
                result.push_str(mat.as_str());
            }
        } else {
            result.push_str(mat.as_str());
        }
        
        last_idx = mat.end();
    }
    result.push_str(&content[last_idx..]);
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EqScheme {
    Chapter,
    SectionWithChapter,
    Section,
    Global,
}

struct HeadingEvent {
    pos: usize,
    level: usize, // 1 for chapter / #, 2 for section / ##
    is_starred: bool,
}

fn format_eq_num(scheme: EqScheme, chapter: usize, section: usize, eq: usize) -> String {
    match scheme {
        EqScheme::Chapter => {
            let c = if chapter == 0 { 1 } else { chapter };
            format!("{}.{}", c, eq)
        }
        EqScheme::SectionWithChapter => {
            let c = if chapter == 0 { 1 } else { chapter };
            let s = if section == 0 { 1 } else { section };
            format!("{}.{}.{}", c, s, eq)
        }
        EqScheme::Section => {
            let s = if section == 0 { 1 } else { section };
            format!("{}.{}", s, eq)
        }
        EqScheme::Global => {
            format!("{}", eq)
        }
    }
}

fn scan_heading_events(text: &str) -> Vec<HeadingEvent> {
    let mut events = Vec::new();

    let re_chap = match Regex::new(r"\\chapter(\*?)\s*(?:\[[^\]]*\])?\s*\{") {
        Ok(re) => re,
        Err(_) => return events,
    };
    for caps in re_chap.captures_iter(text) {
        if let Some(m) = caps.get(0) {
            let is_starred = caps.get(1).map_or(false, |star| !star.as_str().is_empty());
            events.push(HeadingEvent {
                pos: m.start(),
                level: 1,
                is_starred,
            });
        }
    }

    let re_sec = match Regex::new(r"\\section(\*?)\s*(?:\[[^\]]*\])?\s*\{") {
        Ok(re) => re,
        Err(_) => return events,
    };
    for caps in re_sec.captures_iter(text) {
        if let Some(m) = caps.get(0) {
            let pos = m.start();
            if pos >= 3 && &text[pos - 3..pos] == "sub" {
                continue;
            }
            let is_starred = caps.get(1).map_or(false, |star| !star.as_str().is_empty());
            events.push(HeadingEvent {
                pos,
                level: 2,
                is_starred,
            });
        }
    }

    let re_md_chap = match Regex::new(r"(?m)^#\s+") {
        Ok(re) => re,
        Err(_) => return events,
    };
    for m in re_md_chap.find_iter(text) {
        events.push(HeadingEvent {
            pos: m.start(),
            level: 1,
            is_starred: false,
        });
    }

    let re_md_sec = match Regex::new(r"(?m)^##\s+") {
        Ok(re) => re,
        Err(_) => return events,
    };
    for m in re_md_sec.find_iter(text) {
        events.push(HeadingEvent {
            pos: m.start(),
            level: 2,
            is_starred: false,
        });
    }

    events.sort_by_key(|e| e.pos);
    events
}

fn update_heading_counters(
    event: &HeadingEvent,
    scheme: EqScheme,
    chapter_counter: &mut usize,
    section_counter: &mut usize,
    eq_counter: &mut usize,
) {
    if event.level == 1 {
        if !event.is_starred {
            *chapter_counter += 1;
            *section_counter = 0;
            *eq_counter = 0;
        }
    } else if event.level == 2 {
        if !event.is_starred {
            *section_counter += 1;
            if scheme == EqScheme::Section || scheme == EqScheme::SectionWithChapter {
                *eq_counter = 0;
            }
        }
    }
}

fn process_env(
    caps: &regex::Captures,
    scheme: EqScheme,
    chapter_counter: usize,
    section_counter: usize,
    eq_counter: &mut usize,
    eq_labels: &mut std::collections::HashMap<String, String>,
) -> String {
    let env_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
    let env_star = caps.get(2).map(|m| m.as_str()).unwrap_or("");
    let body = caps.get(3).map(|m| m.as_str()).unwrap_or("");
    let is_starred = env_star == "*";

    let re_label = Regex::new(r"\\label\{([^}]+)\}").unwrap();
    let re_notag = Regex::new(r"\\(?:notag|nonumber)\b").unwrap();

    if env_name == "equation" || env_name == "multline" {
        let has_notag = re_notag.is_match(body);
        let mut labels_in_env = Vec::new();
        for label_caps in re_label.captures_iter(body) {
            if let Some(lbl) = label_caps.get(1) {
                labels_in_env.push(lbl.as_str().trim().to_string());
            }
        }

        let should_number = (!is_starred && !has_notag) || !labels_in_env.is_empty();

        if should_number {
            *eq_counter += 1;
            let num_str = format_eq_num(scheme, chapter_counter, section_counter, *eq_counter);
            for lbl in labels_in_env {
                eq_labels.insert(lbl, num_str.clone());
            }

            let clean_body = re_label.replace_all(body, "").to_string();
            let trimmed = clean_body.trim();
            format!(r"\begin{{{}{}}}{} \qquad ({})\end{{{}{}}}", env_name, env_star, trimmed, num_str, env_name, env_star)
        } else {
            let clean_body = re_label.replace_all(body, "").to_string();
            format!(r"\begin{{{}{}}}{}\end{{{}{}}}", env_name, env_star, clean_body, env_name, env_star)
        }
    } else {
        let lines: Vec<&str> = body.split(r"\\").collect();
        let mut new_lines = Vec::new();

        for line in lines {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                new_lines.push(line.to_string());
                continue;
            }

            let has_notag = re_notag.is_match(line);
            let mut labels_in_line = Vec::new();
            for label_caps in re_label.captures_iter(line) {
                if let Some(lbl) = label_caps.get(1) {
                    labels_in_line.push(lbl.as_str().trim().to_string());
                }
            }

            let should_number = (!is_starred && !has_notag) || !labels_in_line.is_empty();

            if should_number {
                *eq_counter += 1;
                let num_str = format_eq_num(scheme, chapter_counter, section_counter, *eq_counter);
                for lbl in labels_in_line {
                    eq_labels.insert(lbl, num_str.clone());
                }

                let clean_line = re_label.replace_all(line, "").to_string();
                let trimmed = clean_line.trim_end();
                new_lines.push(format!(r"{} \qquad ({})", trimmed, num_str));
            } else {
                let clean_line = re_label.replace_all(line, "").to_string();
                new_lines.push(clean_line);
            }
        }

        let new_body = new_lines.join(r"\\");
        format!(r"\begin{{{}{}}}{}\end{{{}{}}}", env_name, env_star, new_body, env_name, env_star)
    }
}

fn detect_document_has_chapters(content: &str) -> bool {
    if content.is_empty() {
        return false;
    }
    let has_tex_chap = content.contains(r"\chapter") 
        || content.contains(r"\documentclass{report}") 
        || content.contains(r"\documentclass{book}")
        || content.contains(r"\documentclass{scrreprt}")
        || content.contains(r"\documentclass{scrbook}");
    
    let re_h1 = match Regex::new(r"(?m)^#\s+") { Ok(r) => r, Err(_) => return has_tex_chap };
    let h1_count = re_h1.find_iter(content).count();
    let re_h2_chap = match Regex::new(r"(?m)^##\s*Chapter\b") { Ok(r) => r, Err(_) => return has_tex_chap };
    let re_yaml = match Regex::new(r"(?m)^top-level-division:\s*chapter") { Ok(r) => r, Err(_) => return has_tex_chap };
    
    has_tex_chap || re_h2_chap.is_match(content) || re_yaml.is_match(content) || (h1_count >= 2)
}

pub fn resolve_equation_references(content: &str) -> String {
    if content.is_empty() {
        return content.to_string();
    }

    let re_env = match Regex::new(r"(?s)\\begin\{(equation|align|gather|eqnarray|multline)(\*?)\}(.*?)\\end\{(?:equation|align|gather|eqnarray|multline)(?:\*)?\}") {
        Ok(re) => re,
        Err(_) => return content.to_string(),
    };
    
    let re_label = match Regex::new(r"\\label\{([^}]+)\}") {
        Ok(re) => re,
        Err(_) => return content.to_string(),
    };

    let has_numberwithin_chapter = content.contains(r"\numberwithin{equation}{chapter}") || content.contains(r"\numberwithin*{equation}{chapter}");
    let has_numberwithin_section = content.contains(r"\numberwithin{equation}{section}") || content.contains(r"\numberwithin*{equation}{section}");

    let has_chapter_cmd = detect_document_has_chapters(content);
    let re_sec_detect = Regex::new(r"(?:\b|\\)section\b").unwrap();
    let re_md_sec_detect = Regex::new(r"(?m)^##\s+").unwrap();

    let has_section_cmd = (re_sec_detect.is_match(content) && !content.contains(r"\subsection")) || re_md_sec_detect.is_match(content);

    let scheme = if has_numberwithin_chapter {
        EqScheme::Chapter
    } else if has_numberwithin_section {
        if has_chapter_cmd {
            EqScheme::SectionWithChapter
        } else {
            EqScheme::Section
        }
    } else if has_chapter_cmd {
        EqScheme::Chapter
    } else if has_section_cmd {
        EqScheme::Section
    } else {
        EqScheme::Global
    };

    let mut chapter_counter = 0;
    let mut section_counter = 0;
    let mut eq_counter = 0;
    let mut eq_labels = std::collections::HashMap::new();

    let mut last_idx = 0;
    for caps in re_env.captures_iter(content) {
        let mat = match caps.get(0) {
            Some(m) => m,
            None => continue,
        };

        let text_slice = &content[last_idx..mat.start()];
        let h_events = scan_heading_events(text_slice);
        for ev in h_events {
            update_heading_counters(&ev, scheme, &mut chapter_counter, &mut section_counter, &mut eq_counter);
        }

        process_env(&caps, scheme, chapter_counter, section_counter, &mut eq_counter, &mut eq_labels);
        last_idx = mat.end();
    }
    
    let mut result = String::new();
    last_idx = 0;
    chapter_counter = 0;
    section_counter = 0;
    eq_counter = 0;
    let mut dummy_labels = std::collections::HashMap::new();

    for caps in re_env.captures_iter(content) {
        let mat = match caps.get(0) {
            Some(m) => m,
            None => continue,
        };

        let text_slice = &content[last_idx..mat.start()];
        let h_events = scan_heading_events(text_slice);
        for ev in h_events {
            update_heading_counters(&ev, scheme, &mut chapter_counter, &mut section_counter, &mut eq_counter);
        }
        result.push_str(text_slice);

        let new_env_str = process_env(&caps, scheme, chapter_counter, section_counter, &mut eq_counter, &mut dummy_labels);
        result.push_str(&new_env_str);
        last_idx = mat.end();
    }
    result.push_str(&content[last_idx..]);

    let mut other_labels = std::collections::HashMap::new();
    let mut tab_counter = 0;
    let mut fig_counter = 0;

    for caps in re_label.captures_iter(&result) {
        if let Some(lbl_cap) = caps.get(1) {
            let label = lbl_cap.as_str().trim().to_string();
            if label.starts_with("tab:") && !other_labels.contains_key(&label) {
                tab_counter += 1;
                other_labels.insert(label, tab_counter);
            } else if label.starts_with("fig:") && !other_labels.contains_key(&label) {
                fig_counter += 1;
                other_labels.insert(label, fig_counter);
            }
        }
    }
    
    let mut final_content = result;

    for (label, num_str) in &eq_labels {
        let escaped = regex::escape(label);
        let num_formatted = format!("({})", num_str);

        if let Ok(re_eq) = Regex::new(&format!(r"\\eqref\{{\s*{}\s*\}}", escaped)) {
            final_content = re_eq.replace_all(&final_content, regex::NoExpand(&num_formatted)).to_string();
        }
        if let Ok(re_ref) = Regex::new(&format!(r"\\ref\{{\s*{}\s*\}}", escaped)) {
            final_content = re_ref.replace_all(&final_content, regex::NoExpand(num_str)).to_string();
        }
    }

    for (label, num) in &other_labels {
        let escaped = regex::escape(label);
        let num_plain = format!("{}", num);
        if let Ok(re_ref) = Regex::new(&format!(r"\\ref\{{\s*{}\s*\}}", escaped)) {
            final_content = re_ref.replace_all(&final_content, regex::NoExpand(&num_plain)).to_string();
        }
    }

    if let Ok(re_tabularx_start) = Regex::new(r"\\begin\{tabularx\}\{[^}]*\}\{([^}]*)\}") {
        final_content = re_tabularx_start.replace_all(&final_content, |caps: &regex::Captures| {
            let spec = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let spec_no_at = Regex::new(r"@\{[^}]*\}").unwrap().replace_all(spec, "");
            let spec_l = Regex::new(r"p\{[^}]*\}").unwrap().replace_all(&spec_no_at, "l");
            let spec_clean = Regex::new(r"[YX]").unwrap().replace_all(&spec_l, "l");
            let cols: Vec<&str> = spec_clean.split_whitespace().collect();
            let final_cols = if cols.is_empty() { "l l l".to_string() } else { cols.join(" ") };
            format!(r"\begin{{tabular}}{{{}}}", final_cols)
        }).to_string();
    }
    if let Ok(re_tabularx_end) = Regex::new(r"\\end\{tabularx\}") {
        final_content = re_tabularx_end.replace_all(&final_content, r"\end{tabular}").to_string();
    }
    
    final_content
}

#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ManuscriptIssue {
    line: usize,
    level: String,
    message: String,
    snippet: String,
}

fn sanitize_math_environments(content: &str) -> String {
    let mut result = content.to_string();
    let math_envs = r"(equation|align|gather|aligned|eqnarray|multline)";

    // 1. Extract long explanatory prose \text{...} out of equation environments so Pandoc never fails math parsing
    if let Ok(re_env) = Regex::new(&format!(r"(?s)\\begin\{{{}}}(\*?)\}}(.*?)\\end\{{{}}}(\*?)\}}", math_envs, math_envs)) {
        if let Ok(re_text) = Regex::new(r"(?:\\quad|\\qquad|,|;|\s)*\\text\{\s*([^}]+)\s*\}") {
            let mut out = String::new();
            let mut last_idx = 0;

            for caps in re_env.captures_iter(&result) {
                let mat = match caps.get(0) {
                    Some(m) => m,
                    None => continue,
                };
                out.push_str(&result[last_idx..mat.start()]);

                let env_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let env_star = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let env_body = caps.get(3).map(|m| m.as_str()).unwrap_or("");

                if env_body.contains(r"\begin{aligned}") || env_body.contains(r"\begin{array}") || env_body.contains(r"\begin{matrix}") {
                    out.push_str(mat.as_str());
                    last_idx = mat.end();
                    continue;
                }

                if let Some(text_caps) = re_text.captures(env_body) {
                    if let Some(txt_match) = text_caps.get(1) {
                        let prose = txt_match.as_str().trim();
                        if !prose.starts_with("\\cite") && !prose.starts_with("~\\cite") && prose.len() > 35 && prose.contains(' ') {
                            let clean_body = re_text.replace(env_body, "").to_string();
                            out.push_str(&format!("\\begin{{{}}}{}\\end{{{}}}\n\n{}\n", format_args!("{}{}", env_name, env_star), clean_body, format_args!("{}{}", env_name, env_star), prose));
                            last_idx = mat.end();
                            continue;
                        }
                    }
                }

                out.push_str(mat.as_str());
                last_idx = mat.end();
            }
            out.push_str(&result[last_idx..]);
            result = out;
        }
    }

    // 2. Strip $$ wrappers around equation environments
    if let Ok(re) = Regex::new(&format!(r"(?s)\$\$\s*\\begin\{{{}}}(\*?)\}}", math_envs)) {
        result = re.replace_all(&result, r"\begin{$1$2}").to_string();
    }
    if let Ok(re) = Regex::new(&format!(r"(?s)\\end\{{{}}}(\*?)\}}\s*\$\$", math_envs)) {
        result = re.replace_all(&result, r"\end{$1$2}").to_string();
    }
    if let Ok(re) = Regex::new(&format!(r"(?s)\\\[\s*\\begin\{{{}}}(\*?)\}}", math_envs)) {
        result = re.replace_all(&result, r"\begin{$1$2}").to_string();
    }
    if let Ok(re) = Regex::new(&format!(r"(?s)\\end\{{{}}}(\*?)\}}\s*\\\]", math_envs)) {
        result = re.replace_all(&result, r"\end{$1$2}").to_string();
    }
    if let Ok(re) = Regex::new(r"\$\$\s*\\\[") {
        result = re.replace_all(&result, r"\\[").to_string();
    }
    if let Ok(re) = Regex::new(r"\\\]\s*\$\$") {
        result = re.replace_all(&result, r"\]").to_string();
    }

    result
}

#[allow(dead_code)]
#[tauri::command]
fn validate_manuscript(path: String) -> Result<Vec<ManuscriptIssue>, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let mut issues = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let re_nested_start = Regex::new(r"\$\$\s*\\begin\{(equation|align|gather|aligned|eqnarray|multline)").unwrap();
    let re_nested_end = Regex::new(r"\\end\{(equation|align|gather|aligned|eqnarray|multline)(\*?)\}\s*\$\$").unwrap();
    let re_dollar_bracket = Regex::new(r"\$\$\s*\\\[|\\\]\s*\$\$").unwrap();
    let re_bracket_env = Regex::new(r"\\\[\s*\\begin\{(equation|align|gather|aligned|eqnarray|multline)").unwrap();
    let re_env_bracket = Regex::new(r"\\end\{(equation|align|gather|aligned|eqnarray|multline)(\*?)\}\s*\\\]").unwrap();
    let re_text = Regex::new(r"\\text\{\s*([^}]+)\s*\}").unwrap();

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let stripped = line.trim();

        if re_nested_start.is_match(line) {
            issues.push(ManuscriptIssue {
                line: line_num,
                level: "ERROR".to_string(),
                message: "Invalid nested math mode: Found '$$\\begin{...}'. Display math delimiters ($$ or \\[ \\]) must NEVER wrap LaTeX equation environments (\\begin{equation}). Use one method only.".to_string(),
                snippet: stripped.to_string(),
            });
        }
        if re_nested_end.is_match(line) {
            issues.push(ManuscriptIssue {
                line: line_num,
                level: "ERROR".to_string(),
                message: "Invalid nested math mode: Found '\\end{...}$$'. Remove redundant trailing '$$' delimiters.".to_string(),
                snippet: stripped.to_string(),
            });
        }
        if re_dollar_bracket.is_match(line) {
            issues.push(ManuscriptIssue {
                line: line_num,
                level: "ERROR".to_string(),
                message: "Invalid nested math mode: Found '$$\\[' or '\\]$$'. Do not combine '$$' delimiters with '\\[ \\]' brackets.".to_string(),
                snippet: stripped.to_string(),
            });
        }
        if re_bracket_env.is_match(line) {
            issues.push(ManuscriptIssue {
                line: line_num,
                level: "ERROR".to_string(),
                message: "Invalid nested math mode: Found '\\[\\begin{...}'. Display math brackets '\\[ \\]' must not wrap equation environments.".to_string(),
                snippet: stripped.to_string(),
            });
        }
        if re_env_bracket.is_match(line) {
            issues.push(ManuscriptIssue {
                line: line_num,
                level: "ERROR".to_string(),
                message: "Invalid nested math mode: Found '\\end{...}\\]'. Remove redundant trailing '\\]' brackets.".to_string(),
                snippet: stripped.to_string(),
            });
        }

        for caps in re_text.captures_iter(line) {
            if let Some(txt_match) = caps.get(1) {
                let txt = txt_match.as_str();
                if !txt.starts_with("\\cite") && !txt.starts_with("~\\cite") && txt.len() > 25 && txt.contains(' ') {
                    let end_idx = std::cmp::min(txt.len(), 40);
                    issues.push(ManuscriptIssue {
                        line: line_num,
                        level: "WARNING".to_string(),
                        message: format!("Compatibility Warning: Long explanatory prose found inside \\text{{...}} in equation environment ('{}...'). For best DOCX/Pandoc compatibility, move explanatory prose out of the equation into standard paragraph text.", &txt[..end_idx]),
                        snippet: stripped.to_string(),
                    });
                }
            }
        }
    }

    Ok(issues)
}

#[tauri::command]
fn export_docx(path: String, mut bib_file: String, mut use_citeproc: bool) -> Result<String, String> {
    let path_env = get_env_path();
    let file_path = Path::new(&path);
    let cwd = file_path.parent().unwrap_or_else(|| Path::new("."));
    let output_file = file_path.with_extension("export.docx");

    let is_tex = path.to_lowercase().ends_with(".tex");
    let from_fmt = if is_tex { "latex" } else { "markdown" };

    let temp_ref = std::env::temp_dir().join("temp_reference.docx");
    let temp_lua = std::env::temp_dir().join("temp_pagebreak.lua");
    let _ = fs::write(&temp_ref, CUSTOM_REFERENCE_DOCX);

    let lua_filter = r#"
-- Preserve author-requested page breaks, but do not emit one where Word will
-- already start a top-level/chapter heading on a new page.  Consecutive and
-- trailing breaks are also collapsed so they cannot create blank pages.
local function is_page_break(block)
  return block.t == "Div"
    and (block.classes:includes("newpage") or block.classes:includes("clearpage"))
end

function Pandoc(doc)
  local blocks = pandoc.List()
  for i, block in ipairs(doc.blocks) do
    if is_page_break(block) then
      local next_block = doc.blocks[i + 1]
      local previous_is_break = #blocks > 0
        and blocks[#blocks].t == "RawBlock"
        and blocks[#blocks].format == "openxml"
        and blocks[#blocks].text:match('w:type="page"')
      local next_starts_page = next_block
        and next_block.t == "Header"
        and next_block.level == 1

      if not previous_is_break and not next_starts_page and next_block then
        blocks:insert(pandoc.RawBlock(
          "openxml",
          '<w:p><w:r><w:br w:type="page"/></w:r></w:p>'
        ))
      end
    else
      blocks:insert(block)
    end
  end
  return pandoc.Pandoc(blocks, doc.meta)
end

-- Starred/unnumbered level-three headings are body labels, not structural TOC
-- entries.  Keep their heading appearance but write them with a non-outline
-- custom style so Word's dynamic TOC does not collect them.  Unnumbered
-- front-matter headings at levels one and two remain eligible for the TOC.
function Header(el)
  if el.level == 3 and el.classes:includes("unnumbered") then
    return pandoc.Div(
      {pandoc.Para(el.content)},
      pandoc.Attr(
        el.identifier,
        {},
        {["custom-style"] = "Unlisted Heading 3"}
      )
    )
  end
end
"#;
    let _ = fs::write(&temp_lua, lua_filter);

    let mut final_input_path = path.clone();

    let mut content_opt: Option<String> = None;

    if is_tex {
        let lp_out = Command::new("latexpand")
            .env("PATH", &path_env)
            .current_dir(cwd)
            .arg(&path)
            .output();

        if let Ok(out) = lp_out {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    if !s.is_empty() {
                        content_opt = Some(s);
                    }
                }
            }
        }

        if content_opt.is_none() {
            let mut visited = std::collections::HashSet::new();
            if let Ok(c) = flatten_latex(file_path, cwd, &mut visited) {
                content_opt = Some(c);
            }
        }
    } else {
        if let Ok(c) = fs::read_to_string(&path) {
            content_opt = Some(c);
        }
    }

    let mut has_chapters = false;

    if let Some(mut content) = content_opt {
        has_chapters = detect_document_has_chapters(&content);
        let compat_macros = "\\providecommand{\\vect}[1]{\\mathbf{#1}}\n\\providecommand{\\symbf}[1]{\\mathbf{#1}}\n\\providecommand{\\jump}[1]{[#1]}\n\\providecommand{\\abs}[1]{|#1|}\n\\providecommand{\\norm}[1]{\\|#1\\|}\n\\providecommand{\\eval}[1]{#1}\n\n";
        content = format!("{}{}", compat_macros, content);

        // ALWAYS sanitize math environments for both .tex and .md
        content = sanitize_math_environments(&content);

        if is_tex {
            if bib_file.is_empty() {
                if let Ok(re_bib) = Regex::new(r"\\(?:addbibresource|bibliography)\{([^}]+)\}") {
                    if let Some(caps) = re_bib.captures(&content) {
                        if let Some(cap) = caps.get(1) {
                            let mut detected = cap.as_str().to_string();
                            if !detected.ends_with(".bib") && !detected.contains('.') {
                                detected.push_str(".bib");
                            }
                            let resolved_bib = cwd.join(&detected);
                            if resolved_bib.exists() {
                                bib_file = resolved_bib.to_string_lossy().to_string();
                                use_citeproc = true;
                            }
                        }
                    }
                }
            }

            if let Ok(re_newpage) = Regex::new(r"\\newpage\b") {
                content = re_newpage.replace_all(&content, "\\begin{newpage}\\end{newpage}").to_string();
            }

            if let Ok(re_clearpage) = Regex::new(r"\\clearpage\b") {
                content = re_clearpage.replace_all(&content, "\\begin{clearpage}\\end{clearpage}").to_string();
            }
        }

        content = resolve_equation_references(&content);
        content = sanitize_math_environments(&content);

        let temp_ext = if is_tex { "tex" } else { "md" };
        let temp_input = std::env::temp_dir().join(format!("temp_input_docx.{}", temp_ext));
        if let Err(e) = fs::write(&temp_input, &content) {
            return Err(format!("Failed to write preprocessed input file: {}", e));
        }
        final_input_path = temp_input.to_string_lossy().to_string();
    }

    let mut args = vec![
        final_input_path.clone(),
        "--from".to_string(), from_fmt.to_string(),
        "--to".to_string(), "docx".to_string(),
        "--table-of-contents".to_string(),
        "--number-sections".to_string(),
        "-V".to_string(), "reference-section-title=References".to_string(),
        "--metadata".to_string(), "reference-section-title=References".to_string(),
    ];

    if has_chapters {
        args.push("--top-level-division=chapter".to_string());
    }

    args.extend(vec![
        "--reference-doc".to_string(), temp_ref.to_string_lossy().to_string(),
        "--lua-filter".to_string(), temp_lua.to_string_lossy().to_string(),
        "-o".to_string(), output_file.to_string_lossy().to_string(),
    ]);
    
    if use_citeproc || !bib_file.is_empty() {
        args.push("--citeproc".to_string());
    }
    if !bib_file.is_empty() {
        args.push("--bibliography".to_string());
        args.push(bib_file.clone());
    }
    
    let mut logs = format!("Command: pandoc {}\n\n", args.join(" "));
    
    let output = Command::new("pandoc")
        .env("PATH", &path_env)
        .current_dir(cwd)
        .args(&args)
        .output();
        
    let _ = fs::remove_file(&temp_ref);
    let _ = fs::remove_file(&temp_lua);
    if final_input_path != path {
        let _ = fs::remove_file(&final_input_path);
    }
    
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stdout.is_empty() {
                logs.push_str(&stdout);
                logs.push('\n');
            }
            if out.status.success() {
                logs.push_str("SUCCESS: Word document generated successfully.\n");
                logs.push_str(&format!("Output: {}\n", output_file.display()));
                Ok(logs)
            } else {
                logs.push_str(&format!("ERROR:\n{}\n", stderr));
                Err(logs)
            }
        }
        Err(e) => {
            logs.push_str(&format!("Failed to execute pandoc: {}\n", e));
            Err(logs)
        }
    }
}

#[tauri::command]
fn select_save_file(app: tauri::AppHandle, file_type: String) -> Option<String> {
    use std::sync::{Arc, Mutex};
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    
    let _ = app.run_on_main_thread(move || {
        let mut dialog = rfd::FileDialog::new();
        if file_type == "tex" {
            dialog = dialog.add_filter("LaTeX Document (*.tex)", &["tex"]);
        } else if file_type == "md" {
            dialog = dialog.add_filter("Markdown Document (*.md)", &["md"]);
        } else if file_type == "txt" {
            dialog = dialog.add_filter("Text Document (*.txt)", &["txt"]);
        } else {
            dialog = dialog.add_filter("Documents", &["md", "tex", "txt"]);
        }
        let mut path = dialog.save_file();
        if let Some(ref mut p) = path {
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
            if ext.is_empty() {
                p.set_extension(&file_type);
            } else {
                let file_name = p.file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let file_name_lower = file_name.to_lowercase();
                
                if file_type == "tex" {
                    if file_name_lower.ends_with(".tex.tex") || file_name_lower.ends_with(".md.tex") || file_name_lower.ends_with(".txt.tex") {
                        let new_name = &file_name[..file_name.len() - 4];
                        p.set_file_name(new_name);
                    }
                } else if file_type == "md" {
                    if file_name_lower.ends_with(".md.md") || file_name_lower.ends_with(".tex.md") || file_name_lower.ends_with(".txt.md") {
                        let new_name = &file_name[..file_name.len() - 3];
                        p.set_file_name(new_name);
                    }
                } else if file_type == "txt" {
                    if file_name_lower.ends_with(".txt.txt") || file_name_lower.ends_with(".md.txt") || file_name_lower.ends_with(".tex.txt") {
                        let new_name = &file_name[..file_name.len() - 4];
                        p.set_file_name(new_name);
                    }
                }
            }
            *result_clone.lock().unwrap() = Some(p.to_string_lossy().to_string());
        }
        let _ = tx.send(());
    });
    let _ = rx.recv();
    let val = result.lock().unwrap().clone();
    val
}

#[derive(serde::Serialize)]
struct DependencyStatus {
    has_pandoc: bool,
    has_cspell: bool,
    has_lualatex: bool,
    has_xelatex: bool,
    has_pdflatex: bool,
    has_node: bool,
}

fn is_command_available(cmd: &str) -> bool {
    let path_env = get_env_path();
    for path_dir in path_env.split(':') {
        let p = std::path::Path::new(path_dir).join(cmd);
        if p.exists() && p.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if let Ok(meta) = p.metadata() {
                    if meta.mode() & 0o111 != 0 {
                        return true;
                    }
                }
            }
            #[cfg(not(unix))]
            return true;
        }
    }
    false
}

#[tauri::command]
fn check_dependencies() -> DependencyStatus {
    DependencyStatus {
        has_pandoc: is_command_available("pandoc"),
        has_cspell: is_command_available("cspell"),
        has_lualatex: is_command_available("lualatex"),
        has_xelatex: is_command_available("xelatex"),
        has_pdflatex: is_command_available("pdflatex"),
        has_node: is_command_available("node") || is_command_available("npm"),
    }
}

#[tauri::command]
fn clean_aux_files(path: String) -> Result<(), String> {
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        if let Some(stem) = file_path.file_stem() {
            let stem_str = stem.to_string_lossy();
            let extensions = ["aux", "toc", "out", "log"];
            for ext in &extensions {
                let aux_file = parent.join(format!("{}.{}", stem_str, ext));
                if aux_file.exists() {
                    let _ = fs::remove_file(aux_file);
                }
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    let path_env = get_env_path();
    Command::new("open")
        .env("PATH", &path_env)
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            select_file,
            select_save_file,
            read_file,
            write_file,
            run_spell_check,
            fetch_suggestions,
            add_to_dictionary,
            render_latex_preview,
            compile_pdf,
            export_html,
            export_latex,
            export_docx,
            validate_manuscript,
            open_file,
            clean_aux_files,
            check_dependencies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_equation_numbering() {
        let input = r#"
\chapter{First Chapter}
\begin{equation}
E = mc^2 \label{eq:einstein}
\end{equation}

\chapter{Second Chapter}
\begin{equation}
F = ma \label{eq:newton}
\end{equation}

As seen in \eqref{eq:einstein} and \eqref{eq:newton}.
"#;
        let res = resolve_equation_references(input);
        assert!(res.contains(r"E = mc^2 \qquad (1.1)"));
        assert!(res.contains(r"F = ma \qquad (2.1)"));
        assert!(res.contains("As seen in (1.1) and (2.1)."));
    }

    #[test]
    fn test_section_equation_numbering() {
        let input = r#"
\section{Introduction}
\begin{equation}
a = b \label{eq:one}
\end{equation}

\section{Methods}
\begin{equation}
c = d \label{eq:two}
\end{equation}

Ref: \ref{eq:one} and \eqref{eq:two}.
"#;
        let res = resolve_equation_references(input);
        assert!(res.contains(r"a = b \qquad (1.1)"));
        assert!(res.contains(r"c = d \qquad (2.1)"));
        assert!(res.contains("Ref: 1.1 and (2.1)."));
    }

    #[test]
    fn test_numberwithin_section_with_chapter() {
        let input = r#"
\numberwithin{equation}{section}
\chapter{Main Chapter}
\section{Sub Section}
\begin{equation}
x = y \label{eq:sub}
\end{equation}

See \eqref{eq:sub}.
"#;
        let res = resolve_equation_references(input);
        assert!(res.contains(r"x = y \qquad (1.1.1)"));
        assert!(res.contains("See (1.1.1)."));
    }

    #[test]
    fn test_global_equation_numbering() {
        let input = r#"
\begin{equation}
x = 1 \label{eq:g1}
\end{equation}
\begin{equation}
y = 2 \label{eq:g2}
\end{equation}
"#;
        let res = resolve_equation_references(input);
        assert!(res.contains(r"x = 1 \qquad (1)"));
        assert!(res.contains(r"y = 2 \qquad (2)"));
    }

    #[test]
    fn test_starred_equation_is_not_numbered_but_labelled_one_is() {
        let input = r#"
\begin{equation*}
x = 1
\end{equation*}
\begin{equation*}
y = 2 \label{eq:labelled-star}
\end{equation*}
See \eqref{eq:labelled-star}.
"#;
        let res = resolve_equation_references(input);
        assert!(res.contains("\\begin{equation*}\nx = 1\n\\end{equation*}"));
        assert!(res.contains(r"y = 2 \qquad (1)"));
        assert!(res.contains("See (1)."));
    }

    #[test]
    fn test_file_line_error_parser_uses_real_source_line() {
        let source = "\\documentclass{article}\n\\begin{document}\n\\badcommand{value}\n\\end{document}\n";
        let log = "main.tex:3: Undefined control sequence.\nl.3 \\badcommand{value}\n";
        let errors = parse_compiler_errors(log, source, false);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
        assert!(errors[0].context.contains(r"\badcommand"));
        assert!(errors[0].suggestion.contains(r"\badcommand"));
    }

    #[test]
    fn test_classic_tex_error_parser_collects_context() {
        let source = "Text\n\\begin{equation}\nx = \\notacommand{y}\n\\end{equation}\n";
        let log = "! Undefined control sequence.\n<recently read> \\notacommand\nl.3 x = \\notacommand{y}\n";
        let errors = parse_compiler_errors(log, source, false);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
        assert_eq!(errors[0].context, r"x = \notacommand{y}");
    }

    #[test]
    fn test_generated_tex_error_maps_back_to_markdown_command() {
        let source = "# Heading\n\nA formula with \\uniqueinvalidcommand{x}.\n";
        let log = "! Undefined control sequence.\nl.87 ...formula with \\uniqueinvalidcommand{x}.\n";
        let errors = parse_compiler_errors(log, source, true);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
    }

    #[test]
    fn test_pandoc_duplicate_error_is_deduplicated_after_remap() {
        let source = "# Heading\n\nText with \\onlybadcommand{x}.\n";
        let log = "/tmp/generated.tex:87: Undefined control sequence.\n! Undefined control sequence.\nl.87 ...with \\onlybadcommand{x}.\n";
        let errors = parse_compiler_errors(log, source, true);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
    }

    #[test]
    fn test_missing_item_gets_specific_list_fix() {
        let suggestion = generate_suggestion(
            "LaTeX Error: Something's wrong--perhaps a missing \\item.",
            r"\begin{itemize}",
        );
        assert!(suggestion.contains(r"\item"));
        assert!(suggestion.contains("list"));
        assert!(!suggestion.starts_with("Inspect the highlighted"));
    }

    #[test]
    fn test_wrapped_latex_error_preserves_detail() {
        let source = "\\begin{document}\ntext\n\\end{document}\n";
        let log = "main.tex:2: LaTeX Error:\nSomething's wrong--perhaps a missing \\item.\nl.2 text\n";
        let errors = parse_compiler_errors(log, source, false);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("missing \\item"));
        assert!(errors[0].suggestion.contains("list"));
    }

    #[test]
    fn test_unknown_error_fallback_quotes_real_message() {
        let suggestion = generate_suggestion("A very unusual compiler failure", "source");
        assert!(suggestion.contains("A very unusual compiler failure"));
        assert!(!suggestion.starts_with("Inspect the highlighted"));
    }

    #[test]
    fn test_wrapped_latex_error_word_is_reassembled() {
        let source = "Intro\nLet \\mathcal{A} denote the algebra.\n";
        let log = "book.tex:2: LaTeX Er\nror: \\mathcal allowed only in math mode.\nl.2 Let \\mathcal{A} denote the algebra.\n";
        let errors = parse_compiler_errors(log, source, false);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("LaTeX Error:"));
        assert!(errors[0].message.contains("allowed only in math mode"));
        assert!(errors[0].suggestion.contains(r"$\mathcal{A}$"));
    }

    #[test]
    fn test_mathcal_text_mode_error_gets_concrete_fix() {
        let suggestion = generate_suggestion(
            r"LaTeX Error: \mathcal allowed only in math mode.",
            r"Let \mathcal{A} denote the algebra.",
        );
        assert!(suggestion.contains(r"$\mathcal{A}$"));
        assert!(suggestion.contains("math-only"));
    }

    #[test]
    fn test_generated_preamble_error_does_not_highlight_unrelated_prose() {
        let source = "# Chapter\n\nLet $\\mathcal{A}$ denote the algebra.\n";
        let log = "/tmp/generated.tex:41: LaTeX Error: Command `\\eth' already defined.\nl.41 \\newcommand{\\eth}{x}\n";
        let errors = parse_compiler_errors(log, source, true);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 1);
        assert!(errors[0].context.contains("Generated LaTeX preamble"));
        assert!(errors[0].suggestion.contains("amssymb"));
        assert!(!errors[0].context.contains(r"\mathcal"));
    }

    #[test]
    fn test_symbol_package_errors_are_classified_and_consolidated() {
        let source = "Let $\\mathcal{A}$ denote the algebra.\n";
        let log = concat!(
            "/texmf/amssymb.sty:240: LaTeX Error: Command `\\eth' already defined.\n",
            "/texmf/amssymb.sty:251: LaTeX Error: Command `\\smallsetminus' already defined.\n",
            "/texmf/amssymb.sty:259: LaTeX Error: Command `\\digamma' already defined.\n",
            "/texmf/amssymb.sty:265: LaTeX Error: Command `\\backepsilon' already defined.\n",
        );
        let errors = parse_compiler_errors(log, source, true);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 1);
        assert!(errors[0].message.contains("Symbol package conflict"));
        assert!(errors[0].suggestion.contains("amssymb"));
        assert!(errors[0].suggestion.contains("before unicode-math"));
        assert!(!errors[0].context.contains(r"\mathcal"));
    }
}
