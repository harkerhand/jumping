enum Shell {
    Bash,
    Zsh,
    Fish,
    Other,
}

pub fn init() {
    let shell = detect_shell();
    match shell {
        Shell::Fish => println!("{}", fish_init()),
        _ => println!("{}", bash_init()),
    }
}

fn detect_shell() -> Shell {
    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("bash") {
            Shell::Bash
        } else if shell.contains("zsh") {
            Shell::Zsh
        } else if shell.contains("fish") {
            Shell::Fish
        } else {
            Shell::Other
        }
    } else {
        Shell::Other
    }
}

pub(crate) fn get_tmp_file_path() -> std::path::PathBuf {
    let uid = unsafe { libc::getuid() };
    std::env::temp_dir().join(format!("jumping-{}", uid))
}

fn fish_init() -> String {
    let path = get_tmp_file_path().to_string_lossy().into_owned();
    format!(
        r#"function jp
    set -l result_file {path}
    if test -f $result_file
        rm $result_file
    end
    jumping
    if test -f $result_file
        set -l dest (cat $result_file)
        if test -d $dest
            cd $dest
        end
        rm $result_file
    end
end"#
    )
}

fn bash_init() -> String {
    let path = get_tmp_file_path().to_string_lossy().into_owned();

    format!(
        r#"jp() {{
    local TMP_FILE="{path}"
    [ -f "$TMP_FILE" ] && rm "$TMP_FILE"

    jumping

    if [ -f "$TMP_FILE" ]; then
        local DEST=$(cat "$TMP_FILE")
        if [ -d "$DEST" ]; then
            cd "$DEST"
            pwd
        fi
        rm "$TMP_FILE"
    fi
}}"#
    )
}
