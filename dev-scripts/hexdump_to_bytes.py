"""
python dev-scripts/hexdump_to_bytes.py infile.log outfile.log
"""
from sys import argv
from binascii import unhexlify

if __name__ == "__main__":
    s, o = argv[1], argv[2]
    with open(s, "rt", encoding="utf-8") as infile:
        txt = infile.read()
    hex_txt = txt.replace(" ", "").replace("\n", "")
    print(repr(hex_txt))
    b = unhexlify(hex_txt)
    with open(o, "wb") as outfile:
        outfile.write(b)