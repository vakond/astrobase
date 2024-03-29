#!/usr/bin/env bash
# Smoke test.
# Requires Rust installed.

bin=./target/release
srv="astrobase-server"
cli="cli"
db="/tmp/astrobase.db"

if [ "$#" -ne 1 ]; then
    echo "Error: no argument"
    echo "Usage:"
    echo "    smoke-test.sh inmemory"
    echo "or"
    echo "    smoke-test.sh persistent"
    exit $result
fi

echo
echo "Building..."
cargo build --release --no-default-features --features $1
result=$?
echo "Result: $result"
if [ $result -ne 0 ]; then
    killall $srv
    echo "FAIL"
    exit $result
fi

rm -f $db

echo
echo "Starting server..."
$bin/$srv run &

echo
echo "Starting client..."
sleep 1s
$bin/$cli insert smoke test
result=$?
echo "Result: $result"
if [ $result -ne 0 ]; then
    sleep 1s
    killall $srv
    echo "FAIL"
    exit $result
fi

echo
echo "Waiting 2 seconds..."
sleep 2s
echo "Stopping server..."
killall $srv

echo "OK"
