# tinyserve

最初需求：为了自定义http文件下载服务中的Content-Type字段，以满足测试需求。

## ToDo
默认的Header字段全是小写，。。。。。。    Fuck YOU !!!!

```
 ~/tinyserve/target/debug/ [main+*] ./tinyserve -h
Usage: tinyserve [OPTIONS]

Options:
  -p <port>         监听端口，默认8808 [default: 8808]
  -H <headers>      需要修改的Header项，如： Content-Type: image/png
  -T <target>       指定待分享的文件，（暂不支持目录）。
  -h, --help        Print help
  -V, --version     Print version
 ~/tinyserve/target/debug/ [main+*] ./tinyserve -H "Content-Type: image/png" -H "Server: tinyserve" -T ~/shell.php
127.0.0.1:61417 - - GET /1  target to /Users/triplebiu/shell.php

---------------------------
 ~/ curl -v http://127.0.0.1:8808/1
*   Trying 127.0.0.1:8808...
* Connected to 127.0.0.1 (127.0.0.1) port 8808 (#0)
> GET /1 HTTP/1.1
> Host: 127.0.0.1:8808
> User-Agent: curl/7.88.1
> Accept: */*
>
< HTTP/1.1 200 OK
< content-type:  image/png
< server:  tinyserve
< transfer-encoding: chunked
< date: Wed, 14 Jun 2023 06:44:02 GMT
<
<?php
```


# 改用Python实现：

```
python tinyserve.py
```