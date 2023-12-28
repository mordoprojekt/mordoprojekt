#!/bin/bash
tmux kill-session -t mordoprojekt || true
tmux new -d -s mordoprojekt -- ./mordoprojekt
