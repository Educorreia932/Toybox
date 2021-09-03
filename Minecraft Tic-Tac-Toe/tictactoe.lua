circleLocation = "~1 ~-2 ~-1"
crossLocation = "~2 ~-2 ~-1"

grid = {
    {0, 0, 0},
    {0, 0, 0},
    {0, 0, 0}
}

function placeBlock(i, j, playerNumber)
    if playerNumber == 1 then
        commands.clone(circleLocation .. " " .. circleLocation .. " " .. string.format("~%d ~%d ~%d", i + 1, j, 1))

    else
        commands.clone(crossLocation .. " " .. crossLocation .. " " .. string.format("~%d ~%d ~%d", i + 1, j, 1))

    end

    grid[i + 1][j + 1] = playerNumber
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

function clear() 
    grid = {
        {0, 0, 0},
        {0, 0, 0},
        {0, 0, 0}
    }
end

function userInput(playerNumber)
    print("Enter the i coordinate of your play: ")
    local i = read()
    print("Enter the j coordinate of your play: ")
    local j = read()
    print()

    placeBlock(i, j, playerNumber)
end

counter = 1 

while true do
    local playerNumber = counter % 2

    if playerNumber == 0 then
        playerNumber = -1
    end

    userInput(playerNumber)
    counter = counter + 1

    local victory = checkVictory()

    if victory == -1 then
        print("Red wins!")
        break

    elseif victory == 1 then
        print("Blue wins!")
        break
    end
end
