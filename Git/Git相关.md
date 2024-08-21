## 1. git diff接受的编码格式为utf-8，而文件名是以gb2312格式编写的，所以git diff显示了乱码
```git
git config --global core.quotepath false
```

## 2. clone 单一文件夹
```git

git init
git remote add origin https://github.com/mbadolato/iTerm2-Color-Schemes.git
git branch -M master

## 配置单独clone
git config core.sparsecheckout true
## 将需要clone的文件夹写到sparse-checkout文件
echo 'schemes' > .git/info/sparse-checkout
## 添加额外文件夹
git sparse-checkout add terminal
## 重新应用
git sparse-checkout reapply

git pull origin master 

```
