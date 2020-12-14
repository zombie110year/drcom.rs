from io import StringIO

def read_rust_vec(literal: str) -> bytes:
    """将 log 中的列表转换为字节序列

    >>> read_rust_vec("[1, 2, 3]")
    b"\x01\x02\x03"
    """
    assert isinstance(literal, str)
    assert literal.startswith("[") and literal.endswith("]")
    vec = eval(literal)
    data = bytes(vec)
    return data


def annotate_bytes(b: bytes) -> str:
    io = StringIO()
    parts = [
        # title, len, from, to
        ("header", 4, 0, 4),
        ("password_md5", 16, 4, 20),
        ("account", 36, 20, 56),
        ("control_check_status", 1, 56, 57),
        ("adapter_num", 1, 57, 58),
        ("mac_xor_password_md5", 6, 58, 64),
        ("password_md5_2", 16, 64, 80),
        ("host_ip_num", 1, 80, 81),
        ("host_ip_list", 16, 81, 97),
        ("half_md5", 8, 97, 105),
        ("ip_dog", 1, 105, 106),
        ("unknown", 4, 106, 110),
        ("host_name", 32, 110, 142),
        ("dns", 4, 142, 146),
        ("dhcp", 4, 146, 150),
        ("dns2", 4, 150, 154),
        ("winsip1", 4, 154, 158),
        ("winsip2", 4, 158, 162),
        ("os_version_size", 4, 162, 166),
        ("major", 4, 166, 170),
        ("minor", 4, 170, 174),
        ("build_number", 4, 174, 178),
        ("platform_id", 4, 178, 182),
        ("service_pack", 128, 182, 310),
        ("auth_version", 2, 310, 312),
        ("ext_code", 1, 312, 313),
        ("ext_len", 1, 313, 314),
        ("ext_crc", 4, 314, 318),
        ("ext_option", 2, 318, 320),
        ("adapter_addr", 6, 320, 326),
        ("auto_logout", 1, 326, 327),
        ("broadcast_mode", 1, 327, 328),
        ("unknown", 2, 328, 330),
    ]
    for label, len, start, end in parts:
        title = f"# {label} ({len}, {start}:{end})"
        body = b[start:end]
        io.write(title)
        io.write("\n")
        io.write(hexdump(body))
        io.write("\n")
    io.seek(0, 0)
    return io.read()

def hexdump(b: bytes) -> str:
    io = StringIO()
    numbers = [f"{i:02x}" for i in b]
    for i in range(len(numbers) // 8 + 1):
        chunk = numbers[8*i:8*(i+1)]
        io.write(" ".join(chunk))
        io.write("\n")
    io.seek(0, 0)
    return io.read()

if __name__ == "__main__":
    vec = input()
    data = read_rust_vec(vec)
    hx = annotate_bytes(data)
    print(hx)