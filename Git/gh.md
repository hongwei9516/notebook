```sh
# create a repository interactively 
# 交互式创建仓库
gh repo create

# create a new remote repository and clone it locally
# 创建一个新的远程仓库并克隆到本地
gh repo create my-project --public --clone

# create a remote repository from the current directory
# 从当前目录创建远程仓库
gh repo create my-project --private --source=. --remote=upstream
```
