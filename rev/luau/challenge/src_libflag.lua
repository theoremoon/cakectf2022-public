local function _checkFlag(_flag, _key)
   local ans = {62, 85, 25, 84, 47, 56, 118, 71, 109, 0, 90, 71, 115, 9, 30, 58, 32, 101, 40, 20, 66, 111, 3, 92, 119, 22, 90, 11, 119, 35, 61, 102, 102, 115, 87, 89, 34, 34}
   if #_flag ~= #ans then
      return false
   end

   local flag = {}
   local key = {}
   for i = 1, #_flag do
      flag[i] = string.byte(_flag:sub(i, i+1))
   end
   for i = 1, #_key do
      key[i] = string.byte(_key:sub(i, i+1))
   end

   for i = 1, #flag do
      for j = i + 1, #flag do
         local t = flag[i]
         flag[i] = flag[j]
         flag[j] = t
      end
   end

   for i = 1, #flag do
      flag[i] = flag[i] ~ key[1 + ((i-1) % #key)]
      if flag[i] ~= ans[i] then
         return false
      end
   end

   return true
end

return {
   checkFlag = _checkFlag
}
