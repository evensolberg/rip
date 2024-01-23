# RIP (Rm ImProved)

`rip` is a command-line deletion tool focused on safety and ergonomics.

Deleted files get sent to the graveyard (`/tmp/graveyard-$USER` or the trashcan on a Mac by default) under their absolute path, giving you a chance to recover them.  No data is overwritten.  If files that share the same path are deleted, they will be renamed as numbered backups.

`rip` is made for lazy people.  If any part of the interface could be more intuitive, please open an issue or pull request.

## Usage

```console
Usage: rip [OPTIONS] [TARGET]...

Arguments:
  [TARGET]...  File or directory to remove

Options:
  -g, --graveyard <graveyard>  Directory where deleted files go to rest
  -d, --decompose              Permanently deletes (unlink) the entire graveyard
  -s, --seance                 Prints files that were sent under the current directory
  -u, --unbury <target>...     Undo the last removal by the current user, or specify some file(s) in the graveyard.  Combine with -s to restore everything printed by -s.
  -y, --yes-to-all             Auto-confirm any prompts.
  -i, --inspect                Prints some info about TARGET before prompting for action.
  -h, --help                   Print help
  -V, --version                Print version
```

### Example

`rip --graveyard /tmp/graveyard ~/myfile.txt`

## Notes

   - You probably shouldn't alias `rm` to `rip`.  Unlearning muscle memory is hard, but it's harder to ensure that every =rm= you make (as different users, from different machines and application environments) is the aliased one.
   - If you have `$XDG_DATA_HOME=` environment variable set, `rip` will use `$XDG_DATA_HOME/graveyard` instead of the `/tmp/graveyard-$USER`.
   - If you want to put the graveyard somewhere else (like `~/.local/share/Trash`), you have two options, in order of precedence:
        1. Alias `rip` to `rip --graveyard ~/.local/share/Trash`
        2. Set the environment variable `$GRAVEYARD` to `~/.local/share/Trash`.
     This can be a good idea because if the graveyard is mounted on an in-memory filesystem (as /tmp is in Arch Linux), deleting large files can quickly fill up your RAM.  It's also much slower to move files across filesystems, although the delay should be minimal with an SSD.
   - In general, a deletion followed by a `--unbury` should be idempotent.
   - The deletion log is kept in `.record`, found in the top level of the graveyard.
   - On a Mac, files are sent to `~/.Trash` by default so they are easy to recover.
