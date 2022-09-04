import sequtils, strutils
echo if stdin.readLine == (@['\xBC', '\x9E', '\x94', '\x9A', '\xBC', '\xAB', '\xB9', '\x84', '\x8C', '\xCF', '\x92', '\xCC', '\x8B', '\xCE', '\x92', '\xCC', '\x8C', '\xA0', '\x91', '\xCF', '\x8B', '\xA0', '\xBC', '\x82'].map do (c:char) -> char: char(uint8(c).xor(0xff))).join(""):
  "Correct!"
else:
  "Wrong..."
