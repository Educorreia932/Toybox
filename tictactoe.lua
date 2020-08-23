circleLocation = "~1 ~-2 ~-1"
crossLocation = "~2 ~-2 ~-1"

grid = {
    {0, 0, 0},
    {0, 0, 0},
    {0, 0, 0}
}

function placeBlock(i, j, type)
    commands.clone(circleLocation .. " " .. circleLocation .. " " .. string.format("~%d ~%d ~%d", i + 1, j, 1))
    grid[i + 1][j + 1] = 1
end

function checkVictory()
    for i = 1, 3
    do 
        rowSum = sumRow(i)
        collumnSum = sumCollumn(i)
        diagonalSum = sumDiagonal(i)

        if rowSum == 3 or collumnSum == 3 or diagonalSum == 3 then
            return 1

        elseif rowSum == -3 or collumnSum == -3 or diagonalSum == -3 then
            return -1
        
        else
            return 0
        end
    end
end

function sumDiagonal(slope)
    result = 0

    for i = 1, 3
    do
        for j = 1, 3
        do
            if i == j then
                result = result + grid[i][j]
            end
        end
    end

    return result
end

function sumRow(i)
    result = 0

    for j = 1, 3
    do
        result = result + grid[i][j]
    end

    return result
end

function sumCollumn(j)
    result = 0

    for i = 1, 3
    do
        result = result + grid[i][j]
    end

    return result
end

function clear() 

end

placeBlock(0, 0, 0)
