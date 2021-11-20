import html
import sys

# https://stackoverflow.com/a/42672936
[print(html.unescape(l), end="") for l in sys.stdin]
