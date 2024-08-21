## 1. docker 基本使用

```
                            仓库
                            ｜｜
                            ｜｜
                        pull｜｜push
                            ｜｜
                  build     ｜｜     save
   Dockerfile   =========>  镜像 <==========> tar
                            ｜｜     load
                            ｜｜
                        run ｜｜commit
                            ｜｜
                            ｜｜
                            容器
```

## 2. docker 容器间的访问
```sh
docker run -d -p80:80 --name mynginx nginx
## 通过 --link 的方式将mynginx容器映射为myng, alpine容器就可以通过 myng 访问mynginx容器
docker run -dit --link mynginx:myng alpine
```
![](https://raw.githubusercontent.com/hongwei36/image/main/Screenshot%202024-04-23%20at%2010.13.06.png)

