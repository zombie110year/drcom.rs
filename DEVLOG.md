# 2021-05-09

从 [Hagb/CQU_drcom](https://github.com/purefkh/CQU_drcom/pull/31/files) 处得知，在 `keep_alive_stable` 阶段中，服务器可能会返回

```
检测发现 1 次, 2 次后处理 , 终端 > 2,
```

的 GBK 字符串，这时将会导致中断，需要重新登录。

```log
[2021-05-09T11:13:16Z TRACE drcom::app keep_alive_3 recv(42)]
4d 38 bc cf bc ec b2 e2 b7 a2 cf d6 20 31 20 b4
ce 2c 20 32 20 b4 ce ba f3 b4 a6 c0 ed 20 2c 20
d6 d5 b6 cb 20 3e 20 32 2c 20
M8枷检测发现 1 次, 2 次后处理 , 终端 > 2,
```

+ header `4d 38 bc cf`