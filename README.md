# drcom by rust

# 安装

在 Release 界面下载预编译包：

| 二进制包 | 适用平台 |
|:-:|:-:|
| `drcom` | x86_64-unknown-linux-gnu |
| `drcom.exe` | x86_64-pc-windows-gnu |
| `drcom-aarch64` | aarch64-unknown-linux-gnu |

自行编译：

```sh
cargo install drcom
```

# 使用方法

运行 `drcom` 即可。

1. 默认载入配置文件 `./drcom.toml` 或 `$CONFIG_DIR/drcom/drcom.toml`
	1. Windows: `%APPDATA%/drcom/drcom.toml`
	2. Linux: `$XDG_CONFIG_HOME/drcom/drcom.toml`
2. 手动指定配置文件：在命令行参数的第一项指定 `drcom example.toml`

# 配置文件模板

默认的配置文件：

```toml
[account]
# DRCOM 账户
username = ""
password = ""

[behavior]
log_level = "trace"
# note 暂时无法指定日志文件，需要手动重定向
log_file = "./drcom.log"
ror_version = false
max_retry = 10

[server]
dhcp_server = "0.0.0.0"
# todo 路由器或者上网设备的 IP
host_ip = ""
host_name = "HOME"
host_os = "Windows"
# 随机生成，也可以用真实 MAC 地址
mac = 20015998341138
# Google 公共 DNS，也可以改成别的
primary_dns = "8.8.8.8"
# 重庆大学 DRCOM 验证服务器地址
server = "gate.cqu.edu.cn:61440"

[signal]
adapter_num = 7
auth_version = [10, 0]
control_check_status = 32
ip_dog = 1
keep_alive_version = [220, 2]
```

