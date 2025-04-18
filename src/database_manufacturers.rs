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
        self.add_standard_manufacture(0x000133, String::from("afterFIT Co., Ltd."));
        self.add_standard_manufacture(0x00002F, String::from("AIPHONE CO., LTD."));
        self.add_standard_manufacture(0x00011A, String::from("ACCESS CO.,LTD."));
        self.add_standard_manufacture(0x00002C, String::from("AFT CO.,LTD"));
        self.add_standard_manufacture(0x000068, String::from("AISIN CORPORATION"));
        self.add_standard_manufacture(0x000079, String::from("Anritsu Engineering Co.,Ltd,"));
        self.add_standard_manufacture(0x00007F, String::from("ANRITSU CUSTOMER SUPPORT CO., LTD."));
        self.add_standard_manufacture(0x000110, String::from("ASUKA SOLUTION COMPANY LIMITED"));
        self.add_standard_manufacture(0x000096, String::from("Azbil Corporation"));
        self.add_standard_manufacture(0x0000FD, String::from("Bellnix Co.,LTD"));
        self.add_standard_manufacture(0x00006F, String::from("BUFFALO INC."));
        self.add_standard_manufacture(0x0000B6, String::from("Bunka Shutter Co., Ltd"));
        self.add_standard_manufacture(0x000017, String::from("Carrier Japan Corporation"));
        self.add_standard_manufacture(0x0000D2, String::from("CHOFUKOSAN.Co.Ltd"));
        self.add_standard_manufacture(0x000088, String::from("CHOFU SEISAKUSHO CO.,LTD."));
        self.add_standard_manufacture(0x0000D5, String::from("Choshu Industry Co., Ltd."));
        self.add_standard_manufacture(0x0000A3, String::from("Chubu Electric Power Grid Co.,Inc."));
        self.add_standard_manufacture(0x000136, String::from("Chuo Bussan Co.,Ltd."));
        self.add_standard_manufacture(0x0000FB, String::from("CICO CORPORATION"));
        self.add_standard_manufacture(0x0000F8, String::from("CIMX INITIATIVE INC."));
        self.add_standard_manufacture(0x000123, String::from("Contec Co., Ltd."));
        self.add_standard_manufacture(0x000130, String::from("COOLDESIGN Corporation"));
        self.add_standard_manufacture(0x000067, String::from("CORONA CORPORATION"));
        self.add_standard_manufacture(0x00013B, String::from("Crossdoor Inc."));
        self.add_standard_manufacture(0x00009C, String::from("Diamond Electric Mfg.Co.,Ltd."));
        self.add_standard_manufacture(
            0x000080,
            String::from("DIAMOND&ZEBRA ELECTRIC MFG.CO.,LTD."),
        );
        self.add_standard_manufacture(0x000119, String::from("DAIHEN Corporation"));
        self.add_standard_manufacture(0x000008, String::from("DAIKIN INDUSTRIES,LTD."));
        self.add_standard_manufacture(
            0x000015,
            String::from("Daikin Systems&Solutions Laboratory Ltd."),
        );
        self.add_standard_manufacture(0x00004F, String::from("Daiwa House Industry co.,Ltd"));
        self.add_standard_manufacture(0x000103, String::from("Data Technology Inc."));
        self.add_standard_manufacture(0x0000E3, String::from("DDL Co.,Ltd"));
        self.add_standard_manufacture(0x0000AD, String::from("Delta Electronics (Japan), Inc."));
        self.add_standard_manufacture(0x0000BE, String::from("DENKEN Co.,Ltd."));
        self.add_standard_manufacture(0x0000F4, String::from("DENSO AIRCOOL CORPORATION"));
        self.add_standard_manufacture(0x00003C, String::from("DENSO Corporation"));
        self.add_standard_manufacture(0x000109, String::from("DENSO Co.,LTD."));
        self.add_standard_manufacture(0x00012B, String::from("DENSO WAVE INCORPORATED"));
        self.add_standard_manufacture(0x000113, String::from("EBARA JITSUGYO CO., LTD"));
        self.add_standard_manufacture(0x000057, String::from("ELIIYPower Co.,ltd"));
        self.add_standard_manufacture(0x00012E, String::from("Eneres Co.,Ltd."));
        self.add_standard_manufacture(0x000041, String::from("ENEGATE CO.,LTD."));
        self.add_standard_manufacture(0x00010A, String::from("ENERGY GAP CORPORATION"));
        self.add_standard_manufacture(0x0000F3, String::from("Energy Gateway, Inc"));
        self.add_standard_manufacture(0x0000F2, String::from("Energy Solutions Inc."));
        self.add_standard_manufacture(0x000072, String::from("Eneres Co.,Ltd."));
        self.add_standard_manufacture(0x0000DD, String::from("EneStone Corporation"));
        self.add_standard_manufacture(0x000132, String::from("EX4Energy, Inc."));
        self.add_standard_manufacture(0x000055, String::from("FAMILYNET JAPAN CORPORATION"));
        self.add_standard_manufacture(0x0000F6, String::from("Field Logic Inc."));
        self.add_standard_manufacture(0x00012F, String::from("FORMOSA BIO AND ENERGY CORP JAPAN"));
        self.add_standard_manufacture(0x0000CB, String::from("Fuji Electric Co.,Ltd"));
        self.add_standard_manufacture(0x0000FC, String::from("FUJI INDUSTRIAL CO.,Ltd."));
        self.add_standard_manufacture(0x000051, String::from("Fuji IT Co.,Ltd."));
        self.add_standard_manufacture(0x00008A, String::from("FUJITSU GENERAL LIMITED"));
        self.add_standard_manufacture(0x00004E, String::from("FUJITSU LIMITED"));
        self.add_standard_manufacture(0x000090, String::from("Fujitsu Component Limited"));
        self.add_standard_manufacture(
            0x0000DE,
            String::from("FUJIFILM Business Innovation Japan Corp"),
        );
        self.add_standard_manufacture(
            0x000097,
            String::from("Future Technology Laboratories, Inc."),
        );
        self.add_standard_manufacture(0x00009B, String::from("GASTAR Co.,Ltd"));
        self.add_standard_manufacture(0x00008F, String::from("Glamo Inc."));
        self.add_standard_manufacture(0x000129, String::from("GoodWe Japan K.K"));
        self.add_standard_manufacture(0x000134, String::from("GoodWe Technologies Co.,Ltd."));
        self.add_standard_manufacture(0x00009F, String::from("GS Yuasa International Ltd"));
        self.add_standard_manufacture(0x00013A, String::from("GUGEN,Inc."));
        self.add_standard_manufacture(0x000122, String::from("Hanwha Q CELLS Japan CO.,LTD."));
        self.add_standard_manufacture(0x000001, String::from("Hitachi, Ltd."));
        self.add_standard_manufacture(
            0x000022,
            String::from("Hitachi Global Life Solutions, Inc."),
        );
        self.add_standard_manufacture(
            0x000040,
            String::from("Hitachi High-Tech Solutions Corporation"),
        );
        self.add_standard_manufacture(
            0x000044,
            String::from("Hitachi Industrial Equipment Systems Co.,Ltd."),
        );
        self.add_standard_manufacture(
            0x0000CC,
            String::from("Hitachi-Johnson Controls Air Conditioning,Inc."),
        );
        self.add_standard_manufacture(0x0000E5, String::from("Hitachi Power Solutions Co.,Ltd."));
        self.add_standard_manufacture(
            0x0000E6,
            String::from("Hokkaido Electrical Safety Services Foundation"),
        );
        self.add_standard_manufacture(0x0000B8, String::from("Hokkaido Electric Power Co.,Inc."));
        self.add_standard_manufacture(
            0x0000BB,
            String::from("Hokuriku Electric Power Transmission & Distribution Company"),
        );
        self.add_standard_manufacture(0x0000A1, String::from("Honda R&D Co., Ltd."));
        self.add_standard_manufacture(0x000115, String::from("HUAWEI TECHNOLOGIES JAPAN K.K."));
        self.add_standard_manufacture(0x0000AC, String::from("IDEC COROPRATION"));
        self.add_standard_manufacture(0x00013E, String::from("i GRID SOLUTIONS Inc."));
        self.add_standard_manufacture(0x00004D, String::from("INABA DENKI SANGYO CO.,LTD."));
        self.add_standard_manufacture(0x000056, String::from("iND Co.,Ltd"));
        self.add_standard_manufacture(0x0000ED, String::from("INFINI Co. LTD"));
        self.add_standard_manufacture(0x000124, String::from("INTEC Inc."));
        self.add_standard_manufacture(0x0000B1, String::from("Internet Initiative Japan Inc."));
        self.add_standard_manufacture(0x000087, String::from("I-O DATA DEVICE,INC."));
        self.add_standard_manufacture(0x00006B, String::from("ISB Corporation"));
        self.add_standard_manufacture(0x00010F, String::from("Iwatani Corporation"));
        self.add_standard_manufacture(0x000081, String::from("IWATSU ELECTRIC CO., LTD."));
        self.add_standard_manufacture(
            0x0000C3,
            String::from("Japan Electric Meters Inspection Corporation"),
        );
        self.add_standard_manufacture(0x0000F7, String::from("JCity,Inc."));
        self.add_standard_manufacture(0x0000CA, String::from("JSP CO.,LTD."));
        self.add_standard_manufacture(0x0000D7, String::from("Kaga Electronics co.,ltd."));
        self.add_standard_manufacture(0x000077, String::from("Kanagawa Institute of Technology"));
        self.add_standard_manufacture(0x000063, String::from("Kawamura Electric Inc."));
        self.add_standard_manufacture(0x0000F0, String::from("KANEKA CORPORATION"));
        self.add_standard_manufacture(0x00010B, String::from("KITANIHON ELECTRIC CABLE CO.,LTD."));
        self.add_standard_manufacture(0x0000E8, String::from("KOIZUMI LIGHTING TECHNOLOGY CORP."));
        self.add_standard_manufacture(0x00003B, String::from("KYOCERA Corporation"));
        self.add_standard_manufacture(0x00008C, String::from("Kyuden Technosystems Corporation"));
        self.add_standard_manufacture(
            0x0000BF,
            String::from("KYUSHU ELECTRIC POWER TRANSMISSION AND DISTRIBUTION CO.,INC."),
        );
        self.add_standard_manufacture(0x0000F1, String::from("Laplace Systems Co., Ltd."));
        self.add_standard_manufacture(0x000135, String::from("LinkJapan Inc."));
        self.add_standard_manufacture(0x000125, String::from("LiveSmart KK"));
        self.add_standard_manufacture(0x000025, String::from("LIXIL Corporation"));
        self.add_standard_manufacture(0x0000E0, String::from("Looop Inc"));
        self.add_standard_manufacture(0x00010C, String::from("MAX CO., LTD."));
        self.add_standard_manufacture(0x000078, String::from("Maxell, Ltd."));
        self.add_standard_manufacture(0x000058, String::from("Mediotec Corporation"));
        self.add_standard_manufacture(0x000120, String::from("Meisei electric co.,ltd."));
        self.add_standard_manufacture(0x000083, String::from("Melco Techno Yokohama Corporation"));
        self.add_standard_manufacture(0x000006, String::from("Mitsubishi Electric Corp."));
        self.add_standard_manufacture(
            0x000034,
            String::from("MITSUBISHI ELECTRIC ENGINEERING COMPANY LIMITED"),
        );
        self.add_standard_manufacture(
            0x000105,
            String::from("Mitsubishi Electric Lighting Corporation"),
        );
        self.add_standard_manufacture(0x00012A, String::from("Monochrome Inc."));
        self.add_standard_manufacture(0x00011D, String::from("mui Lab, Inc."));
        self.add_standard_manufacture(0x0000D4, String::from("Murata Manufacturing Co.,Ltd."));
        self.add_standard_manufacture(0x0000B0, String::from("NALTEC, Inc."));
        self.add_standard_manufacture(0x000106, String::from("Nature Inc."));
        self.add_standard_manufacture(0x000009, String::from("NEC Corp."));
        self.add_standard_manufacture(0x000118, String::from("NEC Magnus Communications, Ltd"));
        self.add_standard_manufacture(0x000073, String::from("NEC Platforms, Ltd."));
        self.add_standard_manufacture(0x000091, String::from("NEC Platforms, Ltd."));
        self.add_standard_manufacture(0x0000E2, String::from("NextDrive Inc."));
        self.add_standard_manufacture(0x000104, String::from("Next Energy & Resources Co., Ltd."));
        self.add_standard_manufacture(0x0000B2, String::from("NF Blossom Technologies, Inc."));
        self.add_standard_manufacture(0x0000A5, String::from("Nichibei Co., Ltd."));
        self.add_standard_manufacture(0x00006C, String::from("NICHICON CORPORATION"));
        self.add_standard_manufacture(0x0000EB, String::from("NICHICON (KAMEOKA) CORPORATION"));
        self.add_standard_manufacture(0x000102, String::from("NICHICON (KUSATSU) CORPORATION"));
        self.add_standard_manufacture(0x000112, String::from("NICHIEI INTEC CO., LTD"));
        self.add_standard_manufacture(0x000071, String::from("NIHON SANGYO CO.,LTD."));
        self.add_standard_manufacture(0x0000DC, String::from("NIHON TECHNO CO.,LTD."));
        self.add_standard_manufacture(0x00013D, String::from("NIPPON GAS CO.,LTD."));
        self.add_standard_manufacture(
            0x00008D,
            String::from("NIPPON TELEGRAPH AND TELEPHONE CORPORATION"),
        );
        self.add_standard_manufacture(
            0x000047,
            String::from("NIPPON TELEGRAPH AND TELEPHONE EAST CORPORATION"),
        );
        self.add_standard_manufacture(
            0x000086,
            String::from("NIPPON TELEGRAPH AND TELEPHONE WEST CORPORATION"),
        );
        self.add_standard_manufacture(0x000036, String::from("NISSIN SYSTEMS CO., LTD."));
        self.add_standard_manufacture(0x0000B7, String::from("Nitto Kogyo Corporation"));
        self.add_standard_manufacture(0x000054, String::from("NORITZ CORP."));
        self.add_standard_manufacture(0x00007C, String::from("NSW Inc."));
        self.add_standard_manufacture(
            0x0000A0,
            String::from("NTT Advanced Technology Corporation"),
        );
        self.add_standard_manufacture(0x000023, String::from("NTT COMWARE CORPORATION"));
        self.add_standard_manufacture(0x000061, String::from("NTT DATA INTELLILINK CORPORATION"));
        self.add_standard_manufacture(0x0000E9, String::from("NTT Smile Energy Inc."));
        self.add_standard_manufacture(0x0000F5, String::from("ODELIC CO., LTD."));
        self.add_standard_manufacture(0x000012, String::from("Oi Electric Co., Ltd"));
        self.add_standard_manufacture(0x00006A, String::from("OKAYA & CO., LTD."));
        self.add_standard_manufacture(0x000114, String::from("OkayaKiden Co.,Ltd."));
        self.add_standard_manufacture(0x000137, String::from("OkayaKiden Co.,Ltd."));
        self.add_standard_manufacture(0x000048, String::from("Oki Electric Industry Co., Ltd."));
        self.add_standard_manufacture(0x000064, String::from("OMRON SOCIAL SOLUTIONS CO.,LTD."));
        self.add_standard_manufacture(0x00012C, String::from("Onamba Co., Ltd."));
        self.add_standard_manufacture(0x0000D8, String::from("OSAKI DATATECH CO.,LTD."));
        self.add_standard_manufacture(0x000052, String::from("OSAKI ELECTRIC CO.,LTD."));
        self.add_standard_manufacture(0x000127, String::from("Paloma Co.,Ltd."));
        self.add_standard_manufacture(0x00000B, String::from("Panasonic Holdings Corporation"));
        self.add_standard_manufacture(
            0x0000DA,
            String::from("Panasonic Commercial Equipment Systems Co.,Ltd."),
        );
        self.add_standard_manufacture(0x0000FE, String::from("Panasonic Ecology Systems Co.,Ltd."));
        self.add_standard_manufacture(0x0000FA, String::from("Plat'Home Co."));
        self.add_standard_manufacture(0x000082, String::from("PURPOSE CO.,LTD."));
        self.add_standard_manufacture(0x000139, String::from("RATOC Systems, Inc."));
        self.add_standard_manufacture(0x000059, String::from("Rinnai Corporation"));
        self.add_standard_manufacture(0x000128, String::from("SAIKOH ENGINEERING Co.,Ltd."));
        self.add_standard_manufacture(0x00011E, String::from("SAKAIGAWA CO., LTD"));
        self.add_standard_manufacture(0x00011C, String::from("SANDEN RETAIL SYSTEMS CORPORATION"));
        self.add_standard_manufacture(0x00010E, String::from("SANIX INCORPORATED"));
        self.add_standard_manufacture(0x0000BA, String::from("SankyoTateyama, Inc."));
        self.add_standard_manufacture(0x0000C5, String::from("SANWA SHUTTER CORPORATION"));
        self.add_standard_manufacture(0x000093, String::from("SATORI ELECTRIC CO.,LTD."));
        self.add_standard_manufacture(0x000107, String::from("SEIKO ELECTRIC CO.,LTD."));
        self.add_standard_manufacture(0x00003A, String::from("SEKISUI HOUSE, LTD."));
        self.add_standard_manufacture(0x000005, String::from("Sharp Corp"));
        self.add_standard_manufacture(
            0x000131,
            String::from("Shenzhen Eternalplanet Energy Pingshan Ltd."),
        );
        self.add_standard_manufacture(0x0000AE, String::from("SHIKOKU ELECTRIC POWER CO.,INC."));
        self.add_standard_manufacture(0x00002E, String::from("SHIKOKU INSTRUMENTATION CO.,LTD"));
        self.add_standard_manufacture(
            0x0000CE,
            String::from("SHINDENGEN ELECTRIC MANUFACTURING CO.LTD."),
        );
        self.add_standard_manufacture(0x00010D, String::from("Shizen Energy Inc."));
        self.add_standard_manufacture(0x0000A8, String::from("Smart Power System. Co,. Ltd."));
        self.add_standard_manufacture(0x0000DF, String::from("SMA Japan K.K."));
        self.add_standard_manufacture(0x000100, String::from("Smart Solar Corporation"));
        self.add_standard_manufacture(0x00007E, String::from("SMK Corporation"));
        self.add_standard_manufacture(0x0000E1, String::from("SoftBank Corp."));
        self.add_standard_manufacture(
            0x00011B,
            String::from("SolaX Power Network Technology (Zhe jiang) Co. , Ltd."),
        );
        self.add_standard_manufacture(
            0x000060,
            String::from("Sony Computer Science Laboratories, Inc."),
        );
        self.add_standard_manufacture(0x00006E, String::from("Soundvision co.,ltd."));
        self.add_standard_manufacture(0x000108, String::from("SOUSEI Technology Inc."));
        self.add_standard_manufacture(0x000116, String::from("Sungrow Power Supply Co., Ltd."));
        self.add_standard_manufacture(0x000101, String::from("Sunpot Co., Ltd"));
        self.add_standard_manufacture(0x0000DB, String::from("Suntech Power Japan Corporation"));
        self.add_standard_manufacture(0x00003D, String::from("SUMITOMO ELECTRIC INDUSTRIES, LTD."));
        self.add_standard_manufacture(0x00003E, String::from("SUMITOMO ELECTRIC NETWORKS, INC."));
        self.add_standard_manufacture(0x00012D, String::from("TACHIKAWA CORPORATION"));
        self.add_standard_manufacture(0x000085, String::from("TAKAOKA TOKO CO.,LTD"));
        self.add_standard_manufacture(0x0000AF, String::from("Takara Standard Co.,Ltd"));
        self.add_standard_manufacture(0x0000E4, String::from("technoeye Inc."));
        self.add_standard_manufacture(0x0000FF, String::from("TEPCO Energy Partner, Inc."));
        self.add_standard_manufacture(0x0000EE, String::from("TESSERA TECHNOLOGY INC."));
        self.add_standard_manufacture(
            0x0000B5,
            String::from("The Chugoku Electric Power Co., Ltd"),
        );
        self.add_standard_manufacture(
            0x00009A,
            String::from("The Kansai Electric Power Co., Inc."),
        );
        self.add_standard_manufacture(0x0000CD, String::from("TOCLAS CORPORATION"));
        self.add_standard_manufacture(0x000126, String::from("Togami Electric Mfg.co.,Ltd."));
        self.add_standard_manufacture(0x0000F9, String::from("TOHO ELECTRONICS INC."));
        self.add_standard_manufacture(
            0x0000C2,
            String::from("Tohoku Electric Meter Industry Co.,Inc"),
        );
        self.add_standard_manufacture(
            0x0000BC,
            String::from("TohokuElectric Power Network Company,Incorporated"),
        );
        self.add_standard_manufacture(
            0x000099,
            String::from("Tokyo Electric Power Company Holdings, Inc."),
        );
        self.add_standard_manufacture(0x0000B3, String::from("TOPPERS Project, Inc."));
        self.add_standard_manufacture(0x000111, String::from("Topre Corporation"));
        self.add_standard_manufacture(0x000016, String::from("Toshiba Corp."));
        self.add_standard_manufacture(
            0x000043,
            String::from("TOSHIBA DEVELOPMENT & ENGINEERING CORPORATION"),
        );
        self.add_standard_manufacture(
            0x0000EC,
            String::from("Toshiba Energy Systems & Solutions Corporation"),
        );
        self.add_standard_manufacture(
            0x0000D9,
            String::from("Toshiba IT & Control Systems Corporation"),
        );
        self.add_standard_manufacture(
            0x000069,
            String::from("Toshiba Lifestyle Products & Services Corporation"),
        );
        self.add_standard_manufacture(
            0x00001B,
            String::from("TOSHIBA LIGHTING & TECHNOLOGY CORPORATION"),
        );
        self.add_standard_manufacture(
            0x000035,
            String::from("Toshiba Toko Meter Systems Co.,Ltd."),
        );
        self.add_standard_manufacture(0x000050, String::from("TOTO LTD."));
        self.add_standard_manufacture(0x0000EF, String::from("TOYOTA INDUSTRIES CORPORATION"));
        self.add_standard_manufacture(0x000121, String::from("TOYOTA MOTOR CORPORATION"));
        self.add_standard_manufacture(0x00011F, String::from("TOYOTA TSUSHO CORPORATION"));
        self.add_standard_manufacture(0x00005C, String::from("Tranceboot Co.,Ltd."));
        self.add_standard_manufacture(0x000138, String::from("TRENDE Inc."));
        self.add_standard_manufacture(0x000076, String::from("TSP CO.,Ltd"));
        self.add_standard_manufacture(0x0000D0, String::from("TSUBAKIMOTO CHAIN CO."));
        self.add_standard_manufacture(0x0000C1, String::from("Tsuken Electric Ind Co., Ltd."));
        self.add_standard_manufacture(0x000053, String::from("Ubiquitous AI Corporation"));
        self.add_standard_manufacture(0x000117, String::from("WWB Corporation"));
        self.add_standard_manufacture(0x000095, String::from("Yamato Denki Co.,Ltd."));
        self.add_standard_manufacture(0x00009E, String::from("YASKAWA ELECTRIC CORPORATION"));
        self.add_standard_manufacture(0x00007A, String::from("ZUKEN ELMIC,INC."));
        self.add_standard_manufacture(0x0000B4, String::from("4R Energy Corporation"));
        self.add_standard_manufacture(0xFFFFFF, String::from("Experimental"));
        self.add_standard_manufacture(0xFFFFFE, String::from("Undefined"));
    }
}
