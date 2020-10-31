import struct
import binascii
from hashlib import md5

def makeLoginPacket(
    username: str,
    password: str,
    SALT: bytes,
    CONTROL_CHECK_STATUS: bytes,
    ADAPTER_NUM: bytes,
    mac: int,
    host_ip: str,
    IP_DOG: bytes,
    host_name: str,
    dns: str,
    dhcp: str,
    host_os: str,
    AUTH_VERSION: bytes,
    ROR_VERSION: bool,
) -> bytes:
    """构建 login 包

    读取属性

    -   context.username
    -   context.password
    -   context.SALT
    -   context.CONTROL_CHECK_STATUS
    -   context.ADAPTER_NUM
    -   context.mac
    -   context.host_ip
    -   context.IP_DOG
    -   context.host_name
    -   context.dns
    -   context.dhcp
    -   context.host_os
    -   context.AUTH_VERSION
    -   context.ROR_VERSION

    调用函数

    -   utils.md5sum
    -   utils.dump
    -   utils.ror
    -   utils.checksum

    -
    """
    data = b''
    """
    struct  _tagLoginPacket {
        struct _tagDrCOMHeader Header;
        unsigned char PasswordMd5[MD5_LEN];
        char Account[ACCOUNT_MAX_LEN];
        unsigned char ControlCheckStatus;
        unsigned char AdapterNum;
        unsigned char MacAddrXORPasswordMD5[MAC_LEN];
        unsigned char PasswordMd5_2[MD5_LEN];
        unsigned char HostIpNum;
        unsigned int HostIPList[HOST_MAX_IP_NUM];
        unsigned char HalfMD5[8];
        unsigned char DogFlag;
        unsigned int unkown2;
        struct _tagHostInfo HostInfo;
        unsigned char ClientVerInfoAndInternetMode;
        unsigned char DogVersion;
    };
    """
    # _tagLoginPacket.Header 4, 4
    data += b'\x03\x01\x00' + bytes([len(username) + 20])
    # _tagLoginPacket.PasswordMD5 16, 20
    data += md5sum(b'\x03\x01' + SALT +
                    password.encode())
    # _tagLoginPacket.Account 36, 56
    data += (username.encode() + b'\x00'*36)[:36]
    # _tagLoginPacket.ControlCheckStatus 1, 57
    data += CONTROL_CHECK_STATUS
    # _tagLoginPacket.AdapterNum 1, 58
    data += ADAPTER_NUM
    # _tagLoginPacket.MacAddrXORPasswordMD5 6, 64
    data += hexdump(
        int(
            binascii.hexlify(data[4:10]),
            base=16
        ) ^ mac
    )[-6:]
    # _tagLoginPacket.PasswordMD5_2 16, 80
    data += md5sum(b'\x01' + password.encode() +
                    SALT + b'\x00'*4)
    # _tagLoginPacket.HostIpNum 1, 81
    data += b'\x01'
    # _tagLoginPacket.HostIPList 16, 97
    data += b''.join(
        [bytes([int(i)]) for i in host_ip.split('.')]
    )
    data += b'\x00'*12
    # _tagLoginPacket.HalfMD5 [8] 8, 105
    data += md5sum(data + b'\x14\x00\x07\x0B')[:8]
    # _tagLoginPacket.DogFlag 1, 106
    data += IP_DOG
    # _tagLoginPacket.unkown2 4, 110
    data += b'\x00'*4
    # _tagLoginPacket.HostInfo
    """
    struct  _tagHostInfo {
        char HostName[HOST_NAME_MAX_LEN];
        unsigned int DNSIP1;
        unsigned int DHCPServerIP;
        unsigned int DNSIP2;
        unsigned int WINSIP1;
        unsigned int WINSIP2;
        struct _tagOSVersionInfo OSVersion;
    };
    """
    # _tagHostInfo.HostName 32, 142
    data += (host_name.encode() + b'\x00'*32)[:32]
    # _tagHostInfo.DNSIP1 4, 146
    data += b''.join(
        [bytes([int(i)]) for i in dns.split('.')]
    )
    # _tagHostInfo.DHCPServerIP 4, 150
    data += b''.join(
        [bytes([int(i)]) for i in dhcp.split('.')]
    )
    # _tagHostInfo.DNSIP2 4, 154
    data += b'\x00'*4
    # _tagHostInfo.WINSIP1 4, 158
    data += b'\x00'*4
    # _tagHostInfo.WINSIP2 4, 162
    data += b'\x00'*4
    # _tagHostInfo.OSVersion
    """
    struct  _tagOSVersionInfo {
        unsigned int OSVersionInfoSize;
        unsigned int MajorVersion;
        unsigned int MinorVersion;
        unsigned int BuildNumber;
        unsigned int PlatformID;
        char ServicePack[128];
    };
    """
    # _tagOSVersionInfo.OSVersionInfoSize 4, 166
    data += b'\x94\x00\x00\x00'
    # _tagOSVersionInfo.MajorVersion 4, 170
    data += b'\x05\x00\x00\x00'
    # _tagOSVersionInfo.MinorVersion 4, 174
    data += b'\x01\x00\x00\x00'
    # _tagOSVersionInfo.BuildNumber 4, 178
    data += b'\x28\x0A\x00\x00'
    # _tagOSVersionInfo.PlatformID 4, 182
    data += b'\x02\x00\x00\x00'
    # _tagOSVersionInfo.ServicePack 128, 310
    data += (host_os.encode() + 32*b'\x00')[:32]
    data += b'\x00'*96
    # _tagLoginPacket.ClientVerInfoAndInternetMode 1, 311
    # _tagLoginPack. DogVersion; 1, 312
    data += AUTH_VERSION
    if ROR_VERSION:
        """
        struct _tagLDAPAuth {
            unsigned char Code;
            unsigned char PasswordLen;
            unsigned char Password[MD5_LEN];
        }
        """
        # _tagLDAPAuth.Code
        data += b'\x00'
        # _tagLDAPAuth.PasswordLen
        data += bytes([len(password)])
        # _tagLDAPAuth.Password
        data += ror(md5sum(b'\x03\x01' + SALT +
                            password), password)
    """
    struct  _tagDrcomAuthExtData {
        unsigned char Code;
        unsigned char Len;
        unsigned long CRC;
        unsigned short Option;
        unsigned char AdapterAddress[MAC_LEN];
    };
    """
    # _tagDrcomAuthExtData.Code 1, 313
    data += b'\x02'
    # _tagDrcomAuthExtData.Len 1, 314
    data += b'\x0c'
    # _tagDrcomAuthExtData.CRC 4, 318
    data += checksum(
        data + b'\x01\x26\x07\x11\x00\x00' + hexdump(mac)
    )
    # _tagDrcomAuthExtData.Option 2, 320
    data += b'\x00\x00'
    # _tagDrcomAuthExtData.AdapterAddress 6, 326
    data += hexdump(mac)
    # auto logout / default: False 1, 327
    data += b'\x00'
    # broadcast mode / default: False 1, 328
    data += b'\x00'
    # unknown, 随机填充的 2, 330
    data += b'\xe9\x13'
    return data




