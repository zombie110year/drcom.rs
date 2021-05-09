# drcom by rust

# 安装

在 Release 界面下载预编译包：

|      二进制包       |                 适用平台                 |
| :-----------------: | :--------------------------------------: |
|     `drcom-cli`     |        `x86_64-unknown-linux-gnu`        |
|   `drcom-cli.exe`   |         `x86_64-pc-windows-gnu`          |
| `drcom-cli.aarch64` | `aarch64-unknown-linux-gnu` ，如树莓派 4 |

自行编译：

```sh
git clone https://github.com/zombie110year/drcom.rs.git
cd drcom.rs
cargo install drcom-cli --path ./drcom-cli
```

# 使用方法

由于 `pretty_env_logger` 使用环境变量 `RUST_LOG` 设置日志过滤等级，因此建议使用命令行：

```sh
# Linux
RUST_LOG=trace drcom-cli run 2>&1 > /dev/null | tee -a drcom.log | grep -e 'INFO|WARN|ERROR'
```

这会将完全的日志输出至文件 `drcom.log`，但将 `INFO`, `WARN`, `ERROR` 等级的日志显示在终端。

1. 默认载入配置文件 `./drcom.toml` 或 `$CONFIG_DIR/drcom/drcom.toml`
   1. Windows: `%APPDATA%/drcom/drcom.toml`
   2. Linux: `$XDG_CONFIG_HOME/drcom/drcom.toml`
2. 手动指定配置文件：`drcom-cli run -c example.toml`
3. 生成默认配置文件：`drcom-cli default-toml`，会在当前工作目录下生成 `drcom.default.toml`，修改后保存至 `$CONFIG_DIR/drcom/drcom.toml` 以便自动载入

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
server = "dr.com:61440"

[signal]
adapter_num = 7
auth_version = [10, 0]
control_check_status = 32
ip_dog = 1
keep_alive_version = [220, 2]
```
