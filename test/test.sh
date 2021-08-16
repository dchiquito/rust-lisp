#!/bin/sh

rm -rf target/*
mkdir -p target/racket/src/
mkdir -p target/rust_lisp/

test_racket () {
    # Racket files must start with #lang racket
    echo "#lang racket\n" > target/racket/src/$1
    cat src/$1 >> target/racket/src/$1
    # echo "racket target/racket/src/$1 > target/racket/$1"
    racket target/racket/src/$1 > target/racket/$1
}

test_rust_lisp () {
    cd ..
    cargo run -- test/src/$1 > test/target/rust_lisp/$1 2>/dev/null
    cd test
}

test_file () {
    echo "Testing $1"
    test_racket $1
    test_rust_lisp $1
    diff target/racket/$1 target/rust_lisp/$1
}

test_all () {
    cd src
    TEST_FILES=`echo *.lisp`
    cd ..
    for file in $TEST_FILES
    do
        test_file $file
    done
}

test_all