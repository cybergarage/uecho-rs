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

use log::*;
use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

use crate::message::ResponseErrorMessage;
use crate::node_profile::*;
use crate::object::*;
use crate::property::{Property, PropertyCode};
use crate::protocol::{Message, ESV, TID, TID_MAX, TID_MIN};
use crate::request_handler::*;
use crate::super_object::*;
use crate::transport::{Manager, NotifytManager, PORT};
use crate::transport::{Observer, ObserverObject};

/// Node represents an ECHONET-Lite node which contains ECHONET-Lite objects such the profiles and the devices.
pub struct Node {
    transport_mgr: Manager,
    objects: Vec<Object>,
    last_tid: TID,
    post_sender: Sender<Message>,
    request_mgr: RequestManager,
    notify_mgr: NotifytManager,
}

impl Node {
    pub fn new() -> Arc<Mutex<Node>> {
        let (tx, _): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let node = Arc::new(Mutex::new(Node {
            transport_mgr: Manager::new(),
            objects: Vec::new(),
            last_tid: TID_MIN,
            post_sender: tx,
            request_mgr: RequestManager::new(),
            notify_mgr: NotifytManager::new(),
        }));
        node.lock().unwrap().init_objects();
        node.lock()
            .unwrap()
            .add_request_handler(Arc::new(Mutex::new(node.clone())));
        node
    }

    fn init_objects(&mut self) {
        self.init_node_profile_object();
    }

    pub fn init_node_profile_object(&mut self) {
        let mut prof = NodeProfile::new();
        self.add_object(prof);
    }

