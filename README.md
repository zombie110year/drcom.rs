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

+ `behaviro.log_*` 目前使用环境变量 `RUST_LOG` 来配置日志，配置文件中的设置将被忽略。
+ `server.host_ip` 填写路由器或者上网设备的 IP。
+ `server.mac` 可以随机生成，也可以使用真实设备的 MAC 地址。
+ `server.primary_dns` 主要的 DNS 服务器，默认使用 Google 的公共 DNS。
+ `server.server` DRCOM 认证服务器的地址，`dr.com:61440` 是其所使用的域名，也可以填写 IP 地址；如果已知 IP，想知道域名，可以通过 `nslookup <ip>` 来查询。

以下有一些配置文件模板，来自 [purefkh/CQU_drcom](https://github.com/purefkh/CQU_drcom/)。

<details>
<summary>
虎溪校区（D 校区）的配置，主要修改 <code>[signal]</code> 下的部分。
</summary>

```toml
[account]
username = ""
password = ""

[behavior]
log_level = "trace"
log_file = "./drcom.log"
ror_version = false
max_retry = 1

[server]
dhcp_server = "0.0.0.0"
host_ip = ""
host_name = ""
host_os = ""
mac = 0
primary_dns = "8.8.8.8"
server = "dr.com:61440"

[signal]
adapter_num = 0x00
auth_version = [0x2f, 0x00]
control_check_status = 0x00
ip_dog = 0x01
keep_alive_version = [0xdc, 0x02]
```
</details>

<details>
<summary>
沙坪坝校区（A, B 校区）的配置，主要修改 <code>[signal]</code> 下的部分。
</summary>

```toml
[account]
username = ""
password = ""

[behavior]
log_level = "trace"
log_file = "./drcom.log"
ror_version = false
max_retry = 10

[server]
dhcp_server = "0.0.0.0"
host_ip = ""
host_name = ""
host_os = ""
mac = 0
primary_dns = "8.8.8.8"
server = "dr.com:61440"

[signal]
adapter_num = 0x06
auth_version = [0x25, 0x00]
control_check_status = 0x20
ip_dog = 0x01
keep_alive_version = [0xd8, 0x02]
```
</details>
