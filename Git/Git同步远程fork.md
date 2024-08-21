## 1. 添加上游fork
```sh
git remote add upstream https:/github.com/xx/xx
```

## 2. fetch 上游代码
```sh
git fetch upstream
git branch -a
```

## 3. merge 上游代码到本地
```sh
git merge upstream/xxx
git push origin HEAD --force
```
![](https://raw.githubusercontent.com/hongwei36/image/main/Screenshot%202024-04-21%20at%2021.14.30.png)
![](https://raw.githubusercontent.com/hongwei36/image/main/Screenshot%202024-04-21%20at%2021.50.34.png)

