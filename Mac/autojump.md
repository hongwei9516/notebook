## 1 install
```sh
brew install autojump

## config ~/.zshrc 
[ -f /opt/homebrew/etc/profile.d/autojump.sh ] && . /opt/homebrew/etc/profile.d/autojump.sh
```

## 2 custom command with fzf
```sh

## autojump
j() {
    if [[ "$#" -ne 0 ]]; then
        cd $(autojump $@)
        return
    fi
    cd "$(autojump -s | sort -k1gr | awk '$1 ~ /[0-9]:/ && $2 ~ /^\// { for (i=2; i<=NF; i++) { if (i<NF) { printf "%s ", $(i) } else { print $(i) } } }' |  fzf --height 20% --reverse --inline-info)" 
}

```
