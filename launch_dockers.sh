#!/bin/bash
docker rm -f test-postgres test-mysql

pids=()

docker run --name test-postgres --volume "/var/lib/postgresql/data:/var/lib/postgresql/data" -e POSTGRES_PASSWORD=mysecretpassword -d -p 32768:5432 postgres & pids+=($!)
docker run --name test-mysql --volume "/var/lib/mysql:/var/lib/mysql" -e MYSQL_ROOT_PASSWORD=mysecretpassword -d -p 32769:3306 mysql & pids+=($!)

for pid in ${pids[@]}; do
	wait $pid
done

echo -e "\n"
docker ps --format "{{.Image}}\t{{.Ports}}"
