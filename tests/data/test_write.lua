function process(name)
  for _ = 1, 5 do
    sys_write(name)  
  end
end

function main()
  sys_spawn("process", "A")
  sys_spawn("process", "B")
end
