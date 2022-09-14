#!/usr/bin/perl
# Copyright (C) 2018 The uecho-rs Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.

if (@ARGV < 1){
  exit 1;
}
my $manlist_filename = $ARGV[0];

print<<HEADER;
// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::database::StandardDatabase;
use crate::manufacture::*;

impl StandardDatabase {
    pub fn add_standard_manufacture(&mut self, code: ManufactureCode, name: String) -> Manufacture {
        let mut m = Manufacture::new();
        m.set_code(code);
        m.set_name(name);
        m
    }

    pub fn init_manufactures(&mut self) {
HEADER

open(MANLIST, $manlist_filename) or die "$!";
while(<MANLIST>){
  chomp($_);
  $_ =~ s/(['"].*?['"])/(my $s = $1) =~ tr|,|=|; $s/eg;
  my @row = split(/(?!"),/, $_, -1);;
  my $code = $row[0];
  if (length($code ) != 6) {
    next;
  }
  my $name = $row[1];
  $name =~ s/=/,/g;
  $name =~ s/　/ /g; # converts zenkaku spaces to spaces
  if ($name !~ /^\"/) {
    $name = "\""  . $name
  }
  if ($name !~ /\"$/) {
    $name = $name . "\"" 
  }
  printf("        self.add_standard_manufacture(0x$code, String::from($name));\n");
}
close(MANLIST);
print<<FOTTER;
    }
}
FOTTER