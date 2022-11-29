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

use crate::handler::*;
use crate::message::ResponseErrorMessage;
use crate::node_profile::*;
use crate::protocol::{Esv, Message, TID, TID_MAX, TID_MIN};
use crate::transport::Manager;
use crate::transport::*;
use crate::{object::*, Property};

/// LocalNode represents an ECHONET-Lite node in the controller and device nodes.
pub struct LocalNode {
    transport_mgr: Manager,
    objects: Vec<Object>,
    last_tid: TID,
    post_sender: Sender<Message>,
    request_mgr: RequestManager,
}

impl LocalNode {
    pub fn new() -> Arc<Mutex<LocalNode>> {
        let (tx, _): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let node = Arc::new(Mutex::new(LocalNode {
            transport_mgr: Manager::new(),
            objects: Vec::new(),
            last_tid: TID_MIN,
            post_sender: tx,
            request_mgr: RequestManager::new(),
        }));
        node.lock().unwrap().init();
        node
    }

    pub fn init(&mut self) {
        self.objects.push(NodeProfile::new());
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

    pub fn node_profile_object(&mut self) -> Option<&mut Object> {
        self.find_object_mut(NODE_PROFILE_OBJECT_CODE)
    }

    pub fn set_request_handler(&mut self, handler: RequestHandlerObject) {
        self.request_mgr.add_handler(handler.clone());
    }

    pub fn add_observer(&mut self, observer: ObserverObject) -> bool {
        self.transport_mgr.add_observer(observer.clone())
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
        msg.set_esv(Esv::Notification);
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

    pub fn start(&mut self) -> bool {
        if !self.transport_mgr.is_running() {
            if !self.transport_mgr.start() {
                return false;
            }
        }
        if !self.annouce() {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }

    fn update_message_trandaction_id(&mut self, msg: &mut Message) {
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

    fn message_received(&self, req_msg: &Message) -> Option<Message> {
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

        let mut are_all_properties_available = true;
        for req_msg_prop in req_msg.properties() {
            // (C) Processing when the controlled property exists but the controlled property doesn’t exist or can be processed only partially
            let dst_prop = dst_obj.find_property(req_msg_prop.code());
            if dst_prop.is_none() {
                are_all_properties_available = false;
                break;
            }
            // (D) Processing when the controlled property exists but the stipulated service processing functions are not available
            let dst_prop = dst_prop.unwrap();
            match req_msg.esv() {
                Esv::WriteRequest | Esv::WriteRequestResponseRequired => {
                    if !dst_prop.is_writable() {
                        are_all_properties_available = false;
                        break;
                    }
                }
                Esv::ReadRequest | Esv::NotificationRequest | Esv::NotificationResponseRequired => {
                    if !dst_prop.is_readable() {
                        are_all_properties_available = false;
                        break;
                    }
                }
                Esv::WriteReadRequest => {
                    if !dst_prop.is_writable() {
                        are_all_properties_available = false;
                        break;
                    }
                    if !dst_prop.is_readable() {
                        are_all_properties_available = false;
                        break;
                    }
                }
                _ => {
                    are_all_properties_available = false;
                    break;
                }
            }
            // (E) Processing when the controlled property exists and the stipulated service processing functions are available but the EDT size does not match
            match req_msg.esv() {
                Esv::WriteRequest | Esv::WriteRequestResponseRequired | Esv::WriteReadRequest => {
                    if 0 < dst_prop.capacity() && (dst_prop.capacity() < req_msg_prop.size()) {
                        are_all_properties_available = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if !are_all_properties_available {
            return Some(ResponseErrorMessage::from(req_msg));
        }

        // (F) Processing when the controlled property exists, the stipulated service processing functions are available and also the EDT size matches

        // Verifies whether the request message is allowed by the registered request handlers.

        if !self.request_mgr.property_request_received(req_msg) {
            return Some(ResponseErrorMessage::from(req_msg));
        }

        // Generates a response message for the valid request message and returns the response message.

        dst_obj.message_received(req_msg)
    }
}

impl Observer for Arc<Mutex<LocalNode>> {
    fn message_received(&mut self, req_msg: &Message) {
        let mut node = self.lock().unwrap();
        if node.is_last_message_response(req_msg) {
            node.send_post_reopnse(req_msg.clone());
        }
        info!("message_received {}", req_msg);
        let res_msg = node.message_received(req_msg);
        if res_msg.is_some() {
            let mut res_msg = res_msg.unwrap();
            if res_msg.esv().is_response() {
                if res_msg.esv().is_unicast_response() {
                    node.send_message(SocketAddr::new(req_msg.addr(), PORT), &mut res_msg);
                } else {
                    node.notify(&mut res_msg);
                }
            } else {
                warn!("invalid ESV response ({}): {}", res_msg.esv(), res_msg)
            }
        }
    }
}
