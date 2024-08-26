#!/bin/bash


code_http=$(curl --write-out %{http_code} --silent --output /dev/null  https://www.google.com/)
echo $code_http
