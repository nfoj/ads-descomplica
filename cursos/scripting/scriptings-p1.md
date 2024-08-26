# Scripting: automatizando tarefas com Bash e Docker


## Verification Connect Page

```
  
  // Commands
  sudo apt-get install curl
  curl --write-out %{http_code} --silent --output /dev/null  https://www.google.com/
  
  // Code 200 = Ok!
  
```

- Script 

```

  #!/bin/bash
  code_http=$(curl --write-out %{http_code} --silent --output /dev/null  https://www.google.com/)
  echo $code_http

    
```


## Disk Usage Monitoring

```

  // List disk usage
  df -h 

  // List disk usage path
  df -h | grep /mnt/c 

  // 
  df -h | grep /mnt/c | awk '{print $5}'
  
```

- Script

```

  #!/bin/bash

  path_mont="/mnt/c"
  
  monitoring_disk=$(df -h | grep $path_mont | awk '{print $5}')
  echo "Disk Usage Monitoring: $monitoring_disk"
  
```


## Crontab

```

  // Active
  sudo service cron start

  // Verification 
  service cron status
  
  // sudo crontab -e
  # m h dom mon dow command_path path_exit
  */2 * * /home/user/script.sh /home/user

  m: minutes (0-59)
  h: hour (0-23)
  dom: days (1-31)
  mon: months (1-12 or JAN, FEB ...)
  dow: week (0-7, Sunday)
  command: run

  Example:

  Run a command on the first day of each month:

  0 0 1 * * command

  Run a command every Monday at 9 AM:

  0 9 * * 1 command
  
  
```
