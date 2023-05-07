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

use utf8;
use JSON;
use File::Find;

if (@ARGV < 1){
  exit 1;
}
my $mra_root_dir = $ARGV[0];

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
use crate::object::*;
use crate::property::*;

impl Object {
    fn add_standard_property(&mut self, code: PropertyCode, name: String, data_type: String, data_size: usize, get_rule: PropertyRule, set_rule: PropertyRule, anno_rule: PropertyRule, enums: &mut Vec<PropertyEnum>) {
        let mut prop = Property::new();
        prop.set_code(code);
        prop.set_name(name);
        prop.set_data_type(data_type);
        prop.set_capacity(data_size);
        prop.set_read_attribute(get_rule);
        prop.set_write_attribute(set_rule);
        prop.set_anno_attribute(anno_rule);
        for e in enums.into_iter() {
            prop.add_enum(e.clone());
        }
        enums.clear();
        self.add_property(prop);
    }
}

fn property_string_to_attribute(attr: &str) -> PropertyRule {
    if attr == "required" {
        return PropertyRule::Required;
    }
    if attr == "optional" {
        return PropertyRule::Optional;
    }
    PropertyRule::Prohibited
}

impl StandardDatabase {
    fn create_standard_object(&mut self, cls_name: String, grp_code: u8, cls_code: u8) -> Object {
        let mut obj = Object::new();
        obj.set_class_group_code(grp_code);
        obj.set_class_code(cls_code);
        obj.set_class_name(cls_name);
        obj
    }

    fn create_standard_property_enum(&mut self, code: PropertyEnumCode, name: String, desc: String) -> PropertyEnum {
        let mut e = PropertyEnum::new();
        e.set_code(code);
        e.set_name(name);
        e.set_description(desc);
        e
    }

    pub fn init_objects(&mut self) {
      let mut prop_enums = Vec::new();
HEADER

my $mra_definitions_file = $mra_root_dir . "/mraData/definitions/definitions.json";
open(DEF_JSON_FILE, $mra_definitions_file) or die "Failed to open $mra_definitions_file: $!";
my $def_json_data = join('',<DEF_JSON_FILE>);
close(DEF_JSON_FILE);
my $def_json = decode_json($def_json_data);
my $def_json_root = %{$def_json}{'definitions'};

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
    my $data = %{$prop}{'data'};
    my $data_type = %{$data}{'type'};
    my $data_size = %{$data}{'size'};
    my $data_ref = %{$data}{'$ref'};
    if (0< length($data_ref)) {
      my @data_refs = split(/\//, $data_ref);
      my $data_ref_len = @data_refs;
      my $data_ref_id = $data_refs[$data_ref_len -1];
      my $prop_def = %{$def_json_root}{$data_ref_id};
      $data_type = %{$prop_def}{'type'};  
      $data_size = %{$prop_def}{'size'};
      my $enums = %{$prop_def}{'enum'};
      if (0 < @data_refs) {
        foreach $enum(@{$enums}) {
          my $edt = %{$enum}{'edt'};
          my $name = %{$enum}{'name'};
          my $descs = %{$enum}{'descriptions'};
          my $desc = %{$descs}{'en'};
          if (0< length($edt)) {
            printf("        prop_enums.push(self.create_standard_property_enum(%s, \"%s\".to_string(), \"%s\".to_string()));\n", 
            $edt,
            $name,
            $desc,
            );
          }
        }
      }
    }
    printf("        obj.add_standard_property(%s, \"%s\".to_string(), \"%s\".to_string(), %d, property_string_to_attribute(\"%s\"), property_string_to_attribute(\"%s\"), property_string_to_attribute(\"%s\"), &mut prop_enums);\n",
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