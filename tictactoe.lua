circleLocation = "~1 ~-2 ~-1"
crossLocation = "~2 ~-2 ~-1"

function placeBlock(i, j, type)
    commands.clone(circleLocation .. " " .. circleLocation .. " " .. string.format("~%d ~%d ~%d", i + 1, j, 1))
end

placeBlock(0, 0, 0)
