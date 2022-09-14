# Copyright (C) 2021 The uecho-rs Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.

SHELL := bash

# SOURCE_FILES= \
# 	../database_manufacturers.rs \
# 	../database_mra_objects.rs

SOURCE_FILES= \
	../database_manufacturers.rs

.PHONY: all ${SOURCE_FILES} clean

all: ${SOURCE_FILES} clean

clean:
	@rm -f *.csv *.xlsx

############################################################
# Manufactuer 
############################################################

MANUFACTURE_XLSX=list_code_e.xlsx
MANUFACTURE_CSV=list_code_e.csv
MANUFACTURE_LIST_URL=https://echonet.jp/wp/wp-content/uploads/pdf/General/Echonet/ManufacturerCode_en/${MANUFACTURE_XLSX}

../database_manufacturers.rs : ${MANUFACTURE_CSV}
	@./manufacturers.pl ${MANUFACTURE_CSV} > $@

${MANUFACTURE_XLSX}:
	@wget -q -O ${MANUFACTURE_XLSX} ${MANUFACTURE_LIST_URL}

${MANUFACTURE_CSV}: ${MANUFACTURE_XLSX}
	@-ssconvert $< $@

############################################################
# Machine-readable database from Echonet consortium
############################################################

MRA_JSON_DIR=MRA_en_V1.1.1

../database_mra_objects.rs :
	@./objects_mra.pl ${MRA_JSON_DIR} > $@
