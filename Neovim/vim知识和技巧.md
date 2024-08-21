# (neo)vim 技巧
## 1. 使用 o 或者 O 选项打开水平或者垂直分割窗口
```bash
## 打开两个水平分割窗口，分别显示hello1.txt和hello2.txt的内容
vim -o2 hello1.txt hello2.txt
## 同理，方向改为垂直
vim -O2 hello1.txt hello2.txt
```

## 2. buffers中代码跳转
- `Ctrl-O`: 跳转至跳转列表中旧的位置。
- `Ctrl-I`: 跳转至跳转列表中新的位置。

## 3. 普通模式下关于 windows 的命令
```vim
Ctrl-W V    打开一个新的垂直分割的窗口
Ctrl-W S    打开一个新的水平分割的窗口
Ctrl-W C    关闭一个窗口
Ctrl-W O    除了当前窗口，关闭所有其他的窗口

:vsplit filename    垂直分割当前窗口，并在新窗口中打开名为filename的文件。
:split filename     水平分割当前窗口，并在新窗口中打开名为filename的文件。
:new filename       创建一个新窗口并打开名为filename的文件。
```

## 4. tabs
    tabs就是windows的集合，相当于窗口的布局。可以创建多个tab使用不同的布局使用。
```vim
:tabnew file.txt    在tab中打开一个文件
:tabclose           关闭当前tab
:tabnext            切换至下一个tab
:tabprevious        切换至前一个tab
:tablast            切换至最后一个tab
:tabfirst           切换至第一个tab
```

## 5. :set path使用
默认情况下，path值为:
```vim
path=.,/usr/include,,
```
- . 表示当前打开文件所在目录
- /usr/include 目录
- 空 表示命令行所在目录

`:find` 命令会默认在path默认的目录中查找文件，可以通过`:set path+=`添加额外目录进行拓展
```vim
:set path+=app/controllers/
```

## 6. 点命令
### 用法

正如这个命令的名字一样，你可以通过按下`.`键来使用点命令。

比如，如果你想将下面文本中的所有”let“替换为"const"：

```
let one = "1";
let two = "2";
let three = "3";
```

- 首先，使用`/let`来进行匹配。
- 接着，使用`cwconst<esc>`来将"let"替换成"const"。
- 第三步，使用`n`来找到下一个匹配的位置。
- 最后，使用点命令(`.`)来**重复之前的修改操作**。
- 持续地使用`n . n .`直到每一个匹配的词都被替换。

### 什么算是修改操作
当你使用普通模式下的命令来更新（添加，修改或者删除）当前缓冲区中的内容时，你就是在执行一个修改操作了。

**注意： 修改操作是不包括移动（motions）的，因为移动(motions)不会更新缓冲区的内容。**


## 7. 寄存器
### 寄存器的10中类型：
1. 匿名寄存器（`""`）.
2. 编号寄存器(`"0-9`).
3. 小删除寄存器 (`"-`).
4. 命名寄存器 (`"a-z`).
5. 只读寄存器 (`":`, `".`, and `"%`).
6. Buffer交替文件寄存器 (`"#`).
7. 表达式寄存器 (`"=`).
8. 选取和拖放寄存器(`"*` and `"+`).
9. 黑洞寄存器 (`"_`).
10. 搜索模式寄存器 (`"/`).
### 在输入模式下使用寄存器
```
Ctrl-r x // x为寄存器标志，比如0-9或者其他
```

================================================>
## 1. 代码进入和代码返回
```
ctrl + i ==> 进入
ctrl + o ==> 返回
```

## 2. 跳转
```
g + ` + " ==> go to last change position
g + ` + ` ==> go to last search/jump position
```

