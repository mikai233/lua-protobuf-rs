---@class com_mikai233_TestMessage_A_B
---@field a? integer
---@field b? string
---@field c? string
---@field ccccccccc? com_mikai233_TestMessage_A
local com_mikai233_TestMessage_A_B = { }

---@class com_mikai233_TestMessage_A_A
local com_mikai233_TestMessage_A_A = { }

---@class com_mikai233_TestMessage_A
---@field a? integer
---@field b? string
---@field c? table<integer,com_mikai233_TestMessage_A_B>
---@field d? com_mikai233_TestMessage_A_B[]
local com_mikai233_TestMessage_A = { }

---@class com_mikai233_TestMessage_B
local com_mikai233_TestMessage_B = { }

---@class com_mikai233_TestMessage
---@field a? com_mikai233_TestMessage_A
---@field one_of_b? string
---@field one_of_c? string
---@field one_of_b2? com_mikai233_TestMessage_A
---@field one_of_c2? com_mikai233_TestMessage_B
local com_mikai233_TestMessage = { }

---@class com_mikai233_TestEnum
---@field A integer 0
---@field B integer 1
local com_mikai233_TestEnum = { A = 0, B = 1 }
