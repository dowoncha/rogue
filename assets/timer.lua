local script_name = "dc_timer"

local timer
local init

function on_gamemode(seconds_passed)
    if init == 0
        timer = 25
        init = 1
    else
        if timer > 0
            timer -= seconds_passed 
        else
            // code to execute after timer
            -- message("timer expired")
        end
    end
end