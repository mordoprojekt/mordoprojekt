#!/bin/bash
tmux kill-session -t mordoprojekt || true
sleep 2
tmux new -d -s mordoprojekt -- ./mordoprojekt-bot