def md5sum(x: bytes) -> bytes:
    """得到输入字节的 md5 值

    >>> md5sum(b'\x03\x01')
    b' \xf9\xaa|\x18\x9a\xf6\xe6A\xa46i\xbe\xbf\x1cc'
    """
    m = md5()
    m.update(x)
    return m.digest()


def hexdump(n: int) -> bytes:
    """将整数转换为对应的字节

    (大端)

    >>> dump(1)
    b'\x01'
    >>> dump(0x16)
    b'\x16'
    >>> dump(0xffffffff)
    b'\xff\xff\xff'
    """
    s = '%x' % n
    if len(s) & 1:  # 奇数
        s = '0' + s
    return binascii.unhexlify(bytes(s, 'ascii'))


def ror(md5sum: bytes, pwd: str) -> str:
    """ror 加密

    :param bytes md5sum: md5 检验和 16 字节
    :param str pwd: Drcom 用户的密码
    """
    result = []
    for i in range(len(pwd)):
        x = ord(md5sum[i]) ^ ord(pwd[i])
        result.append(struct.pack("B", ((x << 3) & 0xff) + (x >> 5)))
    return ''.join(result)


def checksum(bytes_: bytes) -> bytes:
    """checksum 验证，引用 self.mac

    (小端) 01020304 -> 0x04030201
    """
    resualt = 1234
    for i in [x*4 for x in range(0, -(-len(bytes_)//4))]:
        resualt ^= int(
            binascii.hexlify(bytes_[i:i+4].ljust(4, b'\x00')[::-1]), 16
        )

    resualt = (1968 * resualt) & 0xffffffff
    return struct.pack('<I', resualt)

if __name__ == "__main__":
    pack = makeLoginPacket(
        "20172744",
        "zxcvbnmasdf",
        b'/\xc3|\x00',
        b"\x20",
        b"\x07",
        0xEC4118D666AF,
        "172.25.148.82",
        b"\x01",
        "XiaoMi Router",
        "8.8.8.8",
        "0.0.0.0",
        "Linux",
        b"\x0a\x00",
        False
    )
    with open("python.bin", "wb") as py:
        py.write(pack)
