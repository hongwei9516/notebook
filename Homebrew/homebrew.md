## 1 homebrew 如何安装特定版本 formula/cask
```sh
# 关闭使用网络接口安装
export HOMEBREW_NO_INSTALL_FROM_API=1

# tap core/cask 到本地
brew tap homebrew/core
brew tap homebrew/cask

# brew edit
brew edit iina-plus

# reinstall
brew reinstall iina-plus
```

## 2. 在线查询formula and cask
formula 查询
https://github.com/Homebrew/homebrew-core/find/master

cask 查询
https://github.com/Homebrew/homebrew-cask/find/master
