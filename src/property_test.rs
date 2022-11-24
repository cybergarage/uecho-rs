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

#[cfg(test)]
mod tests {

    use crate::property::*;

    #[test]
    fn property_set_data() {
        let mut prop = Property::new();
        for n in 1..10 {
            let data = vec![0; n];
            prop.set_data(&data);
            assert_eq!(prop.size(), n);
        }
    }

    #[test]
    fn property_add_data() {
        let mut prop = Property::new();
        let mut total_data_size = 0;
        for n in 1..10 {
            let data = vec![0; n];
            prop.add_data(&data);
            total_data_size += n;
            assert_eq!(prop.size(), total_data_size);
        }
    }
}
