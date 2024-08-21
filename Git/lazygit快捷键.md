# lazygit快捷键
## Global 全局
- d:        删除选中内容，具体操作根据面板变化

## Files 面板
- a:        stage/unstage所有文件和文件夹 (编辑区 <--> 暂存区)
- 空格:     单独stage/unstage文件或文件夹
- Enter + : 文件内选择stage内容，更细粒度 ( 按tab切换 stage <--> unstage)     
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_07-36-59.png)
- c:        提交暂存区内容。(暂存区 --> 归档区)
- C:        使用git editor提交暂存区内容。
- A:        提交暂存区内容，合并到上一次commit (amend last commit)
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_07-56-01.png)
- d:        丢弃选定文件修改（discard all change, unstage中按 e 表示删除修改，该操作不可逆）
- D:        选择丢弃内容，可选丢弃全部或部分，根据情况进行选择
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_07-58-37.png) 
- s:        stash隐藏编辑区 + 暂存区所有文件 (Files --> Stash)

## Local Branches - Remotes - Tags 面板
- n:        从选定分支新建分支
- P:        push 到当前分支的远程分支

## Commits - Reflog 面板
- M:        合并选定分支到当前分支
- 空格:     check out选中提交
- t:        撤回选定提交(revert commit)，会增加一次revert commit 日志
- g:        重置提交(reset commit: hard | mixed | soft) 
- c:        cherry-pick某(多)次commit的hash code(可多选，再按c取消pick)
- v:        merge cherry-pick到选中分支
- W:        比较选中commit和其他commit的不同
- s:        合并选中提交到前一次提交(squash)
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_09-12-57.png)
- f:        fix up选中提交到前一次提交(和squash的区别，fix up会沿用上一次提交的commit msg)
- e:        同时修改某次提交后面的多个提交，支持drop、squash、fixup，一次性执行,按m可以选择不同操作，停止或者继续
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_09-37-43.png)
- T:        对选定提交打标签(tag commit)
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_08-34-00.png)
- Enter + : 查看选中提交内容，进行操作，按<esc>或者 [ ] 返回
            (可以对文件进行custom patch,比如将一次commit的部分内容移动到另一次提交-->即补丁的功能)
![](https://raw.githubusercontent.com/hongwei36/image/main/Snipaste_2022-12-01_08-45-04.png) 
- r:        重命名提交(rename commit)
- R:        使用git editor重命名提交

## Stash
- g:        pop倒出隐藏文件 
- d:        删除选定隐藏
- 空格:     copy
