use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // Read the current directory
    let current_dir = env::current_dir()?;
    let mut html = String::new();
    html.push_str(r#"<style>li {display: flex; align-items: center;} input[type="text"] {margin-left: 8px;}</style>"#);
    html.push_str(r#"<script>
        window.addEventListener('load', () => {
            const data = JSON.parse(localStorage.getItem('fileTreeState')) || {};
            for (const [key, value] of Object.entries(data)) {
                const el = document.querySelector(`[data-path="${key}"]`);
                if (el) {
                    el.querySelector('input[type="checkbox"]').checked = value.checked;
                    el.querySelector('input[type="text"]').value = value.comment;
                    el.style.backgroundColor = value.checked ? 'red' : '';
                }
            }
        });

        function saveState() {
            const data = {};
            document.querySelectorAll('[data-path]').forEach(el => {
                const path = el.getAttribute('data-path');
                const checked = el.querySelector('input[type="checkbox"]').checked;
                const comment = el.querySelector('input[type="text"]').value;
                data[path] = { checked, comment };
            });
            localStorage.setItem('fileTreeState', JSON.stringify(data));
        }
    </script>"#);
    html.push_str("<ul>\n");
    walk_directory(&current_dir, &mut html)?;
    html.push_str("</ul>\n");
    html.push_str(r#"<button onclick="saveState()">Save</button>"#);

    fs::write("file_tree.html", html)?;

    Ok(())
}

fn walk_directory(dir: &PathBuf, html: &mut String) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        let path_str = path.to_string_lossy().into_owned();
        html.push_str(&format!(
            r#"<li data-path="{}"><input type="checkbox" onchange="this.parentNode.style.backgroundColor=this.checked?'red':''">{}<input type="text"></li>"#,
            path_str,
            path.file_name().unwrap().to_string_lossy()
        ));

        if metadata.is_dir() {
            html.push_str("<ul>\n");
            walk_directory(&path, html)?;
            html.push_str("</ul>\n");
        }
    }

    Ok(())
}

