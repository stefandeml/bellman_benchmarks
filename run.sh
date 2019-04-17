#!/bin/bash
#for i in 192 384 768 1536 3072 6144 12288
#for i in 24576 49152 98304 196608
for i in 3 5 10
do
  echo "Start run with $i bytes input"
  cargo run --release  --bin groth16 384 $i &> cargo_log_$i.txt &
  sleep 5
  pid=$(ps auxh | awk -v max=0 '{if($3>max){CPU=$3; PID=$2; NAME=$11; max=$3}} END{printf "%6d",PID}')
  python ps_mem.py -p $pid -w 10 > mem_log_$i.txt
done
