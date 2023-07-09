/// Concatenate two paths, even if the right argument is an absolute path.
fn join_absolute<A: AsRef<Path>, B: AsRef<Path>>(left: A, right: B) -> PathBuf {
    let (left, right) = (left.as_ref(), right.as_ref());
    left.join(
        right.strip_prefix("/").map_or(right, |stripped| stripped)
    )
}

fn symlink_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::symlink_metadata(path).is_ok()
}

fn get_user() -> String {
    env::var("USER").unwrap_or_else(|_| String::from("unknown"))
}

/// Prompt for user input, returning True if the first character is 'y' or 'Y'
fn prompt_yes<T: AsRef<str>>(prompt: T) -> bool {
    print!("{} (y/N) ", prompt.as_ref());
    if io::stdout().flush().is_err() {
        // If stdout wasn't flushed properly, fallback to println
        println!("{} (y/N)", prompt.as_ref());
    }
    let stdin = BufReader::new(io::stdin());
    stdin.bytes().next()
        .and_then(std::result::Result::ok)
        .map(|c| c as char)
        .map_or(false, |c| (c == 'y' || c == 'Y'))
}

/// Add a numbered extension to duplicate filenames to avoid overwriting files.
fn rename_grave<G: AsRef<Path>>(grave: G) -> PathBuf {
    let grave = grave.as_ref();
    let name = grave.to_str().expect("Filename must be valid unicode.");
    (1..)
        .map(|i| PathBuf::from(format!("{name}~{i}")))
        .find(|p| !symlink_exists(p))
        .expect("Failed to rename duplicate file or directory")
}

#[allow(clippy::cast_possible_truncation)]
fn humanize_bytes(bytes: u64) -> String {
    let values = ["bytes", "KB", "MB", "GB", "TB"];
    let pair = values.iter()
        .enumerate()
        .take_while(|x| bytes as usize / 1000_usize.pow(x.0 as u32) > 10)
        .last();
    if let Some((i, unit)) = pair {
        format!("{} {unit}", bytes as usize / 1000_usize.pow(i as u32))
    } else {
        format!("{bytes} {}", values[0])
    }
}
