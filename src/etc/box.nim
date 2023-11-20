import strutils
import sugar

proc green*(s: string): string = "\e[32m" & s & "\e[0m"
proc grey*(s: string): string = "\e[90m" & s & "\e[0m"
proc yellow*(s: string): string = "\e[33m" & s & "\e[0m"
proc red*(s: string): string = "\e[31m" & s & "\e[0m"
proc none*(s: string): string = s

# some how it works idk how
proc makeBox*(text: string, title: string = "", full_style: (s: string) -> string = none, border_style: (s: string) -> string = none, text_style: (s: string) -> string = none): string =
  var items: seq[string] = text.split("\n")
  var maxWidth = title.len + 4
  result &= "╭".border_style
  for item in items:
    if int(item.len) > maxWidth - 4: maxWidth = item.len + 4
  var i = 0
  while i < maxWidth:  
    if i == int(maxWidth / 2) - title.len + 2: 
      result &= title.border_style 
      i = i + title.len + 2
      continue
    result &= "─".border_style
    inc i
  result &= "╮\n".border_style

  for item in items:
    var res = "│ " & item.text_style
    while res.len < maxWidth + text_style("").len:
      res &= " "
    res &= " │\n".border_style 
    result &= res.border_style

  result &= "╰─".border_style
  for i in 5 .. maxWidth: 
    result &= "─".border_style
  result &= "─╯".border_style
  result = result.full_style

echo makeBox("error unknown op got left 9292929 right 889299 op ? \nat idk where maybe 5:6\najjaiaiakakaioakakakiaia", "error", border_style=red, text_style=yellow)
echo "windows moment realll window".makeBox("window its cool")

