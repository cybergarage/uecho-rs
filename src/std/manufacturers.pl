#!/usr/bin/perl
# Copyright (C) 2022 The uecho-rs Authors All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

if (@ARGV < 1){
  exit 1;
}
my $manlist_filename = $ARGV[0];

print<<HEADER;
// Copyright (C) 2022 The uecho-rs Authors All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::database::StandardDatabase;
use crate::manufacture::*;

impl StandardDatabase {
    fn add_standard_manufacture(&mut self, code: ManufactureCode, name: String) {
        let mut m = Manufacture::new();
        m.set_code(code);
        m.set_name(name);
        self.add_manufacture(m);
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
printf("        self.add_standard_manufacture(0xFFFFFF, String::from(\"Experimental\"));\n");
printf("        self.add_standard_manufacture(0xFFFFFE, String::from(\"Undefined\"));\n");
close(MANLIST);
print<<FOTTER;
    }
}
FOTTER