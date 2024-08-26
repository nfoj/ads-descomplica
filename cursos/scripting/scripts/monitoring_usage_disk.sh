#!/bin/bash

path_mont="/mnt/c"

monitoring_disk=$(df -h | grep $path_mont | awk '{print $5}')
echo "Disk Usage Monitoring: $monitoring_disk"
