# Worktree - Git Worktree Manager

A CLI tool for managing git worktrees with ease.

## Installation

Build the tool:
```bash
cargo build --release
```

The binary will be available at `target/release/worktree`.

## Configuration

Set the `WORKTREE_ROOT_DIR` environment variable to specify where worktrees should be created:

```bash
export WORKTREE_ROOT_DIR="$HOME/worktrees"
```

Add this to your shell configuration file (`.bashrc`, `.zshrc`, etc.) to make it permanent.

## Shell Integration

Since a child process cannot change the parent shell's directory, you need to use a shell wrapper function. Add this to your shell configuration:

### For Bash/Zsh:

```bash
wt() {
    local output
    output=$(worktree "$@")
    local exit_code=$?

    # Check if the output contains a cd command
    if echo "$output" | grep -q "^cd "; then
        # Extract and execute the cd command
        eval "$output"
    else
        echo "$output"
    fi

    return $exit_code
}
```

### For Fish:

```fish
function wt
    set output (worktree $argv)
    set exit_code $status

    if string match -q "cd *" $output
        eval $output
    else
        echo $output
    end

    return $exit_code
end
```

## Usage

### Add a new worktree

Create a new worktree for a branch:

```bash
wt add feature-branch
# or
wt new feature-branch
# or
wt init feature-branch
```

This will:
1. Create a worktree at `$WORKTREE_ROOT_DIR/{repo_name}/feature-branch`
2. Copy the `.env` file from the current directory (if it exists)
3. Change to the new worktree directory

### Go back to master

Return to the main repository directory:

```bash
wt master
```

### List all worktrees

View all worktrees for the current repository:

```bash
wt list
```

This will show all worktrees with their paths and current branches.

### Remove a worktree

Remove a worktree by name:

```bash
wt remove feature-branch
# or
wt rm feature-branch
```

Remove the current worktree:

```bash
wt rm .
```

This will remove the current worktree and change back to the main repository directory.

**Note:** You cannot use `wt rm .` in the main repository - it will error.

## Directory Structure

Worktrees are organized as:

```
$WORKTREE_ROOT_DIR/
  └── {repo_name}/
      ├── master/           (main repo)
      ├── feature-branch/   (worktree)
      └── another-branch/   (worktree)
```

## Examples

```bash
# Set up your workspace
export WORKTREE_ROOT_DIR="$HOME/projects"
cd ~/projects/my-repo

# Create a new worktree for a feature
wt add feature-x

# Work on your feature
# ... make changes ...

# Go back to main repo
wt master

# Create another worktree
wt new bugfix-y

# List all worktrees
wt list

# Remove a worktree when done
wt rm feature-x

# Or remove current worktree
wt rm .
```
