#!/bin/bash

session="aoc2024"

# tmux new-session -d -s temp-session

tmux has-session -t $session

if [ $? -eq 1 ]; then
  tmux new-session -d -s $session

  tmux rename-window -t $session:1 nvim

  tmux send-keys -t $session:nvim "cd ~/Documents/aoc2024/" C-m "nvim" C-m

  tmux new-window -t $session -n zsh

  tmux select-window -t 1
fi

# tmux kill-session -t temp-session

tmux attach -t $session
