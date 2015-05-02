#!/usr/bin/env perl

print "Hello, what's your name?\n";
print "> ";
my $name = <STDIN>;
chomp($name);
print "Nice to meet you, $name.\n"
