#!/usr/bin/env bash

# author: nfoj_@hotmail.com
# discription: script for connection check 
# system: arch linux

#!------------------------------------------------------------------------------------------------------!#

COLOR_RED='\033[1;31m'
COLOR_GREEN='\033[1;32m'
NO_COLOR='\033[0m'

#!------------------------------------------------------------------------------------------------------!#

# connect
# 8.8.8.8 == Google

if ! ping -c 1 8.8.8.8 -q &> /dev/null; then
  
  echo -e "${COLOR_RED}  [ERROR] - Check your internet connection and try again.${NO_COLOR}";sleep 2
  exit 1

else

  echo -e "${COLOR_GREEN}  [OK] - Connected to the internet.${NO_COLOR}";sleep 2
    
fi
