#!/bin/bash
! tmux kill-session -t mordoprojekt
sleep 2
tmux new -d -s mordoprojekt -- ./mordoprojekt-bot
