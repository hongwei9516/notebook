## 1 gitstatus
```sh
## gitstatus 
source /opt/homebrew/opt/gitstatus/gitstatus.prompt.zsh
# 终端PS1格式设置：'[hong@MacBook] ~(gitstatus) % '
# %# 普通用户为%,管理员为#
export prompt='%F{cyan}[%f%n%F{red}@%fMacBook%F{cyan}]%f %F{blue}%~%f '
prompt+='${GITSTATUS_PROMPT:+($GITSTATUS_PROMPT)}'
prompt+=$'\n'
prompt+='%F{cyan}$%f '

```