    pub fn add_object(&mut self, obj: Object) -> bool {
        self.objects.push(obj);
        self.update_node_profile();
        true
    }

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
    }

    pub fn find_object(&self, code: ObjectCode) -> Option<&Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&self.objects[n]);
            }
        }
        None
    }

    pub fn find_object_mut(&mut self, code: ObjectCode) -> Option<&mut Object> {
        for n in 0..self.objects.len() {
            if self.objects[n].code() == code {
                return Some(&mut self.objects[n]);
            }
        }
        None
    }

    fn set_object_properties_data(&mut self, code: PropertyCode, data: &[u8]) {
        for obj in self.objects.iter_mut() {
            obj.set_property_data(code, data);
        }
    }

    fn set_object_properties_byte(&mut self, code: PropertyCode, v: u8) {
        let data: &[u8] = &[v];
        self.set_object_properties_data(code, data);
    }

    pub fn node_profile_object(&mut self) -> Option<&mut Object> {
        self.find_object_mut(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn add_request_handler(&mut self, handler: RequestHandlerObject) -> bool {
        self.request_mgr.add_handler(handler.clone())
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        self.notify_mgr.add_observer(observer.clone())
    }

    pub fn send_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> bool {
        self.update_message_trandaction_id(msg);
        self.transport_mgr.send(to_addr, msg)
    }

    pub fn post_message(&mut self, to_addr: SocketAddr, msg: &mut Message) -> Receiver<Message> {
        self.update_message_trandaction_id(msg);
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        self.post_sender = tx;
        self.transport_mgr.send(to_addr, msg);
        rx
    }

    pub fn notify(&mut self, msg: &mut Message) -> bool {
        self.update_message_trandaction_id(msg);
        self.transport_mgr.notify(msg)
    }

    fn create_annouce_property_message(&self, obj: &Object, prop: &Property) -> Message {
        let mut msg = Message::new();
        msg.set_esv(ESV::Notification);
        msg.set_deoj(NODE_PROFILE_OBJECT_CODE);
        msg.set_seoj(obj.code());
        msg.add_property(prop.into());
        msg
    }

    pub fn annouce_property(&mut self, obj: &Object, prop: &Property) -> bool {
        let mut msg = self.create_annouce_property_message(obj, prop);
        self.notify(&mut msg)
    }

    pub fn annouce(&mut self) -> bool {
        let node_profile_obj = self.find_object(NODE_PROFILE_OBJECT_CODE);
        if node_profile_obj.is_none() {
            return false;
        }
        let node_profile_obj = node_profile_obj.unwrap();
        let instance_list_prop =
            node_profile_obj.find_property(NODE_PROFILE_CLASS_INSTANCE_LIST_NOTIFICATION);
        if instance_list_prop.is_none() {
            return false;
        }
        let instance_list_prop = instance_list_prop.unwrap();
        let mut msg = self.create_annouce_property_message(node_profile_obj, instance_list_prop);
        self.notify(&mut msg)
    }

    pub fn has_interface(&self, addr: IpAddr) -> bool {
        self.transport_mgr.has_interface(addr)
    }

    pub fn is_running(&self) -> bool {
        self.transport_mgr.is_running()
    }

    pub fn start(&mut self) -> bool {
        // Starts the transport manager

        if !self.transport_mgr.is_running() {
            if !self.transport_mgr.start() {
                return false;
            }
        }

        // Turns on child objects

        self.set_object_properties_byte(OBJECT_OPERATING_STATUS, OBJECT_OPERATING_STATUS_ON);

        // Sets registerd observers to the transport manager.

        for observer in self.notify_mgr.observers() {
            self.transport_mgr.add_observer(observer.clone());
        }

        if !self.annouce() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        // Turns off child objects

        self.set_object_properties_byte(OBJECT_OPERATING_STATUS, OBJECT_OPERATING_STATUS_OFF);

        // Stops the transport manager

        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }

    fn update_message_trandaction_id(&mut self, msg: &mut Message) {
        if msg.has_tid() {
            return;
        }
        msg.set_tid(self.next_tid());
    }

    fn next_tid(&mut self) -> TID {
        if TID_MAX <= self.last_tid {
            self.last_tid = TID_MIN;
        } else {
            self.last_tid += 1;
        }
        self.last_tid
    }

    fn is_last_message_response(&self, msg: &Message) -> bool {
        if msg.esv().is_request() {
            return false;
        }
        if msg.tid() != self.last_tid {
            return false;
        }
        true
    }

    fn send_post_reopnse(&mut self, msg: Message) {
        match self.post_sender.send(msg) {
            Ok(()) => {}
            Err(_err) => {
                // warn!("{}", err);
            }
        }
        let (tx, _): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        self.post_sender = tx;
    }

    fn update_node_profile(&mut self) {
        let mut obj_codes = Vec::new();
        for obj in self.objects() {
            obj_codes.push(obj.code());
        }
        let node_prof_obj = self.node_profile_object();
        if node_prof_obj.is_none() {
            return;
        }
        let node_prof_obj = node_prof_obj.unwrap();
        let mut node_prof = NodeProfile::from(node_prof_obj);
        node_prof.update(&obj_codes);
    }

    // fn is_local_message(&mut self, msg: &Message) -> bool {
    //     self.transport_mgr.has_interface(msg.from().ip())
    // }

    // fn has_only_node_profile_object(&self) -> bool {
    //     if 1 < self.objects.len() {
    //         return false;
    //     }
    //     true
    // }

    fn message_received(&mut self, req_msg: &Message) -> Option<Message> {
        // Handles only request messages.

        if req_msg.esv().is_response() {
            return None;
        }

        // FIXME: Ignores local message when the node has only the node profile object.
        // if self.is_local_message(&req_msg) && self.has_only_node_profile_object() {
        //     return None;
        // }

        // Part II ECHONET Lite Communication Middleware Specifications
        // 4.2.2 Basic Sequences for Object Control in General

        let dst_obj_code = req_msg.deoj();

        // (A) Processing when the controlled object does not exist

        let dst_obj = self.find_object(dst_obj_code);
        if dst_obj.is_none() {
            return None;
        }
        let dst_obj = dst_obj.unwrap();

        // (B) Processing when the controlled object exists, except when ESV = 0x60 to 0x63, 0x6E and 0x74

        if !req_msg.esv().is_request() {
            return None;
        }

        fn is_valid_request_message(
            dst_obj: &Object,
            req_esv: ESV,
            req_msg_props: &Vec<crate::protocol::Property>,
        ) -> bool {
            for req_msg_prop in req_msg_props.iter() {
                // (C) Processing when the controlled property exists but the controlled property doesn’t exist or can be processed only partially
                let dst_prop = dst_obj.find_property(req_msg_prop.code());
                if dst_prop.is_none() {
                    return false;
                }
                // (D) Processing when the controlled property exists but the stipulated service processing functions are not available
                let dst_prop = dst_prop.unwrap();
                match req_esv {
                    ESV::WriteRequest | ESV::WriteRequestResponseRequired => {
                        if !dst_prop.is_writable() {
                            return false;
                        }
                    }
                    ESV::ReadRequest
                    | ESV::NotificationRequest
                    | ESV::NotificationResponseRequired => {
                        if !dst_prop.is_readable() {
                            return false;
                        }
                    }
                    ESV::WriteReadRequest => {
                        if !dst_prop.is_writable() {
                            return false;
                        }
                        if !dst_prop.is_readable() {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
                // (E) Processing when the controlled property exists and the stipulated service processing functions are available but the EDT size does not match
                match req_esv {
                    ESV::WriteRequest
                    | ESV::WriteRequestResponseRequired
                    | ESV::WriteReadRequest => {
                        if 0 < dst_prop.capacity() && (dst_prop.capacity() < req_msg_prop.size()) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
            true
        }

        match req_msg.esv() {
            ESV::WriteReadRequest => {
                if !is_valid_request_message(dst_obj, ESV::WriteRequest, req_msg.properties_set()) {
                    return Some(ResponseErrorMessage::from(req_msg));
                }
                if !is_valid_request_message(dst_obj, ESV::ReadRequest, req_msg.properties_get()) {
                    return Some(ResponseErrorMessage::from(req_msg));
                }
            }
            _ => {
                if !is_valid_request_message(dst_obj, req_msg.esv(), req_msg.properties()) {
                    return Some(ResponseErrorMessage::from(req_msg));
                }
            }
        }

        // (F) Processing when the controlled property exists, the stipulated service processing functions are available and also the EDT size matches

        // Verifies whether the request message is allowed by the registered request handlers.

        let dst_obj = self.find_object_mut(dst_obj_code);
        if dst_obj.is_none() {
            return None;
        }
        let dst_obj = dst_obj.unwrap();

        // FIXME: rustc --explain E0499
        // if !self.request_mgr.property_request_received(dst_copy_obj, req_msg) {
        let mut dst_copy_obj = dst_obj.clone();
        if !self
            .request_mgr
            .property_request_received(&mut dst_copy_obj, req_msg)
        {
            return Some(ResponseErrorMessage::from(req_msg));
        }

        let dst_obj = self.find_object_mut(dst_obj_code);
        if dst_obj.is_none() {
            return None;
        }
        let dst_obj = dst_obj.unwrap();

        for dst_copy_prop in dst_copy_obj.properties() {
            let dst_prop = dst_obj.find_property_mut(dst_copy_prop.code());
            if dst_prop.is_none() {
                continue;
            }
            let dst_prop = dst_prop.unwrap();
            dst_prop.set_data(dst_copy_prop.data());
        }

        // Writes request property data to the property.

        let dst_obj = self.find_object_mut(dst_obj_code);
        if dst_obj.is_none() {
            return None;
        }
        let dst_obj = dst_obj.unwrap();

        match req_msg.esv() {
            ESV::WriteRequest | ESV::WriteRequestResponseRequired => {
                for req_msg_prop in req_msg.properties() {
                    let dst_prop = dst_obj.find_property_mut(req_msg_prop.code());
                    if dst_prop.is_none() {
                        continue;
                    }
                    let dst_prop = dst_prop.unwrap();
                    dst_prop.set_data(&req_msg_prop.data().clone());
                }
            }
            ESV::WriteReadRequest => {
                for req_msg_prop in req_msg.properties_set() {
                    let dst_prop = dst_obj.find_property_mut(req_msg_prop.code());
                    if dst_prop.is_none() {
                        continue;
                    }
                    let dst_prop = dst_prop.unwrap();
                    dst_prop.set_data(&req_msg_prop.data().clone());
                }
            }
            _ => {}
        }

        // Generates a response message for the valid request message and returns the response message.

        dst_obj.message_received(req_msg)
    }
}

impl Observer for Arc<Mutex<Node>> {
    fn message_received(&mut self, req_msg: &Message) {
        let mut node = self.lock().unwrap();
        if node.is_last_message_response(req_msg) {
            node.send_post_reopnse(req_msg.clone());
        }
        let res_msg = node.message_received(req_msg);
        if res_msg.is_some() {
            let mut res_msg = res_msg.unwrap();
            if res_msg.esv().is_response() {
                if res_msg.esv().is_unicast_response() {
                    node.send_message(SocketAddr::new(req_msg.from().ip(), PORT), &mut res_msg);
                } else {
                    node.notify(&mut res_msg);
                }
            } else {
                warn!("invalid ESV response ({}): {}", res_msg.esv(), res_msg)
            }
        }
    }
}

impl RequestHandler for Arc<Mutex<Node>> {
    fn property_request_received(
        &mut self,
        deoj: &mut Object,
        esv: ESV,
        prop: &crate::protocol::Property,
    ) -> bool {
        if deoj.code() != NODE_PROFILE_OBJECT_CODE {
            return false;
        }
        match esv {
            ESV::ReadRequest | ESV::NotificationRequest => match prop.code() {
                NODE_PROFILE_CLASS_OPERATING_STATUS => true,
                NODE_PROFILE_CLASS_VERSION_INFORMATION => true,
                NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_INSTANCES => true,
                NODE_PROFILE_CLASS_NUMBER_OF_SELF_NODE_CLASSES => true,
                NODE_PROFILE_CLASS_INSTANCE_LIST_NOTIFICATION => true,
                NODE_PROFILE_CLASS_SELF_NODE_INSTANCE_LIST_S => true,
                NODE_PROFILE_CLASS_SELF_NODE_CLASS_LIST_S => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.stop();
    }
}
