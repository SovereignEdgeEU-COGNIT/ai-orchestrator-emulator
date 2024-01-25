#!/bin/bash

blob="\e[1;30m[\e[0;32m+\e[1;30m]\e[0m"
bold="\e[1m"

configfile=$1
if [ "x$configfile" = "x" ]; then
	configfile=tc_config.txt
fi

if=eth0

tc qdisc del dev $if root > /dev/null 2>&1 # shut up about not having any rules

tc qdisc add dev $if root handle 1: htb default 14
tc class add dev $if parent 1: classid 1:0 htb rate 10000Mbps

myip=$(hostname -I)

item=1
handle=11
while read src dst what; do
	ok=0
	if [ $myip = $src ]; then
		ok=1
		target=$dst
	fi
	if [ $myip = $dst ]; then
		ok=1
		target=$src
	fi
	if [ $ok = 0 ]; then
		continue
	fi
	item=$(($item+1))
	handle=$(($handle+1))
	#echo src=$src dst=$dst target=$target what=$what
	case $what in
		*ms)
			tc class add dev $if parent 1:0 classid 1:$item htb rate 10000Mbps
			tc qdisc add dev $if parent 1:$item handle $handle: netem delay $what 5ms
			echo -e "$blob Adding a $what delay for packets to $target"
			;;
		*%)
			tc class add dev $if parent 1:0 classid 1:$item htb rate 10000Mbps
			tc qdisc add dev $if parent 1:$item handle $handle: netem loss random $what
			echo -e "$blob Adding a $what packet loss for packets to $target"
			;;

		*bit)
			tc class add dev $if parent 1:0 classid 1:$item htb rate $what
			echo -e "$blob Adding a $what bandwidth limitation for packets to $target"
			;;
	esac
	tc filter add dev $if parent 1: protocol ip prio 1 u32 \
        match ip dst $target/32 \
        match mark 0x10001 0xffff \
        flowid 1:$item
done < $configfile
