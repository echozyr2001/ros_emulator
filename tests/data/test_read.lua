-- 每次使用sys_read()从系统中读取一个0或1
-- 调用5次sys_read(), 将结果拼接为一个长度为5的字符串
-- 在使用sys_write()将结果写入系统中 
function process()
  local result = ""
  for _ = 1, 5 do
    result = result .. sys_read()
  end
  sys_write(result)
end

function main()
  sys_spawn("process", "")
end
