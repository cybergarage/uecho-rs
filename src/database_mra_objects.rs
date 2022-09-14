// Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::database::StandardDatabase;
use crate::object::*;
use crate::property::*;

impl Object {
    fn add_standard_property(
        &mut self,
        code: PropertyCode,
        name: String,
        data_type: String,
        data_size: usize,
        get_rule: String,
        set_rule: String,
        anno_rule: String,
    ) {
        let mut prop = Property::new();
        self.add_property(prop);
    }
}

impl StandardDatabase {
    fn create_standard_object(&mut self, cls_name: String, grp_code: u8, cls_code: u8) -> Object {
        let mut obj = Object::new();
        // obj.set_code(code);
        // obj.set_name(name);
        obj
    }

    pub fn init_objects(&mut self) {
        // Super class (0x0000)
        let mut obj = self.create_standard_object("Super class".to_string(), 0x00, 0x00);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x81,
            "Installation location".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x81,
            "Installation location".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x82,
            "Standard version information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x83,
            "Identification number".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x83,
            "Identification number".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x84,
            "Measured instantaneous power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x85,
            "Measured cumulative power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x86,
            "Manufacturer's fault code".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x87,
            "Current limit setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x88,
            "Fault status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x89,
            "Fault description".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8A,
            "Manufacturer code".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8B,
            "Business facility code".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8C,
            "Product code".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8D,
            "Production number".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8E,
            "Production date".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8F,
            "Power-saving operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x93,
            "Location information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x93,
            "Location information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x93,
            "Remote control setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x93,
            "Remote controll setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x97,
            "Current time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x98,
            "Current date setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x99,
            "Power limit setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9A,
            "Cumulative operating time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9D,
            "Status change announcement property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9E,
            "Set property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9F,
            "Get property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Node profile (0x0EF0)
        let mut obj = self.create_standard_object("Node profile".to_string(), 0x0E, 0xF0);
        obj.add_standard_property(
            0x80,
            "Operating status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x82,
            "Version information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x83,
            "Identification number".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x88,
            "Fault status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x89,
            "Fault description".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8A,
            "Manufacturer code".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8B,
            "Business facility code".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8C,
            "Product code".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8D,
            "Production number".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x8E,
            "Production date".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9D,
            "Status change announcement property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9E,
            "Set property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x9F,
            "Get property map".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Unique identifier data".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Number of self-node instances".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Number of self-node classes".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Instance list notification".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Self-node instance list S".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Self-node class list S".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Hybrid water heater (0x02A6)
        let mut obj = self.create_standard_object("Hybrid water heater".to_string(), 0x02, 0xA6);
        obj.add_standard_property(
            0xB0,
            "Automatic water heating setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Water heating status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Heater status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Hot water supply mode setting for auxiliary heat source machine".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB7,
            "Heater mode setting for auxiliary heat source machine.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB8,
            "Linkage mode setting for solar power generation".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB9,
            "Solar power generations utilization time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Hot water supply status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measured amount of hot water remaining in tank".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Tank capacity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Switch (supporting JEM-A/HA terminals) (0x05FD)
        let mut obj = self.create_standard_object(
            "Switch (supporting JEM-A/HA terminals)".to_string(),
            0x05,
            0xFD,
        );
        obj.add_standard_property(
            0xE0,
            "Connected device".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Instantaneous water heater (0x0272)
        let mut obj =
            self.create_standard_object("Instantaneous water heater".to_string(), 0x02, 0x72);
        obj.add_standard_property(
            0x90,
            "ON timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "Set value of ON timer relative time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Hot water heating status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Set value of hot water temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Hot water warmer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Bath water volume setting 4".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Bath water volume setting 4 Maximum settable level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Volume setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Mute setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Duration of Automatic operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Remaining Automatic operation time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Set value of bath temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Bath water heater status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Bath Auto mode setting".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "required_o".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Bath additional boil-up operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Bath hot water adding operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Bath water temperature lowering operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Bath hot water volume setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Bath hot water volume setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Bathroom priority setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Bathroom priority setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Shower hot water supply status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Kitchen hot water supply status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Hot water warmer ON timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Set value of hot water warmer ON timer time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Bath hot water volume setting 3".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Bath operation status monitor".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        self.add_object(obj);

        // Commercial showcase (0x03CE)
        let mut obj = self.create_standard_object("Commercial showcase".to_string(), 0x03, 0xCE);
        obj.add_standard_property(
            0xB0,
            "Operation mode".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xBD,
            "Used to acquire measurements of discharge temperature.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "This property indicates the type of the showcase.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "This property indicates the type of the showcase door.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "This property indicates refrigerator type, such as built-in or separate.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "This property indicates the shape of the showcase.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xD4, "This property indicates the purpose of the showcase, either refrigeration or freezing.".to_string(), "".to_string(), 0, "required".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE0,
            "Indicates on/off status of lighting installed inside the showcase.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Indicates ON/OFF status of lighting installed outside the showcase.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Indicates on/off status of compressor when showcase and compressor are a single unit."
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Used to acquire internal temperature measurements inside the showcase.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Indicates rated power consumption necessary when showcase is cooling.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE5, "Indicates rated power consumption when heater is operating during showcase defrosting.".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE6,
            "Indicates rated power consumption when showcase is operating fan motor.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Indicates on/off status of showcases with heater for hot function.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Indicates type of lighting installed inside the showcase.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Indicates type of lighting installed outside the showcase.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Indicates lighting level in % installed inside of the showcase.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Indicates lighting level in % installed outside of the showcase.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Set temperature setting of inside the case and acquire the current setting."
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Illuminance sensor (0x00D0)
        let mut obj = self.create_standard_object("Illuminance sensor".to_string(), 0x00, 0xD0);
        obj.add_standard_property(
            0xE0,
            "Measured illuminance value 1".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measured illuminance value 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Television (0x0602)
        let mut obj = self.create_standard_object("Television".to_string(), 0x06, 0x02);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required_o".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Display control setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Character string setting acceptance status".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Supported character codes".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Character string to present to the user".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Length of character string accepted".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // EV charger and discharger (0x027E)
        let mut obj =
            self.create_standard_object("EV charger and discharger".to_string(), 0x02, 0x7E);
        obj.add_standard_property(
            0xC0,
            "Dischargeable capacity of vehicle mounted battery 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Dischargeable capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Dischargeable capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Remaining dischargeable capacity of vehicle mounted battery 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Remaining dischargeable capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Remaining dischargeable capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Remaining dischargeable capacity of vehicle mounted battery 3".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC5,
            "Rated charge capacity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Rated discharge capacity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Vehicle connection and chargeable/dischargeable status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Vehicle connection and chargeable/dischargeable status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Vehicle connection and chargeable/dischargeable status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Minimum/maximum charging electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Minimum/maximum discharging electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Minimum/maximum charging current".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Minimum/maximum discharging current".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Charger/Discharger type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Vehicle connection confirmation".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "required".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Chargeable capacity of vehicle mounted battery".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Remaining chargeable capacity of vehicle mounted battery".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Used capacity of vehicle mounted battery 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Used capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Used capacity of vehicle mounted battery 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Rated voltage".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measured instantaneous charging/discharging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Measured instantaneous charging/discharging current".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Measured instantaneous charging/discharging voltage".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Measured cumulative amount of discharging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Cumulative amount of discharging electric energy reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Measured cumulative amount of charging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Cumulative amount of charging electric energy reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "System interconnected type".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Charging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Charging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Discharging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Discharging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDE,
            "Purchasing electric power setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDF,
            "Re-interconnection permission setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Charging/Discharging electric power setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Remaining stored electricity of vehicle mounted battery1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Remaining stored electricity of vehicle mounted battery2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Remaining stored electricity of vehicle mounted battery2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Remaining stored electricity of vehicle mounted battery3".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Maintenance status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Vehicle ID".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Charging amount setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Charging amount setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Discharging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Charging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Discharging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Charging current setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Discharging current setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Rated voltage (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Package-type commercial air conditioner (outdoor unit) (0x0157)
        let mut obj = self.create_standard_object(
            "Package-type commercial air conditioner (outdoor unit)".to_string(),
            0x01,
            0x57,
        );
        obj.add_standard_property(
            0xAB,
            "Special state".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB8,
            "Rated power consumption of outdoor unit".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Measured outdoor unit temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Measured power consumption of outdoor unit".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Possible power savings for outdoor units".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDE,
            "Settings restricting power consumption of outdoor units".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDF,
            "Minimum power consumption for restricted outdoor unit".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Humidity sensor (0x0012)
        let mut obj = self.create_standard_object("Humidity sensor".to_string(), 0x00, 0x12);
        obj.add_standard_property(
            0xE0,
            "Measured value of relative humidity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Mono functional lighting (0x0291)
        let mut obj =
            self.create_standard_object("Mono functional lighting".to_string(), 0x02, 0x91);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Light level Setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Power distribution board metering (0x0287)
        let mut obj = self.create_standard_object(
            "Power distribution board metering".to_string(),
            0x02,
            0x87,
        );
        obj.add_standard_property(
            0xB0,
            "Master rated capacity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Number of measurement channels (simplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xB2, "Channel range specification for cumulative amount of electric power consumption measurement (simplex)".to_string(), "".to_string(), 0, "optional".to_string(), "optional".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xB3,
            "Measured cumulative amount of electric power consumption list (simplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Channel range specification for instantaneous current measurement (simplex)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Measured instantaneous current list (simplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Channel range specification for instantaneous power consumption measurement (simplex)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB7,
            "Measured instantaneous power consumption list (simplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB8,
            "Number of measurement channels (duplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xB9, "Channel range specification for cumulative amount of electric power consumption measurement (duplex)".to_string(), "".to_string(), 0, "optional".to_string(), "optional".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xBA,
            "Measured cumulative amount of electric power consumption list (duplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Channel range specification for instantaneous current measurement (duplex)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBC,
            "Measured instantaneous current list (duplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBD,
            "Channel range specification for instantaneous power consumption measurement (duplex)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Measured instantaneous power consumption list (duplex)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Measured cumulative amount of electric energy (normal direction)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Measured cumulative amount of electric energy (reverse direction)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Unit for cumulative amounts of electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Historical data of measured cumulative amounts of electric energy (normal direction)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Historical data of measured cumulative amounts of electric energy (reverse direction)"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xC5, "Day for which the historical data of measured cumulative amounts of electric energy is to be retrieved".to_string(), "".to_string(), 0, "optional".to_string(), "optional".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xC6,
            "Measured instantaneous amount of electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Measured instantaneous currents".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Measured instantaneous voltages".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Measurement channel 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Measurement channel 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Measurement channel 3".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measurement channel 4".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Measurement channel 5".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Measurement channel 6".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Measurement channel 7".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Measurement channel 8".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Measurement channel 9".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Measurement channel 10".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Measurement channel 11".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Measurement channel 12".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Measurement channel 13".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Measurement channel 14".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDE,
            "Measurement channel 15".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDF,
            "Measurement channel 16".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Measurement channel 17".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measurement channel 18".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Measurement channel 19".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measurement channel 20".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Measurement channel 21".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Measurement channel 22".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Measurement channel 23".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Measurement channel 24".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Measurement channel 25".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Measurement channel 26".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Measurement channel 27".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Measurement channel 28".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Measurement channel 29".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Measurement channel 30".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Measurement channel 31".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Measurement channel 32".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Electric water heater (0x026B)
        let mut obj = self.create_standard_object("Electric water heater".to_string(), 0x02, 0x6B);
        obj.add_standard_property(
            0x90,
            "ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Automatic water heating setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Automatic water heating setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Automatic water temperature control setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Water heater status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Water heater status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Water heating temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Manual water heating stop days setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Relative time setting value for manual water heating OFF".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Tank operation mode setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Daytime reheating permission setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Daytime reheating permission setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Measured temperature of water in water heater".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Alarm status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Hot water supply status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Relative time setting for keeping bath temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Participation in energy shift".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Standard time to start heating".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Number of energy shifts".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Daytime heating shift time 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Expected electric energy at daytime heating shift time 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Consumption of electric energy per hour 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Daytime heating shift time 2".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Expected electric energy at daytime heating shift time 2".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Consumption of electric energy per hour 2".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Temperature of supplied water setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Bath water temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Bath water volume setting4".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Bath water volume setting4 maximum settable level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Volume setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Mute setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Remaining hot water volume".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Surplus electric energy power prediction value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Rated power consumption of H/P unit in wintertime".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Rated power consumption of H/P unit in in-between seasons".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Rated power consumption of H/P unit in summertime".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Bath water volume setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measured amount of water remaining in tank".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Tank capacity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Automatic Bath Water Heating Mode Setting".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "required_o".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Manual bath reheating operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Addition of hot water function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Manual bath hot water addition function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Slight bath water temperature lowering function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Manual lukewarm water temperature lowering function setting.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Bath water volume setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Bath water volume setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Bathroom priority setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Bath Operation Status Monitor".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Bath water volume setting 3".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // General lighting (0x0290)
        let mut obj = self.create_standard_object("General lighting".to_string(), 0x02, 0x90);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x90,
            "ON timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x94,
            "OFF timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x95,
            "Time set by OFF timer".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Light level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Light color setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Light color setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Light color setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Light level step setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Light color step setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Maximum specifiable values".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Maximum value of settable level for night lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Lighting mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB7,
            "Light level setting for main lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB8,
            "Light level step setting for main lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB9,
            "Light level setting for night lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBA,
            "Light level step setting for night lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Light color setting for main lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBC,
            "Light color level step setting for main lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBD,
            "Light color setting for night lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Light color level step setting for night lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Lighting mode status in auto mode".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "RGB setting for color lighting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Storage battery (0x027D)
        let mut obj = self.create_standard_object("Storage battery".to_string(), 0x02, 0x7D);
        obj.add_standard_property(
            0x83,
            "Identification number".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x97,
            "Current time setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x98,
            "Current date setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "AC effective capacity (charging)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA1,
            "AC effective capacity (discharging)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA2,
            "AC chargeable capacity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA3,
            "AC dischargeable capacity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA4,
            "AC chargeable electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA5,
            "AC dischargeable electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA6,
            "AC charge upper limit setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA7,
            "AC discharge lower limit setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA8,
            "AC measured cumulative charging electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA9,
            "AC measured cumulative discharging electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xAA,
            "AC charge amount setting value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xAB,
            "AC discharge amount setting value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Charging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Discharging method".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "AC rated electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Minimum/maximum charging electric power".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Minimum/maximum charging electric power".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Minimum/maximum discharging electric power".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Minimum/maximum discharging electric power".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Minimum/maximum charging current".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Minimum/maximum discharging current".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Re-interconnection permission setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Operation permission setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Independent operation permission setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Working operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Working operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Rated electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Rated capacity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Rated voltage".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measured instantaneous charging/discharging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Measured instantaneous charging/discharging current".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Measured instantaneous charging/discharging voltage".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Measured cumulative discharging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Measured cumulative discharging electric energy reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Measured cumulative charging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Measured cumulative charging electric energy reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "System-interconnected type".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "System-interconnected type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Minimum/maximum charging power (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDD,
            "Minimum/maximum discharging power (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDE,
            "Minimum/maximum charging current (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDF,
            "Minimum/maximum discharging current (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Charging/discharging amount setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Charging/discharging amount setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Charging/discharging amount setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Charging/discharging amount setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Remaining stored electricity 1".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Remaining stored electricity 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Remaining stored electricity 3".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Battery state of health".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Battery type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Charging amount setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Discharging amount setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Charging amount setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Discharging amount setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Charging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Discharging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Charging current setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Discharging current setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Rated voltage (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Package-type commercial air conditioner (indoor unit) (except those for facilities) (0x0156)
        let mut obj = self.create_standard_object(
            "Package-type commercial air conditioner (indoor unit) (except those for facilities)"
                .to_string(),
            0x01,
            0x56,
        );
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xAC,
            "Thermostat state".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xAE,
            "Current function (automatic operation mode)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Temperature setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Measured indoor unit temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Power consumption range for indoor units".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Rice cooker (0x03BB)
        let mut obj = self.create_standard_object("Rice cooker".to_string(), 0x03, 0xBB);
        obj.add_standard_property(
            0x90,
            "Rice cooking reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "Rice cooking reservation time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "Rice cooking reservation relative time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Cover closure status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Rice cooking status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Rice cooking control setting".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "required_o".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Warmer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Inner pot removal status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Cover removal status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // High-voltage smart electric energy meter (0x028A)
        let mut obj = self.create_standard_object(
            "High-voltage smart electric energy meter".to_string(),
            0x02,
            0x8A,
        );
        obj.add_standard_property(
            0xC1,
            "Monthly maximum electric power demand".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Cumulative maximum electric power demand".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Electric power demand at fixed time (30-minute average electric power)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Number of effective digits of electric power demand".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC5,
            "Unit of electric power demand".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Historical data of measured electric power demand".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Unit of cumulative maximum electric power demand".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xCA, "Measurement data of reactive electric power consumption (lag) for power factor measurement".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xCB, "Measurement data of cumulative amount of reactive electric power consumption (lag) at fixed time for power factor measurement".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xCC, "Number of effective digits for measurement data of cumulative amount of reactive electric power consumption (lag) for power factor measurement".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xCD, "Unit of measurement data of cumulative amount of reactive electric power consumption (lag)".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xCE, "Historical data of measurement data of cumulative amount of reactive electric power consumption (lag) for power factor measurement".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xD3,
            "Coefficient".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Multiplying factor for coefficient".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Fixed date".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE1, "Day for which the historical data of measured cumulative amounts of electric energy is to be retrieved".to_string(), "".to_string(), 0, "required".to_string(), "required".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE2,
            "Measured cumulative amounts of active electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Cumulative amounts of active electric energy at fixed time".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE4, "Measurement data of cumulative amounts of active electric energy for power factor measurement".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE5,
            "Number of effective digits for cumulative amount of active electric energy"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Unit of cumulative amounts of effective electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Historical data of measured cumulative amount of active electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Cooking heater (0x03B9)
        let mut obj = self.create_standard_object("Cooking heater".to_string(), 0x03, 0xB9);
        obj.add_standard_property(
            0x96,
            "Relative time settings of off timers".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA1,
            "Child lock setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA2,
            "Radiant heater lock setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Heating status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Heating setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "All stop setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "required_o".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Heating modes of stoves".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Heating temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Heating power setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // VOC sensor (0x001D)
        let mut obj = self.create_standard_object("VOC sensor".to_string(), 0x00, 0x1D);
        obj.add_standard_property(
            0xB0,
            "Detection threshold level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "VOC detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Measured value of VOC concentration".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Bathroom heater dryer (0x0273)
        let mut obj = self.create_standard_object("Bathroom heater dryer".to_string(), 0x02, 0x73);
        obj.add_standard_property(
            0x90,
            "On timer reservation setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "On timer setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "On relative timer setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x94,
            "Off timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x95,
            "OFF timer setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x96,
            "Off relative timer setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Ventilation operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Bathroom prewarming operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Bathroom prewarming operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Bathroom heating operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Bathroom drying operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Cool air circulation operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Mist sauna operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB7,
            "Water mist operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBA,
            "Measured value of bathroom relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Measured value of bathroom temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Ventilation air flow rate setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Filter cleaning reminder sign setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Human body detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "On timer reservation setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "On timer reservation setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Commercial show case outdoor unit (0x03D4)
        let mut obj = self.create_standard_object(
            "Commercial show case outdoor unit".to_string(),
            0x03,
            0xD4,
        );
        obj.add_standard_property(
            0xAA,
            "Indicates that the showcase freezer is in an exceptional status.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Used to acquire measurements of outdoor air temperature.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Group information".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Indicates compressor ON/OFF status.".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Fuel cell (0x027C)
        let mut obj = self.create_standard_object("Fuel cell".to_string(), 0x02, 0x7C);
        obj.add_standard_property(
            0xC1,
            "Measured temperature of water in water heater".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Rated power generation output".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Rated power generation output".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Heating value of hot water storage tank".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Measured instantaneous power generation output".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC5,
            "Measured cumulative power generation output".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Cumulative power generation output reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Cumulative energy generation output reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Measured instantaneous gas consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Measured cumulative gas consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Cumulative gas consumption reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Power generation setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Power generation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Power generation status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Power generation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Measured in-house instantaneous power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Measured in-house cumulative power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Measured in-house cumulative energy consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "In-house cumulative power consumption reset".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "In-house cumulative energy consumption reset".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "System interconnected type".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "System interconnected type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Power generation request time setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Designated power generation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measured remaining hot water amount".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Tank capacity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Water flowmeter (0x0281)
        let mut obj = self.create_standard_object("Water flowmeter".to_string(), 0x02, 0x81);
        obj.add_standard_property(
            0xD0,
            "Water flowmeter classification".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Owner classification".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Measured cumulative amount of flowing water".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Unit for measured Cumulative amounts of flowing water".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Historical data of measured cumulative amount of flowing water".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Detection of abnormal value in metering data".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Security data information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "ID number setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Verification expiration information".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Home air conditioner (0x0130)
        let mut obj = self.create_standard_object("Home air conditioner".to_string(), 0x01, 0x30);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x8F,
            "Power-saving operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x90,
            "ON timer-based reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "ON timer setting (time)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "ON timer setting (relative time)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x94,
            "OFF timer-based reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x95,
            "OFF timer setting (time)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x96,
            "OFF timer setting (relative time)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Air flow rate setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Air flow rate setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xA1,
            "Automatic control of air flow direction setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA3,
            "Automatic swing of air flow setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA4,
            "Air flow direction (vertical) setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA5,
            "Air flow direction (horizontal) setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xAA,
            "Special state".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xAB,
            "Non-priority state".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Automatic temperature control setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Normal/highspeed/silent operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Set temperature value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Set value of relative humidity in dehumidifying mode".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Set temperature value in cooling mode".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Set temperature value in heating mode".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB7,
            "Set temperature value in dehumidifying mode".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB8,
            "Rated power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB9,
            "Measured value of current consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBA,
            "Measured value of room relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Measured value of room temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBB,
            "Measured value of room temperature".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBC,
            "Set temperature value of user remote control".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBD,
            "Measured cooled air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Measured outdoor air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Relative temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Ventilation function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Humidifier function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Ventilation air flow rate setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Degree of humidification setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Mounted air cleaning method".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Air purifier function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Mounted air refresh method".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Air refresher function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Mounted self-cleaning method".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Self-cleaning function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Special function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Operation status of components".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Thermostat setting override function".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Thermostat setting override function".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Air purification mode setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Buzzer".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        self.add_object(obj);

        // Electric energy sensor (0x0022)
        let mut obj = self.create_standard_object("Electric energy sensor".to_string(), 0x00, 0x22);
        obj.add_standard_property(
            0xE0,
            "Cumulative amounts of electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Medium-capacity sensor instantaneous electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Small-capacity sensor instantaneous electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Large-capacity sensor instantaneous electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Cumulative amounts of electric energy measurement log".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Cumulative amounts of electric energy measurement log".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Effective voltage value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Washer and dryer (0x03D3)
        let mut obj = self.create_standard_object("Washer and dryer".to_string(), 0x03, 0xD3);
        obj.add_standard_property(
            0x90,
            "On timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "On timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "Relative time-based on timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Door/cover open/close status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Washer and dryer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Washer and dryer cycle setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Washer and dryer cycle setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Drying cycle setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Washer and dryer cycle option list 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Washer and dryer cycle option list 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Washer and dryer cycle option list 3".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD6,
            "Water flow rate setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Rotation speed for spin drying setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Degree of drying setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDB,
            "Remaining washing time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Remaining drying time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDF,
            "Elapsed time on the ON timer".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Presoaking time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Current stage of washer and dryer cycle".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Water volume setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Water volume setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Washing time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Number of times of rinsing setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Rinsing process setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Spin drying time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Drying time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Warm water setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Bathtub water recycle setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Wrinkling minimization setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Time remaining to complete washer and dryer cycle".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "Door/cover lock setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "Washer and dryer cycle".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Electrically operated rain sliding door/shutter (0x0263)
        let mut obj = self.create_standard_object(
            "Electrically operated rain sliding door/shutter".to_string(),
            0x02,
            0x63,
        );
        obj.add_standard_property(
            0x89,
            "Fault description (Recoverable faults)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x90,
            "Timer operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Opening speed setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Closing speed setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Operation time".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Open/close operation setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Degree-of-opening setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Blind angle setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Opening/closing speed setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Electric lock setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Remote operation setting status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Selective degree-of-opening setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Open/closed status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Slit degree-of-opening".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEE,
            "One-time opening speed setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEF,
            "One-time closing speed setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // CO2 sensor (0x001B)
        let mut obj = self.create_standard_object("CO2 sensor".to_string(), 0x00, 0x1B);
        obj.add_standard_property(
            0xE0,
            "Measured value of CO2 concentration".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Current sensor (0x0023)
        let mut obj = self.create_standard_object("Current sensor".to_string(), 0x00, 0x23);
        obj.add_standard_property(
            0xE0,
            "Measured current value 1".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Rated voltage to be measured".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Measured current value 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // EV Charger (0x02A1)
        let mut obj = self.create_standard_object("EV Charger".to_string(), 0x02, 0xA1);
        obj.add_standard_property(
            0xC5,
            "Rated charge capacity".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Vehicle connection and chargeable status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Vehicle connection and chargeable status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Minimum/maximum charging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Minimum/maximum charging electric current".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Charger type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Vehicle connection confirmation".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Chargeable capacity of vehicle mounted battery".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Remaining chargeable capacity of vehicle mounted battery".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Used capacity of vehicle mounted battery 1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Rated voltage".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measured instantaneous charging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Measured cumulative amount of charging electric energy".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Cumulative amount of charging electric energy reset setting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Remaining stored electricity of vehicle mounted battery1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Remaining stored electricity of vehicle mounted battery3".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Vehicle ID".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Charging amount setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Charging electric energy setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Charging current setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Watt-hour meter (0x0280)
        let mut obj = self.create_standard_object("Watt-hour meter".to_string(), 0x02, 0x80);
        obj.add_standard_property(
            0xE0,
            "Cumulative amounts of electric energy measurement value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Cumulative amounts of electric energy unit".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Cumulative amounts of electric energy measurement log 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Emergency button (0x0003)
        let mut obj = self.create_standard_object("Emergency button".to_string(), 0x00, 0x03);
        obj.add_standard_property(
            0xB1,
            "Emergency occurrence status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Emergency occurrence status resetting".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        self.add_object(obj);

        // Household solar power generation (0x0279)
        let mut obj =
            self.create_standard_object("Household solar power generation".to_string(), 0x02, 0x79);
        obj.add_standard_property(
            0x83,
            "Identification number".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x97,
            "Current time setting".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x98,
            "Current date setting".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Output power control setting 1".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA1,
            "Output power control setting 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA2,
            "Function to control purchase surplus electricity setting".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Output power controlling schedule".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Next access date and time".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Function to control the type of surplus electricity purchase".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Output power change time setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Upper limit clip setting value".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Operation power factor setting value".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "FIT contract type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Self-consumption type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Capacity approved by equipment".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Conversion coefficient".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "System-interconnected type".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "System-interconnected type".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Output power restraint status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Measured instantaneous amount of electricity generated".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Measured cumulative amount of electric energy generated".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Resetting cumulative amount of electric energy generated".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measured cumulative amount of electric energy sold".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Resetting cumulative amount of electric energy sold".to_string(),
            "".to_string(),
            0,
            "notApplicable".to_string(),
            "optional".to_string(),
            "notApplicable".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Power generation output limit setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Power generation output limit setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Limit setting for the amount of electricity sold".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Rated power generation output (System-interconnected)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Rated power generation output (System-interconnected)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Rated power generation output (Independent)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Floor heater (0x027B)
        let mut obj = self.create_standard_object("Floor heater".to_string(), 0x02, 0x7B);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0x90,
            "ON timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "Time set by ON timer".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "Relative ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x94,
            "OFF timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x95,
            "Time set by OFF timer".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x96,
            "Relative OFF timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Maximum temperature level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Set temperature value".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Set temperature level by 15 steps".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Measured room temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measured floor temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measured floor temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Zone change setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Special operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Daily timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Daily timer setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Daily timer setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Rated power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Power consumption measurement method".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Bath heating status sensor (0x0016)
        let mut obj =
            self.create_standard_object("Bath heating status sensor".to_string(), 0x00, 0x16);
        obj.add_standard_property(
            0xB0,
            "Detection threshold level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Bath heating detection status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        self.add_object(obj);

        // Cold or hot water heat source equipment (0x027A)
        let mut obj = self.create_standard_object(
            "Cold or hot water heat source equipment".to_string(),
            0x02,
            0x7A,
        );
        obj.add_standard_property(
            0x90,
            "ON timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x91,
            "ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x92,
            "Relative ON timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x94,
            "OFF timer reservation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x95,
            "Time set by OFF timer".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0x96,
            "Relative OFF timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Cold water temperature setting 2 Maximum allowable setting level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Warm water temperature setting 2 Maximum allowable setting level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Operation mode setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Water temperature setting 1".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Water temperature setting 2".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "required_c".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measured temperature of outward water (Exit water Temperature)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Measured temperature of inward water (Entrance water Temperature)".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Special operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Daily timer setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Daily timer setting 1".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Daily timer setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Rated power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Power consumption measurement method".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Smart electric energy meter for sub-metering (0x028D)
        let mut obj = self.create_standard_object(
            "Smart electric energy meter for sub-metering".to_string(),
            0x02,
            0x8D,
        );
        obj.add_standard_property(
            0xD3,
            "Electric energy coefficient".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Unit for cumulative amount of electric energy (normal and reverse directions)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Number of effective digits for cumulative amounts of electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Electric current coefficient".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD9,
            "Voltage coefficient".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE0, "Day for which the historical data of measured cumulative amounts of electric energy is to be retrieved".to_string(), "".to_string(), 0, "required".to_string(), "required".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE1,
            "Measured cumulative amount of electric energy(normal direction)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Historical data of measured cumulative amounts of electric energy (normal direction)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Measured cumulative amount of electric energy (reverse direction)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Historical data of measured cumulative amounts of electric energy (reverse direction)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Measured instantaneous electric power".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Measured instantaneous currents".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Measured instantaneous voltages".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Cumulative amounts of electric energy measured at fixed time(normal direction)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Cumulative amounts of electric energy measured at fixed time(reverse direction)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Lighting system (0x02A3)
        let mut obj = self.create_standard_object("Lighting system".to_string(), 0x02, 0xA3);
        obj.add_standard_property(
            0xB0,
            "Light level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Scene control setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Number that can assign scene control setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Ventilation fan (0x0133)
        let mut obj = self.create_standard_object("Ventilation fan".to_string(), 0x01, 0x33);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Set value of ventilation air flow rate".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Ventilation Auto setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Electric lock (0x026F)
        let mut obj = self.create_standard_object("Electric lock".to_string(), 0x02, 0x6F);
        obj.add_standard_property(
            0xE0,
            "Lock setting1".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Lock setting 2".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Lock status of door guard".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Door open/close status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Occupant/ non-occupant status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Alarm status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Auto lock mode setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE7,
            "Battery level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        self.add_object(obj);

        // Gas meter (0x0282)
        let mut obj = self.create_standard_object("Gas meter".to_string(), 0x02, 0x82);
        obj.add_standard_property(
            0xE0,
            "Cumulative amount of gas consumption measurement value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Cumulative amounts of gas consumption measurement log".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Air conditioner ventilation fan (0x0134)
        let mut obj =
            self.create_standard_object("Air conditioner ventilation fan".to_string(), 0x01, 0x34);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Set value of ventilation air flow rate".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Ventilation mode automatic setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Ventilation method setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Ventilation mode setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Cooling / heating high-low setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Set value of room relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB9,
            "Measured value of electric current consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBA,
            "Measured value of room relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBE,
            "Measured value of outdoor air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xBF,
            "Ventilation auto setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Measured value of CO2 concentration".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Smoke (cigarette) detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Pollution detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Measured value of outdoor relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD0,
            "Measured value of return air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Measured value of return relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Measured value of charging air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measured value of charging relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Measured value of discharging air temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Measured value of discharging air relative humidity".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Heat exchanger operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Controller (0x05FF)
        let mut obj = self.create_standard_object("Controller".to_string(), 0x05, 0xFF);
        obj.add_standard_property(
            0xC0,
            "Controller ID".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Number of devices controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Index".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Device ID".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Device type".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC5,
            "Name".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Connection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Business code of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Business code of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Product code of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC8,
            "Product code of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Manufacture date of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC9,
            "Manufacture date of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Registerd information renewal date of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCA,
            "Registerd information renewal date of the device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Registerd information renewal version information of the device to be controlled"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCB,
            "Registerd information renewal version information of the device to be controlled"
                .to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCC,
            "Place to install device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCD,
            "Fault status of device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCE,
            "Set property map for device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xCF,
            "Get property map for device to be controlled".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Address of installation location".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Extended lighting system (0x02A4)
        let mut obj =
            self.create_standard_object("Extended lighting system".to_string(), 0x02, 0xA4);
        obj.add_standard_property(
            0xB0,
            "Light level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Scene control setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Number that can assign scene control setting.".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Power consumption rate list".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC3,
            "Power consumption when fully lighted".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC4,
            "Possible power savings".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC5,
            "Power consumption limit setting".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC6,
            "Automatic operation controlling setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC7,
            "Fading control change time setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Temperature sensor (0x0011)
        let mut obj = self.create_standard_object("Temperature sensor".to_string(), 0x00, 0x11);
        obj.add_standard_property(
            0xE0,
            "Measured temperature value".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Human detection sensor (0x0007)
        let mut obj = self.create_standard_object("Human detection sensor".to_string(), 0x00, 0x07);
        obj.add_standard_property(
            0xB0,
            "Detection threshold level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Human detection status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        self.add_object(obj);

        // Refrigerator (0x03B7)
        let mut obj = self.create_standard_object("Refrigerator".to_string(), 0x03, 0xB7);
        obj.add_standard_property(
            0xA0,
            "Quick freeze function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA1,
            "Quick refrigeration function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA4,
            "Icemaker setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA5,
            "Icemaker operation status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA6,
            "Icemaker tank status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA8,
            "Refrigerator compartment humidification function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xA9,
            "Vegetable compartment humidification function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xAD,
            "Deodorization function setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB0,
            "Door open/close status".to_string(),
            "".to_string(),
            0,
            "required_o".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB1,
            "Door open warning".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xB2,
            "Refrigerator compartment door status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB3,
            "Freezer compartment door status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB4,
            "Ice compartment door status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB5,
            "Vegetable compartment door status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xB6,
            "Multi-refrigerating mode compartment door status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD1,
            "Measured refrigerator compartment temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD2,
            "Measured freezer compartment temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD3,
            "Measured subzero-fresh compartment temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD4,
            "Measured vegetable compartment temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD5,
            "Measured multi-refrigerating mode compartment temperature".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD8,
            "Compressor rotation speed".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDA,
            "Measured electric current consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xDC,
            "Rated power consumption".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Maximum allowable temperature setting level".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE2,
            "Refrigerator compartment temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE3,
            "Freezer compartment temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE4,
            "Ice compartment temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE5,
            "Vegetable compartment temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE6,
            "Multi-refrigerating mode compartment temperature setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE9,
            "Refrigerator compartment temperature level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Freezer compartment temperature level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Ice compartment temperature level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEC,
            "Vegetable compartment temperature level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xED,
            "Multi-refrigerating mode compartment temperature level setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Air cleaner (0x0135)
        let mut obj = self.create_standard_object("Air cleaner".to_string(), 0x01, 0x35);
        obj.add_standard_property(
            0x80,
            "Operation status".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "required".to_string(),
            "required".to_string(),
        );
        obj.add_standard_property(
            0xA0,
            "Air flow rate setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC0,
            "Air pollution detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC1,
            "Smoke (cigarette) detection status".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xC2,
            "Optical catalyst operation setting".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "optional".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Filter change notice".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        self.add_object(obj);

        // Low-voltage smart electric energy meter (0x0288)
        let mut obj = self.create_standard_object(
            "Low-voltage smart electric energy meter".to_string(),
            0x02,
            0x88,
        );
        obj.add_standard_property(
            0xD3,
            "Coefficient".to_string(),
            "".to_string(),
            0,
            "optional".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xD7,
            "Number of effective digits for cumulative amounts of electric energy".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE0,
            "Measured cumulative amount of electric energy (normal direction)".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE1,
            "Unit for cumulative amounts of electric energy (normal and reverse directions)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE2, "Historical data of measured cumulative amounts of electric energy 1 (normal direction)".to_string(), "".to_string(), 0, "required".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE3,
            "Measured cumulative amount of electric energy (reverse direction)".to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xE4, "Historical data of measured cumulative amounts of electric energy 1 (reverse direction)".to_string(), "".to_string(), 0, "required_c".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xE5, "Day for which the historical data of measured cumulative amounts of electric energy is to be retrieved 1".to_string(), "".to_string(), 0, "required".to_string(), "required".to_string(), "optional".to_string());
        obj.add_standard_property(
            0xE7,
            "Measured instantaneous electric power".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xE8,
            "Measured instantaneous currents".to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEA,
            "Cumulative amounts of electric energy measured at fixed time (normal direction)"
                .to_string(),
            "".to_string(),
            0,
            "required".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(
            0xEB,
            "Cumulative amounts of electric energy measured at fixed time (reverse direction)"
                .to_string(),
            "".to_string(),
            0,
            "required_c".to_string(),
            "notApplicable".to_string(),
            "optional".to_string(),
        );
        obj.add_standard_property(0xEC, "Historical data of measured cumulative amounts of electric energy 2 (normal and reverse directions)".to_string(), "".to_string(), 0, "optional".to_string(), "notApplicable".to_string(), "optional".to_string());
        obj.add_standard_property(0xED, "Day for which the historical data of measured cumulative amounts of electric energy is to be retrieved 2".to_string(), "".to_string(), 0, "optional".to_string(), "optional".to_string(), "optional".to_string());
        self.add_object(obj);
    }
}
