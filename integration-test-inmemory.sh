#!/bin/bash
# Integration testing for astrobase-server with in-memory database.
# Requires Rust installed.

srv="astrobase-server"
cli="cli"
bin=./target/release
cfg="/tmp/astrobase-integration-testing.json"
out="/tmp/astrobase-server.out"

function check_exit {
    result=$?
    #echo "Result: $result"
    if [ $result -ne 0 ]; then
	killall $srv
	echo "FAIL"
	exit $result
    fi
}

function check_substring {
    haystack=$1
    needle=$2
    if [[ $haystack != *$needle* ]]; then
	echo "FAIL: $1"
	echo "SHOULD CONTAIN: $2"
	killall $srv
	exit
    fi
}

function check_output {
    nr=$1
    counter=$2
    sleep 1s

    # ensure full line is dumped
    i=0
    stats=""
    stats_ensure="some"
    while [[ $i<10 && $stats != $stats_ensure ]]
    do
        sleep 0.1s
	stats=$(tail -1 $out)
	stats_ensure=$(tail -1 $out)
	((i=i+1))
    done
    #echo "i=$i"

    if [[ $i>=10 ]]
    then
	echo "Wrong output: $stats"
	killall $srv
	exit
    fi

    check_substring "$stats" "$nr"
    check_substring "$stats" "$counter"
}

function build {
    echo
    echo "Building..."
    cargo build --quiet --release --no-default-features --features inmemory
    check_exit
}

function start_server {
    echo
    echo "Starting server..."
    cat << EOF > $cfg
{
    "environment": "integration-testing",
    "server": {
	"endpoint": "[::1]:50051"
    },
    "monitoring": {
	"interval": 1
    }
}
EOF
    $bin/$srv --config $cfg run 2>$out &
    sleep 1s
}

function stop_server {
    echo
    echo "Stopping server..."
    killall $srv
}

function test_successful_insert {
    echo
    echo "test_successful_insert"
    $bin/$cli insert smoke test
    check_exit
    check_output "NR:1" "INSERT(ok/fail):(1, 0)"
}

function test_failing_insert {
    echo
    echo "test_failing_insert"
    $bin/$cli insert smoke test
    check_exit
    check_output "NR:1" "INSERT(ok/fail):(1, 1)"
}

function test_successful_get {
    echo
    echo "test_successful_get"
    $bin/$cli get smoke
    check_exit
    check_output "NR:1" "GET(ok/fail):(1, 0)"
}

function test_failing_get {
    echo
    echo "test_failing_get"
    $bin/$cli get garbage
    check_exit
    check_output "NR:1" "GET(ok/fail):(1, 1)"
}

function test_successful_update {
    echo
    echo "test_successful_update"
    $bin/$cli update smoke "on the water"
    check_exit
    check_output "NR:1" "UPDATE(ok/fail):(1, 0)"
}

function test_failing_update {
    echo
    echo "test_failing_update"
    $bin/$cli update garbage garbage
    check_exit
    check_output "NR:1" "UPDATE(ok/fail):(1, 1)"
    $bin/$cli update smoke "on the water"
    check_exit
    check_output "NR:1" "UPDATE(ok/fail):(1, 2)"
}

function test_successful_delete {
    echo
    echo "test_successful_delete"
    $bin/$cli insert brick wall
    check_exit
    check_output "NR:2" "INSERT(ok/fail):(2, 1)"
    $bin/$cli delete smoke
    check_exit
    check_output "NR:1" "DELETE(ok/fail):(1, 0)"
}

function test_failing_delete {
    echo
    echo "test_failing_delete"
    $bin/$cli delete garbage
    check_exit
    check_output "NR:1" "DELETE(ok/fail):(1, 1)"
}

build
start_server

test_successful_insert
test_failing_insert

test_successful_get
test_failing_get

test_successful_update
test_failing_update

test_successful_delete
test_failing_delete

stop_server

echo "OK"
