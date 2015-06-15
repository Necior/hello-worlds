#!/usr/bin/env clojure

(print
    "Hello, what's your name?\n> ")
(flush)
(let
    [username (read-line)]
    (print "Nice to meet you, ")
    (print username)
    (print ".\n"))
