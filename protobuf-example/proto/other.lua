---@class TestMessage_A_B
---@field a number
---@field b string
---@field c string
---@field ccccccccc TestMessage_A_A
local TestMessage_A_B = { }

---@class TestMessage_A_A
local TestMessage_A_A = { }

---@class TestMessage_A
---@field a number
---@field b string
---@field c table<number,TestMessage_A_B>
---@field d TestMessage_A_B[]
local TestMessage_A = { }

---@class TestMessage_B
local TestMessage_B = { }

---@class TestMessage
---@field a TestMessage_A
---@field one_of_b string
---@field one_of_c string
---@field one_of_b2 TestMessage_A
---@field one_of_c2 TestMessage_B
local TestMessage = { }

---@class TestEnum
---@field A number 0
---@field B number 1
TestEnum = { A = 0, B = 1 }
