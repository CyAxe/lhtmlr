# LHTMLR
 A fast Lua HTML Parsing/Searching library written in Rust for Lotus Project 




### TODO
- [x] HTML Parsing
- [x] Generate CSS Selector 
  - [x] Element
  - [ ] Text
  - [ ] Comment
  - [ ] Document

### Example
```lua
local lhtmlr = require("lhtmlr")

local HTML = [[
<!DOCTYPE html>
<html>
  <head>
    <title>My HTML Page</title>
  </head>
  <body>
    <img src="profile.png">
    <h1>Welcome to my HTML page</h1>
  </body>
</html>
]]

-- Generate a CSS selector pattern for **custom** Element in the HTML input that has attributes.
local pattern = lhtmlr.css_selector("<img src=\"profile.png\">") -- Type: String
local results = lhtmlr.html_search(HTML, pattern) -- Type: Table

for _,value in pairs(results) do
    print(value)
end

-- <img src="profile.png">

```

### Installation

```bash
$ luarocks install lua-lhtmlr --local
```
