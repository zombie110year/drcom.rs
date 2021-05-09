#! /usr/bin/python
# python >= 3.6 < 4.0

import re
from sys import argv
from typing import List, Optional, Tuple
from pathlib import Path


def parse_log(
    line: str,
    pat: re.Pattern = re.compile(
        r"^\[(?P<dt>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\S) (?P<lv>ERROR|WARN |INFO |DEBUG|TRACE) (?P<md>[a-z:_]+)\] (?P<msg>.+?) ?(?P<bt>\[[\d, ]+\])?$"
    )
) -> Tuple[str, str, str, str, Optional[str]]:
    """解析日志

    1. 时间戳
    2. 等级
    3. 模块
    4. 消息
    5. 报文或其他字节
    """
    m = pat.match(line)
    if m is None:
        print(line)
        raise ValueError
    mg = m.groupdict()
    return (mg['dt'], mg['lv'], mg['md'], mg['msg'], mg['bt'])


def hexdump(b: bytes, encoding="gbk") -> str:
    """每行 16 字节
    """
    width = 16
    b_len = len(b)
    b_arr = [
        b[width * i:width * i + width]
        for i in range(b_len // width if b_len %
                       width == 0 else b_len // width + 1)
    ]
    hexcodes = [bi.hex(" ") for bi in b_arr]
    chars = b.decode(encoding, "replace")
    buffer = hexcodes + [chars]
    return "\n".join(buffer)


def main():
    logi, logo = argv[1], argv[2]
    with Path(logi).open("rt", encoding="utf-8") as srcfile:
        with Path(logo).open("wt", encoding="utf-8") as outfile:
            for line in iter(lambda : srcfile.readline(), ""):
                parsed = parse_log(line)
                header = "[" + " ".join(parsed[:4]) + "]"
                print(header)
                if parsed[4] is not None:
                    #! parsed[4] is [...]
                    message = hexdump(bytes(eval(parsed[4])))
                    outfile.write(header)
                    outfile.write("\n")
                    outfile.write(message)
                    outfile.write("\n")
                    outfile.write("\n")
                else:
                    outfile.write(header)
                    outfile.write("\n")
                    outfile.write("\n")

if __name__ == "__main__":
    main()