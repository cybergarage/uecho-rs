#!/usr/bin/perl
# Copyright (C) 2018 The uecho-rs Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.


use utf8;
use JSON;
use File::Find;

if (@ARGV < 1){
  exit 1;
}
my $mra_root_dir = $ARGV[0];

print<<HEADER;
// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::database::StandardDatabase;
use crate::object::*;
use crate::property::*;

impl Object {
    fn add_standard_property(&mut self, code: PropertyCode, name: String, data_type: String, data_size: usize, get_rule: String, set_rule: String, anno_rule: String) {
        let mut prop = Property::new();
        prop.set_name(name);
        self.add_property(prop);
    }
}

impl StandardDatabase {
    fn create_standard_object(&mut self, cls_name: String, grp_code: u8, cls_code: u8) -> Object {
        let mut obj = Object::new();
        obj.set_class_group_code(grp_code);
        obj.set_class_code(cls_code);
        obj.set_class_name(cls_name);
        obj
    }

    pub fn init_objects(&mut self) {
HEADER

my @mra_sub_dirs = (
  "/mraData/superClass/",
  "/mraData/nodeProfile/",
  "/mraData/devices/"
);

my @device_json_files;
foreach my $mra_sub_dir(@mra_sub_dirs){
  my $mra_root_dir = $mra_root_dir . $mra_sub_dir;
  find sub {
      my $file = $_;
      my $path = $File::Find::name;
      if(-f $file){
        push(@device_json_files, $path);
      }
  }, $mra_root_dir;
}

foreach my $device_json_file(@device_json_files){
  open(DEV_JSON_FILE, $device_json_file) or die "$!";
  my $device_json_data = join('',<DEV_JSON_FILE>);
  close(DEV_JSON_FILE);
  my $device_json = decode_json($device_json_data);

  my $cls_names = %{$device_json}{'className'};
  my $cls_name = %{$cls_names}{'en'};
  my $grp_cls_code = %{$device_json}{'eoj'};
  my $grp_code = substr($grp_cls_code, 2, 2);
  my $cls_code = substr($grp_cls_code, 4);
  printf("        // %s (0x%s%s)\n", $cls_name, $grp_code, $cls_code);
  printf("        let mut obj = self.create_standard_object(\"%s\".to_string(), 0x%s, 0x%s);\n", $cls_name, $grp_code, $cls_code);

  my $props = %{$device_json}{'elProperties'};
  foreach $prop(@{$props}) {
    my $epc = %{$prop}{'epc'};
    my $names = %{$prop}{'propertyName'};
    my $name = %{$names}{'en'};
    my $rules = %{$prop}{'accessRule'};
    my $get_rule = %{$rules}{'get'};
    my $set_rule = %{$rules}{'set'};
    my $anno_rule = %{$rules}{'inf'};
    my $data_type = "";
    printf("        obj.add_standard_property(%s, \"%s\".to_string(), \"%s\".to_string(), %d, \"%s\".to_string(), \"%s\".to_string(), \"%s\".to_string());\n",
      $epc,
      $name,
      $data_type,
      $data_size,
      $get_rule,
      $set_rule,
      $anno_rule,
      );
   }
  printf("        self.add_object(obj);\n\n", $grp_code, $cls_code);
}
print<<FOTTER;
    }
}
FOTTER