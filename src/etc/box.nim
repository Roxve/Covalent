import strutils

echo  "╭───╮│─╯╰─"
# some how it works idk how
proc makeBox*(text: string, title: string): string =
  var items: seq[string] = text.split("\n")
  var maxWidth = int(title.len)
  result &= "╭"
  for item in items:
    if int(item.len) > maxWidth - 4: maxWidth = int(item.len) + 4
  var i = 0
  while i < int(maxWidth):  
    if i == int(maxWidth / 2) - 3: 
      result &= title 
      i = i + int(title.len) + 2
      continue
    result &= "─"
    inc i
  result &= "╮\n"

  for item in items:
    var res = "│ " & item 
    while int(res.len) < maxWidth:
      res &= " "
    res &= " │\n"
    result &= res

  result &= "╰─"
  for i in title.len .. maxWidth: 
    result &= "─"
  result &= "─╯"


echo makeBox("error unknown op got left 9292929 right 889299 op ? \nat idk where maybe 5:6\najjaiaiakakaioakakakiaia", "error")
